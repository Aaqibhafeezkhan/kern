use std::{io, path::Path};
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, FontStyle, Style, Theme, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
};

#[derive(Clone, Debug)]
pub struct StyledSpan {
    pub style: Style,
    pub text: String,
}

pub struct SyntaxCache {
    syntaxes: SyntaxSet,
    theme: Theme,
    rows: Vec<Vec<StyledSpan>>,
    background: Color,
    syntax_name: String,
}

impl SyntaxCache {
    pub fn new(path: &Path, theme_name: &str) -> Self {
        let syntaxes = SyntaxSet::load_defaults_newlines();
        let themes = ThemeSet::load_defaults();
        let theme = themes
            .themes
            .get(theme_name)
            .or_else(|| themes.themes.get("base16-ocean.dark"))
            .or_else(|| themes.themes.values().next())
            .expect("syntect did not provide any default themes")
            .clone();
        let background = theme.settings.background.unwrap_or(Color {
            r: 30,
            g: 30,
            b: 30,
            a: 255,
        });
        let syntax_name = syntax_for_path(&syntaxes, path).name.to_owned();
        Self {
            syntaxes,
            theme,
            rows: Vec::new(),
            background,
            syntax_name,
        }
    }

    pub fn background(&self) -> Color {
        self.background
    }
    pub fn syntax_name(&self) -> &str {
        &self.syntax_name
    }
    pub fn rows(&self) -> &[Vec<StyledSpan>] {
        &self.rows
    }

    /// Rebuilds highlighting in source order. HighlightLines is stateful, so it must
    /// never be advanced by rendering or by synthetic padding characters.
    pub fn rebuild(&mut self, lines: &[String], path: &Path) -> io::Result<()> {
        let syntax = syntax_for_path(&self.syntaxes, path);
        self.syntax_name = syntax.name.to_owned();
        let mut highlighter = HighlightLines::new(syntax, &self.theme);
        let mut rows = Vec::with_capacity(lines.len());
        for source_line in lines {
            let mut with_newline = source_line.clone();
            with_newline.push('\n');
            let regions = highlighter
                .highlight_line(&with_newline, &self.syntaxes)
                .map_err(|e| io::Error::other(e.to_string()))?;
            let mut spans = Vec::with_capacity(regions.len());
            for (style, text) in regions {
                let visible = text.strip_suffix('\n').unwrap_or(text);
                if !visible.is_empty() {
                    spans.push(StyledSpan {
                        style,
                        text: visible.to_owned(),
                    });
                }
            }
            rows.push(spans);
        }
        self.rows = rows;
        Ok(())
    }
}

fn syntax_for_path<'a>(syntaxes: &'a SyntaxSet, path: &Path) -> &'a SyntaxReference {
    path.extension()
        .and_then(|s| s.to_str())
        .and_then(|ext| syntaxes.find_syntax_by_extension(ext))
        .unwrap_or_else(|| syntaxes.find_syntax_plain_text())
}

pub fn style_to_crossterm(style: Style) -> crossterm::style::ContentStyle {
    use crossterm::style::{Attribute, Color as CColor, ContentStyle};
    let mut out = ContentStyle {
        foreground_color: Some(CColor::Rgb {
            r: style.foreground.r,
            g: style.foreground.g,
            b: style.foreground.b,
        }),
        background_color: Some(CColor::Rgb {
            r: style.background.r,
            g: style.background.g,
            b: style.background.b,
        }),
        ..ContentStyle::default()
    };
    if style.font_style.contains(FontStyle::BOLD) {
        out.attributes.set(Attribute::Bold);
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        out.attributes.set(Attribute::Italic);
    }
    if style.font_style.contains(FontStyle::UNDERLINE) {
        out.attributes.set(Attribute::Underlined);
    }
    out
}
