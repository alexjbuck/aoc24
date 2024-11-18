// src/commands/watch.rs
use crate::runner::Runner;
use crate::tui::app::{App, Direction};
use anyhow::{Context, Result};
use arboard::Clipboard;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use notify::{RecursiveMode, Watcher};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

fn run_check(app: &mut App, runner: &Runner) {
    app.cargo_output = format!(
        "Checking day {} part {}...\n",
        app.watched_day, app.watched_part
    );
    let result = runner.check_day(app.watched_day);
    app.cargo_output.push_str(&result);
}
fn run_tests(app: &mut App, runner: &Runner) {
    app.cargo_output = format!(
        "Testing day {} part {}...\n",
        app.watched_day, app.watched_part
    );
    let result = runner.test_day(app.watched_day);
    app.cargo_output.push_str(&result);
}

fn setup_watcher() -> Result<(notify::RecommendedWatcher, mpsc::Receiver<notify::Event>)> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = tx.send(event);
        }
    })?;

    // Watch each day's src directory specifically
    for day in 1..=25 {
        let day_path = PathBuf::from(format!("day{:02}/src", day));
        if day_path.exists() {
            watcher.watch(&day_path, RecursiveMode::Recursive)?;
        }
    }

    Ok((watcher, rx))
}
pub fn init_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}
pub fn init_tui() -> std::io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(std::io::stdout()))
}

pub fn restore_tui() -> std::io::Result<()> {
    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn execute() -> Result<()> {
    // Set up panic hook for terminal cleanup
    init_panic_hook();
    let mut terminal = init_tui()?;

    // Create app state and runner
    let mut app = App::new();
    let runner = Runner::default();

    // Setup clipboard
    let mut clipboard = Clipboard::new().context("Failed to initialize clipboard")?;

    // Setup file watcher
    let (_watcher, rx) = setup_watcher()?;

    let mut last_cursor_toggle = std::time::Instant::now();
    let cursor_blink_interval = Duration::from_millis(500);
    app.load_input();
    // Event loop
    loop {
        // Handle cursor blinking
        let now = std::time::Instant::now();
        if now.duration_since(last_cursor_toggle) >= cursor_blink_interval {
            app.toggle_cursor_blink();
            last_cursor_toggle = now;
        }
        // Draw the current state
        terminal.draw(|f| crate::tui::ui::draw(f, &mut app))?;

        // Check for file system events
        if let Ok(_event) = rx.try_recv() {
            run_check(&mut app, &runner);
        }

        // Handle keyboard events
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.input_mode {
                        match (key.code, key.modifiers) {
                            // Save and exit input mode
                            (KeyCode::Char('s'), KeyModifiers::CONTROL)
                            | (KeyCode::Enter, KeyModifiers::CONTROL) => {
                                app.set_input(app.current_input.clone())?;
                                app.input_mode = false;
                                app.current_input.clear();
                                app.cursor_position = 0;
                            }
                            // Paste from clipboard
                            (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                                if let Ok(text) = clipboard.get_text() {
                                    app.insert_text(&text);
                                }
                            }
                            // Regular enter for newline
                            (KeyCode::Enter, KeyModifiers::NONE) => {
                                app.insert_newline();
                            }
                            // Navigation
                            (KeyCode::Left, KeyModifiers::NONE) => app.move_cursor_left(),
                            (KeyCode::Right, KeyModifiers::NONE) => app.move_cursor_right(),
                            (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                                app.move_cursor_start()
                            }
                            (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                                app.move_cursor_end()
                            }
                            (KeyCode::PageUp, _) => app.page_up(),
                            (KeyCode::PageDown, _) => app.page_down(),
                            (KeyCode::Char('u'), KeyModifiers::CONTROL) => app.page_up(),
                            (KeyCode::Char('d'), KeyModifiers::CONTROL) => app.page_down(),
                            (KeyCode::Up, KeyModifiers::CONTROL) => app.scroll_up(),
                            (KeyCode::Down, KeyModifiers::CONTROL) => app.scroll_down(),
                            // Editing
                            (KeyCode::Char(c), KeyModifiers::NONE) => app.insert_char(c),
                            (KeyCode::Backspace, KeyModifiers::NONE) => app.delete_char(),
                            (KeyCode::Esc, KeyModifiers::NONE) => {
                                app.input_mode = false;
                                app.current_input.clear();
                                app.cursor_position = 0;
                            }
                            _ => {}
                        }
                    } else {
                        match (key.code, key.modifiers) {
                            (KeyCode::Char('q'), KeyModifiers::NONE) => break,
                            (KeyCode::Char('h'), KeyModifiers::NONE)
                            | (KeyCode::Left, KeyModifiers::NONE) => {
                                app.move_cursor(Direction::Left)
                            }
                            (KeyCode::Char('l'), KeyModifiers::NONE)
                            | (KeyCode::Right, KeyModifiers::NONE) => {
                                app.move_cursor(Direction::Right)
                            }
                            (KeyCode::Char('j'), KeyModifiers::NONE)
                            | (KeyCode::Down, KeyModifiers::NONE) => {
                                app.move_cursor(Direction::Down)
                            }
                            (KeyCode::Char('k'), KeyModifiers::NONE)
                            | (KeyCode::Up, KeyModifiers::NONE) => app.move_cursor(Direction::Up),
                            (KeyCode::PageUp, _) => app.page_up(),
                            (KeyCode::PageDown, _) => app.page_down(),
                            (KeyCode::Char('u'), KeyModifiers::CONTROL) => app.page_up(),
                            (KeyCode::Char('d'), KeyModifiers::CONTROL) => app.page_down(),
                            (KeyCode::Up, KeyModifiers::CONTROL) => app.scroll_up(),
                            (KeyCode::Down, KeyModifiers::CONTROL) => app.scroll_down(),
                            (KeyCode::Char('w'), KeyModifiers::NONE)
                            | (KeyCode::Enter, KeyModifiers::NONE) => {
                                app.watch(app.selected_day, app.selected_part);
                                run_check(&mut app, &runner);
                            }
                            (KeyCode::Char('t'), KeyModifiers::NONE) => {
                                run_tests(&mut app, &runner)
                            }
                            (KeyCode::Char('c'), KeyModifiers::NONE) => app.toggle_completion(),
                            (KeyCode::Char('i'), KeyModifiers::NONE) => {
                                app.input_mode = true;
                                app.load_input();
                                app.cursor_position = app.current_input.len();
                            }
                            (KeyCode::Char('r'), KeyModifiers::NONE) => {
                                if let Some(challenge) = app.get_selected_challenge() {
                                    if let Some(input) = &challenge.input {
                                        match runner.run_day(
                                            app.selected_day,
                                            app.selected_part,
                                            input,
                                        ) {
                                            Ok(result) => {
                                                app.cargo_output = format!("Result: {}", result);
                                            }
                                            Err(e) => {
                                                app.cargo_output = format!("Error: {}", e);
                                            }
                                        }
                                    } else {
                                        app.cargo_output = String::from(
                                            "No input provided. Press 'i' to add input.",
                                        );
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    restore_tui()?;
    Ok(())
}
