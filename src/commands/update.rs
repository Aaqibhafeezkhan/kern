use std::env;
use std::fs;
use std::io::Read;
use std::process::ExitCode;

use semver::Version;
use serde::Deserialize;

/// GitHub repository in the form "owner/repo". Fill this in.
const GITHUB_REPO: &str = "dominikgobel/kern";

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

type BoxError = Box<dyn std::error::Error>;

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

/// Entry point for `kern update` and `kern update --check`.
pub fn run_update(check: bool) -> ExitCode {
    match update(check) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn update(check: bool) -> Result<(), BoxError> {
    let current = Version::parse(CURRENT_VERSION)?;

    println!("Checking for updates...");
    let release = fetch_latest_release()?;
    let latest = parse_tag(&release.tag_name)?;

    if latest <= current {
        println!("You are up to date (kern {current}).");
        return Ok(());
    }

    if check {
        println!("A newer version is available: {latest} (current: {current}).");
        println!("Run `kern update` to install it.");
        return Ok(());
    }

    println!("Updating kern {current} -> {latest}...");

    let wanted = asset_name()?;
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == wanted)
        .ok_or_else(|| format!("release {} has no asset named `{wanted}`", release.tag_name))?;

    println!("Downloading {}...", asset.name);
    let binary = download(&asset.browser_download_url)?; // let bytes ...

    // let binary = if wanted.ends_with(".tar.gz") {
    //     extract_binary(&bytes)?
    // } else {
    //     bytes
    // };

    install(&binary)?;
    println!("Successfully updated to kern {latest}.");
    Ok(())
}

fn parse_tag(tag: &str) -> Result<Version, BoxError> {
    Ok(Version::parse(tag.trim_start_matches('v'))?)
}

/// Returns the release asset name for the current platform.
fn asset_name() -> Result<&'static str, BoxError> {
    match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => Ok("kern-linux-x86_64"),
        ("linux", "aarch64") => Ok("kern-linux-aarch64"),
        ("macos", "x86_64") => Ok("kern-macos-x86_64"),
        ("macos", "aarch64") => Ok("kern-macos-aarch64"),
        ("windows", "x86_64") => Ok("kern-windows-x86_64.exe"),
        (os, arch) => Err(format!("unsupported platform: {os}/{arch}").into()),
    }
}

fn user_agent() -> String {
    format!("kern/{CURRENT_VERSION}")
}

/// Debug builds: allow overriding the API base URL (used by the integration tests).
#[cfg(debug_assertions)]
fn api_base() -> String {
    env::var("KERN_UPDATE_API_BASE").unwrap_or_else(|_| "https://api.github.com".to_string())
}

/// Release builds: always talk to GitHub, the override is compiled out.
#[cfg(not(debug_assertions))]
fn api_base() -> String {
    "https://api.github.com".to_string()
}

fn fetch_latest_release() -> Result<Release, BoxError> {
    let url = format!("{}/repos/{GITHUB_REPO}/releases/latest", api_base());
    let response = ureq::get(&url)
        .set("User-Agent", &user_agent())
        .set("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| format!("failed to query GitHub releases: {e}"))?;
    Ok(response.into_json::<Release>()?)
}

fn download(url: &str) -> Result<Vec<u8>, BoxError> {
    let response = ureq::get(url)
        .set("User-Agent", &user_agent())
        .call()
        .map_err(|e| format!("download failed: {e}"))?;
    let mut bytes = Vec::new();
    response.into_reader().read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// Replaces the currently running executable with the given binary.
/// Works on Windows too (a running .exe cannot simply be overwritten there).
fn install(binary: &[u8]) -> Result<(), BoxError> {
    let tmp_path = env::temp_dir().join(format!("kern-update-{}.tmp", std::process::id()));
    fs::write(&tmp_path, binary)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&tmp_path, fs::Permissions::from_mode(0o755))?;
    }

    let result = self_replace::self_replace(&tmp_path);
    let _ = fs::remove_file(&tmp_path);

    result.map_err(|e| {
        format!("failed to replace the current executable (do you need elevated permissions?): {e}")
            .into()
    })
}
