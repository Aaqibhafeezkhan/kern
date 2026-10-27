use crate::ui_types::EditorMode;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Semantic actions keep key bindings separate from editing and rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    BeginSave,
    CancelSave,
    Save,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Backspace,
    Enter,
    Insert(char),
    Ignore,
}

#[derive(Debug, Clone, Copy)]
enum KeyBinding {
    Control(char),
    Key(KeyCode),
}

#[derive(Debug, Clone, Copy)]
struct Shortcut {
    binding: KeyBinding,
    action: Action,
    hint_key: &'static str,
    hint_label: &'static str,
}

const WRITING_SHORTCUTS: &[Shortcut] = &[
    Shortcut {
        binding: KeyBinding::Control('x'),
        action: Action::Quit,
        hint_key: "^X",
        hint_label: "Quit",
    },
    Shortcut {
        binding: KeyBinding::Control('o'),
        action: Action::BeginSave,
        hint_key: "^O",
        hint_label: "Save As",
    },
    Shortcut {
        binding: KeyBinding::Key(KeyCode::Left),
        action: Action::MoveLeft,
        hint_key: "←",
        hint_label: "Left",
    },
    Shortcut {
        binding: KeyBinding::Key(KeyCode::Right),
        action: Action::MoveRight,
        hint_key: "→",
        hint_label: "Right",
    },
    Shortcut {
        binding: KeyBinding::Key(KeyCode::Up),
        action: Action::MoveUp,
        hint_key: "↑",
        hint_label: "Up",
    },
    Shortcut {
        binding: KeyBinding::Key(KeyCode::Down),
        action: Action::MoveDown,
        hint_key: "↓",
        hint_label: "Down",
    },
    Shortcut {
        binding: KeyBinding::Key(KeyCode::Backspace),
        action: Action::Backspace,
        hint_key: "Backspace",
        hint_label: "Delete",
    },
    Shortcut {
        binding: KeyBinding::Key(KeyCode::Enter),
        action: Action::Enter,
        hint_key: "Enter",
        hint_label: "New line",
    },
];

const SAVING_SHORTCUTS: &[Shortcut] = &[
    Shortcut {
        binding: KeyBinding::Key(KeyCode::Enter),
        action: Action::Save,
        hint_key: "Enter",
        hint_label: "Save",
    },
    Shortcut {
        binding: KeyBinding::Control('c'),
        action: Action::CancelSave,
        hint_key: "^C",
        hint_label: "Cancel",
    },
    Shortcut {
        binding: KeyBinding::Key(KeyCode::Backspace),
        action: Action::Backspace,
        hint_key: "Backspace",
        hint_label: "Delete",
    },
];

fn shortcuts_for(mode: EditorMode) -> &'static [Shortcut] {
    match mode {
        EditorMode::Writing => WRITING_SHORTCUTS,
        EditorMode::Saving => SAVING_SHORTCUTS,
    }
}

pub fn resolve_key(key: KeyEvent, mode: EditorMode) -> Action {
    for shortcut in shortcuts_for(mode) {
        let matches = match shortcut.binding {
            KeyBinding::Control(ch) => {
                key.modifiers.contains(KeyModifiers::CONTROL)
                    && matches!(key.code, KeyCode::Char(c) if c.eq_ignore_ascii_case(&ch))
            }
            KeyBinding::Key(code) => {
                !key.modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
                    && key.code == code
            }
        };
        if matches {
            return shortcut.action;
        }
    }

    // Printable input is the default once no shortcut matched. A bare Ctrl
    // modifier stays reserved for shortcuts, but characters produced with Alt
    // or AltGr (reported as Ctrl+Alt on many layouts, e.g. AltGr+8 -> '[' on a
    // German keyboard) must still reach the editor.
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let alt = key.modifiers.contains(KeyModifiers::ALT);
    match key.code {
        KeyCode::Char(ch) if alt || !ctrl => Action::Insert(ch),
        KeyCode::Tab if !ctrl && !alt => Action::Insert('\t'),
        _ => Action::Ignore,
    }
}

/// The footer is generated from the same bindings used by `resolve_key`, so
/// visible hints cannot silently drift away from actual key handling.
pub fn shortcut_line(mode: EditorMode) -> String {
    shortcuts_for(mode)
        .iter()
        .filter(|shortcut| {
            !matches!(
                shortcut.action,
                Action::MoveLeft
                    | Action::MoveRight
                    | Action::MoveUp
                    | Action::MoveDown
                    | Action::Backspace
                    | Action::Enter
            )
        })
        .map(|shortcut| format!("{} {}", shortcut.hint_key, shortcut.hint_label))
        .collect::<Vec<_>>()
        .join("    ")
}
