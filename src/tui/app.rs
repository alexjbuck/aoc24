// src/tui/app.rs
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

#[derive(Default)]
pub struct Challenge {
    pub completed: bool,
    pub input: Option<String>,
}

pub struct App {
    pub selected_day: u8,
    pub selected_part: u8,
    pub watched_day: u8,
    pub watched_part: u8,
    pub scroll: u16,
    pub input_mode: bool,
    pub cursor_blink_state: bool,
    pub show_scroll_hint: bool, // To show scroll hint briefly when entering input mode
    pub cursor_position: usize,
    pub current_input: String,
    pub cargo_output: String,
    pub days: HashMap<(u8, u8), Challenge>,
}

impl App {
    pub fn new() -> Self {
        let mut days = HashMap::new();
        for day in 1..=25 {
            for part in 1..=2 {
                days.insert((day, part), Challenge::default());
            }
        }

        Self {
            days,
            selected_day: 1,
            selected_part: 1,
            cargo_output: String::new(),
            input_mode: false,
            current_input: String::new(),
            cursor_position: 0,
            cursor_blink_state: false,
            watched_day: 1,
            watched_part: 1,
            scroll: 0,
            show_scroll_hint: false,
        }
    }

    pub fn watch(&mut self, day: u8, part: u8) {
        self.watched_day = day;
        self.watched_part = part;
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up if self.selected_day > 1 => self.selected_day -= 1,
            Direction::Down if self.selected_day < 25 => self.selected_day += 1,
            Direction::Left if self.selected_part > 1 => self.selected_part -= 1,
            Direction::Right if self.selected_part < 2 => self.selected_part += 1,
            _ => {}
        }
    }

    pub fn toggle_completion(&mut self) {
        if let Some(challenge) = self.days.get_mut(&(self.selected_day, self.selected_part)) {
            challenge.completed = !challenge.completed;
        }
    }

    fn get_input_path(day: u8, part: u8) -> PathBuf {
        PathBuf::from(format!("inputs/day{:02}_part{}.txt", day, part))
    }

    pub fn load_input(&mut self) {
        let input_path = App::get_input_path(self.selected_day, self.selected_part);
        if let Ok(input) = std::fs::read_to_string(input_path) {
            self.current_input = input.clone();
            self.update_challenge_input(input)
        }
    }

    fn update_challenge_input(&mut self, input: String) {
        // Update in memory
        if let Some(challenge) = self.days.get_mut(&(self.selected_day, self.selected_part)) {
            challenge.input = Some(input);
        }
    }

    pub fn set_input(&mut self, input: String) -> Result<()> {
        // Save to file
        let input_path = App::get_input_path(self.selected_day, self.selected_part);
        std::fs::create_dir_all(input_path.parent().unwrap())?;
        std::fs::write(&input_path, &input)?;
        self.update_challenge_input(input);
        Ok(())
    }

    pub fn get_selected_challenge(&self) -> Option<&Challenge> {
        self.days.get(&(self.selected_day, self.selected_part))
    }
    pub fn insert_char(&mut self, c: char) {
        self.current_input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn insert_newline(&mut self) {
        self.current_input.insert(self.cursor_position, '\n');
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            if self.cursor_position < self.current_input.len() {
                self.current_input.remove(self.cursor_position);
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.current_input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.current_input.len();
    }

    pub fn insert_text(&mut self, text: &str) {
        self.current_input.insert_str(self.cursor_position, text);
        self.cursor_position += text.len();
    }

    pub fn toggle_cursor_blink(&mut self) {
        self.cursor_blink_state = !self.cursor_blink_state;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        // We don't limit scrolling here because we don't know the content height
        // The UI will handle preventing over-scrolling
        self.scroll += 1;
    }

    pub fn page_up(&mut self) {
        if self.scroll >= 10 {
            self.scroll -= 10;
        } else {
            self.scroll = 0;
        }
    }

    pub fn page_down(&mut self) {
        self.scroll += 10;
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
