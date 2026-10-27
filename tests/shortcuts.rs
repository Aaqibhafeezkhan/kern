use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use kern::shortcuts::{Action, resolve_key};
use kern::ui_types::EditorMode;

#[test]
fn inserts_plain_characters() {
    let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);

    assert_eq!(resolve_key(key, EditorMode::Writing), Action::Insert('a'));
}

#[test]
fn inserts_alt_modified_characters() {
    // e.g. AltGr+8 -> '[' on a German layout, reported with the ALT modifier.
    let key = KeyEvent::new(KeyCode::Char('['), KeyModifiers::ALT);

    assert_eq!(resolve_key(key, EditorMode::Writing), Action::Insert('['));
}

#[test]
fn inserts_altgr_characters() {
    // AltGr is reported as CONTROL | ALT on many terminals.
    let key = KeyEvent::new(
        KeyCode::Char('['),
        KeyModifiers::ALT | KeyModifiers::CONTROL,
    );

    assert_eq!(resolve_key(key, EditorMode::Writing), Action::Insert('['));
}

#[test]
fn ignores_bare_control_characters() {
    let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);

    assert_eq!(resolve_key(key, EditorMode::Writing), Action::Ignore);
}

#[test]
fn keeps_control_shortcuts_working() {
    let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL);

    assert_eq!(resolve_key(key, EditorMode::Writing), Action::Quit);
}

#[test]
fn alt_does_not_trigger_navigation_shortcuts() {
    let key = KeyEvent::new(KeyCode::Left, KeyModifiers::ALT);

    assert_eq!(resolve_key(key, EditorMode::Writing), Action::Ignore);
}

#[test]
fn inserts_tab_without_modifiers() {
    let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);

    assert_eq!(resolve_key(key, EditorMode::Writing), Action::Insert('\t'));
}

#[test]
fn does_not_insert_tab_with_alt() {
    let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::ALT);

    assert_eq!(resolve_key(key, EditorMode::Writing), Action::Ignore);
}
