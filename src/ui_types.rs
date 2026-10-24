#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorMode {
    #[default]
    Writing,
    Saving,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Viewport {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct UiState {
    pub mode: EditorMode,
    pub modified: bool,
    pub target_path: String,
    pub message: Option<String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            mode: EditorMode::Writing,
            modified: false,
            target_path: String::new(),
            message: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenSize {
    pub width: u16,
    pub _height: u16,
    pub text_rows: u16,
    pub gutter_width: u16,
}

impl ScreenSize {
    pub fn new(width: u16, _height: u16, line_count: usize) -> Self {
        let digits = line_count.max(1).to_string().len() as u16;
        Self {
            width,
            _height,
            text_rows: _height.saturating_sub(3),
            gutter_width: digits.saturating_add(3),
        }
    }
    pub fn text_width(self) -> usize {
        self.width.saturating_sub(self.gutter_width) as usize
    }
}
