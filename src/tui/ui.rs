// stchrc/tui/ui.rs
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use super::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(9), Constraint::default()])
        .split(f.area());

    draw_days_grid(f, app, chunks[0]);

    if app.input_mode {
        draw_input_window(f, app, chunks[1]);
    } else {
        draw_output_window(f, app, chunks[1]);
    }
}

fn draw_days_grid(f: &mut Frame, app: &App, area: Rect) {
    let rows: Vec<Row> = (1..=25)
        .map(|day| {
            let day_cell = Span::raw(format!("{:02}", day));

            // Part 1 cell
            let part1_style = if app.days.get(&(day, 1)).map_or(false, |c| c.completed) {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            let part1_selected = day == app.selected_day && app.selected_part == 1;
            let part1_cell = if part1_selected {
                Span::styled("1", part1_style.bg(Color::DarkGray))
            } else {
                Span::styled("1", part1_style)
            };

            // Part 2 cell
            let part2_style = if app.days.get(&(day, 2)).map_or(false, |c| c.completed) {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            let part2_selected = day == app.selected_day && app.selected_part == 2;
            let part2_cell = if part2_selected {
                Span::styled("2", part2_style.bg(Color::DarkGray))
            } else {
                Span::styled("2", part2_style)
            };

            Row::new(vec![day_cell, part1_cell, part2_cell])
        })
        .collect();

    let widths = vec![
        Constraint::Length(3), // Day number
        Constraint::Length(2), // Part 1
        Constraint::Length(2), // Part 2
    ];
    let table = Table::new(rows, widths)
        .block(Block::default().borders(Borders::ALL).title("Days"))
        .column_spacing(1);

    f.render_widget(table, area);
}

fn draw_output_window(f: &mut Frame, app: &mut App, area: Rect) {
    let scroll_hint = if app.show_scroll_hint {
        " (Use PageUp/PageDown or Ctrl+u/Ctrl+d to scroll)"
    } else {
        ""
    };

    let block = Block::default().borders(Borders::ALL).title(format!(
        "Cargo output day {} part {}{}",
        app.watched_day, app.watched_part, scroll_hint
    ));

    let inner_area = block.inner(area);

    let check_output = Paragraph::new(app.cargo_output.as_str())
        .block(block)
        .scroll((app.scroll, 0));

    f.render_widget(check_output, area);

    // Adjust scroll if it would put the cursor out of view
    let lines: Vec<&str> = app.cargo_output.split('\n').collect();
    let newest_line = lines.len().saturating_sub(1) as u16;

    if newest_line < app.scroll {
        // Cursor is above visible area
        app.scroll = newest_line;
    } else if newest_line >= app.scroll + inner_area.height {
        // Cursor is below visible area
        app.scroll = newest_line.saturating_sub(inner_area.height) + 1;
    }
}

fn draw_input_window(f: &mut Frame, app: &mut App, area: Rect) {
    let scroll_hint = if app.show_scroll_hint {
        " (Use PageUp/PageDown or Ctrl+u/Ctrl+d to scroll)"
    } else {
        ""
    };
    // Create the text with cursor
    let mut text = app.current_input.clone();
    if app.cursor_blink_state && app.input_mode {
        // Insert the cursor at the current position
        text.insert(app.cursor_position, '_');
    }

    let block = Block::default().borders(Borders::ALL).title(format!(
        "Input (Ctrl+S or Ctrl+Enter to save, Ctrl+V to paste){}",
        scroll_hint
    ));
    let inner_area = block.inner(area);

    let input = Paragraph::new(text).block(block).scroll((app.scroll, 0));

    f.render_widget(input, area);

    // Adjust scroll if it would put the cursor out of view
    let lines: Vec<&str> = app.current_input[..app.cursor_position]
        .split('\n')
        .collect();
    let cursor_line = lines.len().saturating_sub(1) as u16;

    if cursor_line < app.scroll {
        // Cursor is above visible area
        app.scroll = cursor_line;
    } else if cursor_line >= app.scroll + inner_area.height {
        // Cursor is below visible area
        app.scroll = cursor_line.saturating_sub(inner_area.height) + 1;
    }
}
