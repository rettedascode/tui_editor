use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(f.size());

    render_tabs(f, app, chunks[0]);
    render_main_content(f, app, chunks[1]);
    render_status_bar(f, app, chunks[2]);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let tab_titles: Vec<String> = app
        .tabs
        .iter()
        .enumerate()
        .map(|(i, tab)| {
            let mut name = tab.name.clone();
            if tab.modified {
                name.push_str(" *");
            }
            if i == app.current_tab {
                format!("▶ {}", name)
            } else {
                name
            }
        })
        .collect();

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn render_main_content(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = if app.show_file_explorer {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(30), // File explorer
                Constraint::Min(0),     // Editor
            ])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0)])
            .split(area)
    };

    if app.show_file_explorer {
        render_file_explorer(f, app, chunks[0]);
    }

    let editor_area = if app.show_file_explorer {
        chunks[1]
    } else {
        chunks[0]
    };
    render_editor(f, app, editor_area);
}

fn render_file_explorer(f: &mut Frame, app: &mut App, area: Rect) {
    let files = app.file_explorer.get_display_lines();
    let items: Vec<ListItem> = files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let style = if i == app.file_explorer.selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(file.clone()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Files")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn render_editor(f: &mut Frame, app: &mut App, area: Rect) {
    if let Some(tab) = app.get_current_tab() {
        let editor = &tab.editor;
        let content = &tab.content;

        // Ensure cursor is visible
        let mut editor_clone = editor.clone();
        editor_clone.ensure_cursor_visible(content, area.width as usize, area.height as usize);

        // Get visible lines
        let visible_lines = editor_clone.get_visible_lines(content, area.height as usize);

        // Create line numbers and content
        let mut display_lines = Vec::new();
        let start_line = editor_clone.scroll_offset.row;

        for (i, line) in visible_lines.iter().enumerate() {
            let line_num = start_line + i + 1;
            let line_num_str = format!("{:4} ", line_num);

            let mut spans = vec![Span::styled(
                line_num_str,
                Style::default().fg(Color::DarkGray),
            )];

            // Add line content with syntax highlighting (basic)
            let content_span = Span::styled(line.clone(), Style::default().fg(Color::White));
            spans.push(content_span);

            display_lines.push(Line::from(spans));
        }

        let paragraph = Paragraph::new(display_lines)
            .block(
                Block::default()
                    .title(format!(" {} ", tab.name))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);

        // Render cursor
        let cursor_x = editor
            .cursor
            .col
            .saturating_sub(editor_clone.scroll_offset.col);
        let cursor_y = editor
            .cursor
            .row
            .saturating_sub(editor_clone.scroll_offset.row);

        if cursor_y < area.height.saturating_sub(2) as usize
            && cursor_x < area.width.saturating_sub(6) as usize
        {
            f.set_cursor(
                area.x + cursor_x as u16 + 5, // +5 for line numbers
                area.y + cursor_y as u16 + 1, // +1 for border
            );
        }
    }
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status_text = if let Some(message) = &app.status_message {
        format!(" {} ", message)
    } else if let Some(tab) = app.get_current_tab() {
        let cursor = &tab.editor.cursor;
        let total_lines = tab.content.len_lines();
        let total_chars = tab.content.len_chars();

        format!(
            " Line: {}, Col: {} | Lines: {} | Chars: {} ",
            cursor.row + 1,
            cursor.col + 1,
            total_lines,
            total_chars
        )
    } else {
        " Ready ".to_string()
    };

    let status_style = Style::default().fg(Color::Black).bg(Color::White);

    let status = Paragraph::new(status_text)
        .style(status_style)
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(status, area);
}

pub fn render_help(f: &mut Frame, app: &App) {
    if !app.show_help {
        return;
    }

    let help_text = vec![
        Line::from(vec![Span::styled(
            "TUI Code Editor Help",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Arrow Keys - Move cursor"),
        Line::from("  Home/End - Line start/end"),
        Line::from("  Page Up/Down - Page navigation"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "File Operations:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Ctrl+N - New file"),
        Line::from("  Ctrl+O - Open file"),
        Line::from("  Ctrl+S - Save file"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Editor:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Tab - Toggle file explorer"),
        Line::from("  F1 - Toggle this help"),
        Line::from("  Q - Quit"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Press any key to close",
            Style::default().fg(Color::Green),
        )]),
    ];

    let help_block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let help_paragraph = Paragraph::new(help_text)
        .block(help_block)
        .style(Style::default().fg(Color::White));

    // Center the help dialog
    let popup_area = centered_rect(60, 20, f.size());
    f.render_widget(help_paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height.saturating_sub(percent_y)) / 2),
            Constraint::Length(percent_y),
            Constraint::Length((r.height.saturating_sub(percent_y)) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width.saturating_sub(percent_x)) / 2),
            Constraint::Length(percent_x),
            Constraint::Length((r.width.saturating_sub(percent_x)) / 2),
        ])
        .split(popup_layout[1])[1]
}
