// src/commands/watch.rs
use crate::runner::Runner;
use crate::tui::app::{App, Direction};
use anyhow::Result;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};
use notify::{RecursiveMode, Watcher};
use ratatui::Terminal;
use std::io::stdout;
use std::path::Path;
use tokio::sync::mpsc;
use tokio::time::Duration;

pub async fn execute() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(stdout()))?;

    // Create app state and runner
    let mut app = App::new();
    let runner = Runner::new()?;

    // Setup file watcher
    let (tx, mut rx) = mpsc::channel(32);
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = tx.blocking_send(event);
        }
    })?;

    // Watch the day crates directory
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    // Event loop
    loop {
        terminal.draw(|f| crate::tui::ui::draw(f, &app))?;

        tokio::select! {
            // Handle file system events
            Some(_event) = rx.recv() => {
                // Run cargo check and test on file changes
                let check_output = runner.check_day(app.selected_day)?;
                let test_output = runner.test_day(app.selected_day)?;
                app.cargo_check_output = check_output;
                app.cargo_test_output = test_output;
            }

            // Handle keyboard events
            Ok(true) = async { event::poll(Duration::from_millis(100)) } => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') | KeyCode::Left => app.move_cursor(Direction::Left),
                        KeyCode::Char('l') | KeyCode::Right => app.move_cursor(Direction::Right),
                        KeyCode::Char('j') | KeyCode::Down => app.move_cursor(Direction::Down),
                        KeyCode::Char('k') | KeyCode::Up => app.move_cursor(Direction::Up),
                        KeyCode::Char('c') => app.toggle_completion(),
                        KeyCode::Char('i') => app.input_mode = true,
                        KeyCode::Char('s') if app.input_mode => {
                            app.set_input(app.current_input.clone());
                            app.input_mode = false;
                            app.current_input.clear();
                        }
                        KeyCode::Char('r') => {
                            if let Some(challenge) = app.days.get(&(app.selected_day, app.selected_part)) {
                                if let Some(input) = &challenge.input {
                                    match runner.run_day(app.selected_day, app.selected_part, input) {
                                        Ok(result) => {
                                            app.cargo_check_output = format!("Result: {}", result);
                                        }
                                        Err(e) => {
                                            app.cargo_check_output = format!("Error: {}", e);
                                        }
                                    }
                                } else {
                                    app.cargo_check_output = String::from("No input provided. Press 'i' to add input.");
                                }
                            }
                        }
                        KeyCode::Char(c) if app.input_mode => {
                            app.current_input.push(c);
                        }
                        KeyCode::Backspace if app.input_mode => {
                            app.current_input.pop();
                        }
                        KeyCode::Esc if app.input_mode => {
                            app.input_mode = false;
                            app.current_input.clear();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
