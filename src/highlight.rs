use ratatui::style::{Color, Style};
use ratatui::text::Span;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

/// Highlighter provides syntax highlighting for code using syntect and ratatui.
pub struct Highlighter {
    /// The loaded syntax set for language definitions.
    pub syntax_set: SyntaxSet,
    /// The currently selected theme.
    pub theme: syntect::highlighting::Theme,
}

impl Highlighter {
    /// Create a new Highlighter with default syntax set and theme.
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();
        Self { syntax_set, theme }
    }

    /// Get the syntax definition for a file extension, or None if not found.
    pub fn get_syntax<'a>(&'a self, extension: &str) -> Option<&'a SyntaxReference> {
        self.syntax_set.find_syntax_by_extension(extension)
    }

    /// Highlight a line of code for a given file extension, returning ratatui Spans.
    pub fn highlight_line(&self, line: &str, extension: &str) -> Vec<Span<'static>> {
        let syntax = self
            .get_syntax(extension)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let mut h = HighlightLines::new(syntax, &self.theme);
        let regions = h.highlight_line(line, &self.syntax_set).unwrap_or_default();
        regions
            .into_iter()
            .map(|(style, text)| Span::styled(text.to_string(), syntect_style_to_tui(style)))
            .collect()
    }
}

/// Convert a syntect style to a ratatui style.
fn syntect_style_to_tui(style: SyntectStyle) -> Style {
    let fg = style.foreground;
    Style::default().fg(Color::Rgb(fg.r, fg.g, fg.b))
}
