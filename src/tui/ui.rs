// src/tui/ui.rs
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use super::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(f.area());

    draw_days_list(f, app, chunks[0]);

    if app.input_mode {
        draw_input_window(f, app, chunks[1]);
    } else {
        draw_output_window(f, app, chunks[1]);
    }
}

fn draw_days_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = (1..=25)
        .map(|day| {
            let mut spans = vec![];

            // Day number
            spans.push(Span::raw(format!("{:02} ", day)));

            // Part 1
            let part1_style = if app.days.get(&(day, 1)).map_or(false, |c| c.completed) {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            spans.push(Span::styled("1", part1_style));

            spans.push(Span::raw(" "));

            // Part 2
            let part2_style = if app.days.get(&(day, 2)).map_or(false, |c| c.completed) {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            spans.push(Span::styled("2", part2_style));

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Days"))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">");

    // Create and set up list state
    let mut list_state = ListState::default();
    list_state.select(Some((app.selected_day - 1) as usize));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn draw_output_window(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let check_output = Paragraph::new(app.cargo_check_output.as_str())
        .block(Block::default().borders(Borders::ALL).title("Cargo Check"));
    f.render_widget(check_output, chunks[0]);

    let test_output = Paragraph::new(app.cargo_test_output.as_str())
        .block(Block::default().borders(Borders::ALL).title("Cargo Test"));
    f.render_widget(test_output, chunks[1]);
}

fn draw_input_window(f: &mut Frame, app: &App, area: Rect) {
    let input = Paragraph::new(app.current_input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, area);
}
