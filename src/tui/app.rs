use std::collections::HashMap;

#[derive(Default)]
pub struct Challenge {
    pub completed: bool,
    pub input: Option<String>,
}

pub struct App {
    pub days: HashMap<(u8, u8), Challenge>,
    pub selected_day: u8,
    pub selected_part: u8,
    pub cargo_check_output: String,
    pub cargo_test_output: String,
    pub input_mode: bool,
    pub current_input: String,
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
            cargo_check_output: String::new(),
            cargo_test_output: String::new(),
            input_mode: false,
            current_input: String::new(),
        }
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

    pub fn set_input(&mut self, input: String) {
        if let Some(challenge) = self.days.get_mut(&(self.selected_day, self.selected_part)) {
            challenge.input = Some(input);
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app() {
        let app = App::new();
        assert_eq!(app.selected_day, 1);
        assert_eq!(app.selected_part, 1);
        assert!(!app.input_mode);
        assert!(app.current_input.is_empty());
        assert!(app.cargo_check_output.is_empty());
        assert!(app.cargo_test_output.is_empty());

        // Check that all days are initialized
        for day in 1..=25 {
            for part in 1..=2 {
                let challenge = app.days.get(&(day, part)).expect("Day/part should exist");
                assert!(!challenge.completed);
                assert!(challenge.input.is_none());
            }
        }
    }

    #[test]
    fn test_move_cursor() {
        let mut app = App::new();

        // Test moving down
        app.move_cursor(Direction::Down);
        assert_eq!(app.selected_day, 2);

        // Test moving up
        app.move_cursor(Direction::Up);
        assert_eq!(app.selected_day, 1);

        // Test moving right
        app.move_cursor(Direction::Right);
        assert_eq!(app.selected_part, 2);

        // Test moving left
        app.move_cursor(Direction::Left);
        assert_eq!(app.selected_part, 1);

        // Test boundaries
        app.selected_day = 1;
        app.move_cursor(Direction::Up);
        assert_eq!(app.selected_day, 1); // Should not go below 1

        app.selected_day = 25;
        app.move_cursor(Direction::Down);
        assert_eq!(app.selected_day, 25); // Should not go above 25

        app.selected_part = 1;
        app.move_cursor(Direction::Left);
        assert_eq!(app.selected_part, 1); // Should not go below 1

        app.selected_part = 2;
        app.move_cursor(Direction::Right);
        assert_eq!(app.selected_part, 2); // Should not go above 2
    }

    #[test]
    fn test_toggle_completion() {
        let mut app = App::new();

        // Test initial state
        let key = (app.selected_day, app.selected_part);
        assert!(!app.days.get(&key).unwrap().completed);

        // Test toggle on
        app.toggle_completion();
        assert!(app.days.get(&key).unwrap().completed);

        // Test toggle off
        app.toggle_completion();
        assert!(!app.days.get(&key).unwrap().completed);
    }

    #[test]
    fn test_set_input() {
        let mut app = App::new();
        let test_input = "test input".to_string();

        // Test setting input
        app.set_input(test_input.clone());
        let key = (app.selected_day, app.selected_part);
        assert_eq!(app.days.get(&key).unwrap().input, Some(test_input));

        // Test overwriting input
        let new_input = "new input".to_string();
        app.set_input(new_input.clone());
        assert_eq!(app.days.get(&key).unwrap().input, Some(new_input));
    }
}
