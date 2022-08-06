use std::io::Stdout;

use termion;
use termion::event::Key;
use termion::raw::RawTerminal;

use std::io::Write;

use crate::field::Field;

type Position = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Sweeper {
    difficulty: Difficulty,
    pub field: Field,
}

impl Sweeper {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            field: {
                match difficulty {
                    Difficulty::Easy => Field::create(9, 9, 10),
                    Difficulty::Medium => Field::create(16, 16, 40),
                    Difficulty::Hard => Field::create(16, 30, 99),
                }
            },
        }
    }

    fn move_cursor(&mut self, current_cursor: Position, direction: CursorDirection) -> Position {
        let mut new_cursor = match direction {
            CursorDirection::Up => (current_cursor.0 - 1, current_cursor.1),
            CursorDirection::Down => (current_cursor.0 + 1, current_cursor.1),
            CursorDirection::Left => (current_cursor.0, current_cursor.1 - 1),
            CursorDirection::Right => (current_cursor.0, current_cursor.1 + 1),
        };

        if new_cursor.0 < 0 {
            new_cursor.0 = 0
        }
        if new_cursor.0 > (self.field.rows as i32) - 1 {
            new_cursor.0 = (self.field.rows as i32) - 1
        }

        if new_cursor.1 < 0 {
            new_cursor.1 = 0
        }
        if new_cursor.1 > (self.field.cols as i32) - 1 {
            new_cursor.1 = (self.field.cols as i32) - 1
        }

        new_cursor
    }

    pub fn display_field(&self, stdout: &mut RawTerminal<Stdout>) {
        stdout.suspend_raw_mode().unwrap();
        writeln!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            self.field,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
        )
        .unwrap();
        stdout.activate_raw_mode().unwrap();
    }

    pub fn tick(&mut self, key: &Key, mut sweeper_cursor: Position) -> (bool, Position) {
        sweeper_cursor = match key {
            Key::Char('w') => self.move_cursor(sweeper_cursor.clone(), CursorDirection::Up),
            Key::Char('s') => self.move_cursor(sweeper_cursor.clone(), CursorDirection::Down),
            Key::Char('a') => self.move_cursor(sweeper_cursor.clone(), CursorDirection::Left),
            Key::Char('d') => self.move_cursor(sweeper_cursor.clone(), CursorDirection::Right),
            _ => sweeper_cursor,
        };

        self.field.select((
            (sweeper_cursor.0.try_into().unwrap()),
            (sweeper_cursor.1.try_into().unwrap()),
        ));

        let should_exit = match key {
            Key::Char('q') | Key::Ctrl('c') => true,
            _ => false,
        };

        (should_exit, sweeper_cursor)
    }
}
