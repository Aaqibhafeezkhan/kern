//! Integration tests for `kern update` / `kern update --check`.
//!
//! Each test copies the compiled `kern` binary into a temp directory, runs the
//! copy against a local mock "GitHub API" server and inspects the result.
//! Requires a debug build (the `KERN_UPDATE_API_BASE` override is debug-only).

use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::{Arc, Mutex};
use std::thread;

use tempfile::TempDir;

const NEWER_TAG: &str = "v999.0.0";
const OLDER_TAG: &str = "v0.0.1";
const FAKE_BINARY: &[u8] = b"FAKE-KERN-BINARY-999";

// ---------------------------------------------------------------------------
// Mock server
// ---------------------------------------------------------------------------

struct Route {
    suffix: String,
    status: u16,
    body: Vec<u8>,
}

struct MockServer {
    base: String,
    requests: Arc<Mutex<Vec<String>>>,
}

impl MockServer {
    /// Starts the server. `build_routes` receives the base URL so that the
    /// release JSON can point its download URLs back at this server.
    fn start(build_routes: impl FnOnce(&str) -> Vec<Route>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let routes = build_routes(&base);
        let requests = Arc::new(Mutex::new(Vec::new()));
        let log = Arc::clone(&requests);

        thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { continue };
                let Ok(clone) = stream.try_clone() else {
                    continue;
                };
                let mut reader = BufReader::new(clone);

                let mut request_line = String::new();
                if reader.read_line(&mut request_line).is_err() {
                    continue;
                }
                // Drain the headers.
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(0) | Err(_) => break,
                        Ok(_) if line == "\r\n" || line == "\n" => break,
                        Ok(_) => {}
                    }
                }

                let path = request_line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("/")
                    .to_string();
                log.lock().unwrap().push(path.clone());

                let (status, body) = routes
                    .iter()
                    .find(|r| path.ends_with(&r.suffix))
                    .map(|r| (r.status, r.body.clone()))
                    .unwrap_or((404, b"not found".to_vec()));

                let header = format!(
                    "HTTP/1.1 {status} X\r\nContent-Length: {}\r\n\
                     Content-Type: application/octet-stream\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(&body);
                let _ = stream.flush();
            }
        });

        MockServer { base, requests }
    }

    fn download_count(&self) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.contains("/download/"))
            .count()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Mirrors the asset selection in `update.rs`. `None` = unsupported platform.
fn platform_asset() -> Option<&'static str> {
    match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => Some("kern-linux-x86_64"),
        ("linux", "aarch64") => Some("kern-linux-aarch64"),
        ("macos", "x86_64") => Some("kern-macos-x86_64"),
        ("macos", "aarch64") => Some("kern-macos-aarch64"),
        ("windows", "x86_64") => Some("kern-windows-x86_64.exe"),
        _ => None,
    }
}

/// What the server delivers for the platform asset: a raw .exe on Windows,
/// a .tar.gz containing `kern` everywhere else.
fn asset_body(binary: &[u8]) -> Vec<u8> {
    binary.to_vec()
}

fn release_route(base: &str, tag: &str, assets: &[&str]) -> Route {
    let assets: Vec<_> = assets
        .iter()
        .map(|name| {
            serde_json::json!({
                "name": name,
                "browser_download_url": format!("{base}/download/{name}"),
            })
        })
        .collect();
    Route {
        suffix: "/releases/latest".into(),
        status: 200,
        body: serde_json::to_vec(&serde_json::json!({ "tag_name": tag, "assets": assets }))
            .unwrap(),
    }
}

fn asset_route(name: &str, body: Vec<u8>) -> Route {
    Route {
        suffix: format!("/download/{name}"),
        status: 200,
        body,
    }
}

/// Copies the compiled binary into a temp dir so it can safely replace itself.
fn install_copy() -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let exe = dir.path().join(format!("kern{}", env::consts::EXE_SUFFIX));
    fs::copy(env!("CARGO_BIN_EXE_kern"), &exe).unwrap();
    (dir, exe)
}

fn run(exe: &Path, args: &[&str], server: &MockServer) -> Output {
    Command::new(exe)
        .args(args)
        .env("KERN_UPDATE_API_BASE", &server.base)
        .output()
        .expect("failed to run kern")
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

// ---------------------------------------------------------------------------
// --check
// ---------------------------------------------------------------------------

#[test]
fn check_reports_newer_version_without_downloading() {
    let Some(asset) = platform_asset() else {
        return;
    };
    let server = MockServer::start(|base| {
        vec![
            release_route(base, NEWER_TAG, &[asset]),
            asset_route(asset, asset_body(FAKE_BINARY)),
        ]
    });
    let (_dir, exe) = install_copy();
    let before = fs::read(&exe).unwrap();

    let out = run(&exe, &["update", "--check"], &server);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    assert!(text.contains("newer version"), "stdout: {text}");
    assert!(text.contains("kern update"), "stdout: {text}");
    assert_eq!(server.download_count(), 0);
    assert_eq!(fs::read(&exe).unwrap(), before);
}

#[test]
fn check_reports_up_to_date() {
    let Some(asset) = platform_asset() else {
        return;
    };
    let server = MockServer::start(|base| vec![release_route(base, OLDER_TAG, &[asset])]);
    let (_dir, exe) = install_copy();

    let out = run(&exe, &["update", "--check"], &server);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        stdout(&out).contains("up to date"),
        "stdout: {}",
        stdout(&out)
    );
    assert_eq!(server.download_count(), 0);
}

#[test]
fn check_accepts_tag_without_v_prefix() {
    let Some(asset) = platform_asset() else {
        return;
    };
    let server = MockServer::start(|base| vec![release_route(base, "999.0.0", &[asset])]);
    let (_dir, exe) = install_copy();

    let out = run(&exe, &["update", "--check"], &server);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(stdout(&out).contains("newer version"));
}

// ---------------------------------------------------------------------------
// Download + install
// ---------------------------------------------------------------------------

#[test]
fn update_downloads_and_replaces_binary() {
    let Some(asset) = platform_asset() else {
        return;
    };
    let server = MockServer::start(|base| {
        vec![
            release_route(base, NEWER_TAG, &[asset]),
            asset_route(asset, asset_body(FAKE_BINARY)),
        ]
    });
    let (_dir, exe) = install_copy();

    let out = run(&exe, &["update"], &server);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        stdout(&out).contains("Successfully updated"),
        "stdout: {}",
        stdout(&out)
    );
    assert_eq!(server.download_count(), 1);
    assert_eq!(fs::read(&exe).unwrap(), FAKE_BINARY);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(&exe).unwrap().permissions().mode();
        assert!(mode & 0o111 != 0, "installed binary is not executable");
    }
}

#[test]
fn update_does_nothing_when_already_up_to_date() {
    let Some(asset) = platform_asset() else {
        return;
    };
    let server = MockServer::start(|base| {
        vec![
            release_route(base, OLDER_TAG, &[asset]),
            asset_route(asset, asset_body(FAKE_BINARY)),
        ]
    });
    let (_dir, exe) = install_copy();
    let before = fs::read(&exe).unwrap();

    let out = run(&exe, &["update"], &server);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(stdout(&out).contains("up to date"));
    assert_eq!(server.download_count(), 0);
    assert_eq!(fs::read(&exe).unwrap(), before);
}

// ---------------------------------------------------------------------------
// Failure cases: the existing binary must stay untouched
// ---------------------------------------------------------------------------

#[test]
fn update_fails_when_release_has_no_asset_for_this_platform() {
    if platform_asset().is_none() {
        return;
    }
    let server = MockServer::start(|base| {
        vec![release_route(
            base,
            NEWER_TAG,
            &["kern-some-other-platform.tar.gz"],
        )]
    });
    let (_dir, exe) = install_copy();
    let before = fs::read(&exe).unwrap();

    let out = run(&exe, &["update"], &server);

    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("no asset"),
        "stderr: {}",
        stderr(&out)
    );
    assert_eq!(fs::read(&exe).unwrap(), before);
}

#[test]
fn update_fails_when_download_returns_404() {
    let Some(asset) = platform_asset() else {
        return;
    };
    // Release lists the asset, but the server has no file for it.
    let server = MockServer::start(|base| vec![release_route(base, NEWER_TAG, &[asset])]);
    let (_dir, exe) = install_copy();
    let before = fs::read(&exe).unwrap();

    let out = run(&exe, &["update"], &server);

    assert!(!out.status.success());
    assert!(
        stderr(&out).contains("download failed"),
        "stderr: {}",
        stderr(&out)
    );
    assert_eq!(fs::read(&exe).unwrap(), before);
}

#[test]
fn check_and_update_fail_when_api_returns_error() {
    let server = MockServer::start(|_| {
        vec![Route {
            suffix: "/releases/latest".into(),
            status: 500,
            body: b"boom".to_vec(),
        }]
    });
    let (_dir, exe) = install_copy();
    let before = fs::read(&exe).unwrap();

    for args in [&["update", "--check"][..], &["update"][..]] {
        let out = run(&exe, args, &server);
        assert!(!out.status.success(), "args: {args:?}");
        assert!(
            stderr(&out).contains("failed to query GitHub releases"),
            "stderr: {}",
            stderr(&out)
        );
    }
    assert_eq!(fs::read(&exe).unwrap(), before);
}
