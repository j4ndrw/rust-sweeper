use std::io::Stdout;

use termion;
use termion::event::Key;
use termion::raw::RawTerminal;

use std::io::Write;

use crate::field::{Field, TileMatrix};
use crate::tile::{Tile, TileKind};

pub type UnsafePosition = (i32, i32);
pub type Position = (usize, usize);

pub fn to_unsafe_position(pos: Position) -> UnsafePosition {
    (pos.0.try_into().unwrap(), pos.1.try_into().unwrap())
}
pub fn to_safe_position(pos: UnsafePosition) -> Position {
    (pos.0.try_into().unwrap(), pos.1.try_into().unwrap())
}

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

    fn move_cursor(
        &mut self,
        current_cursor: UnsafePosition,
        direction: CursorDirection,
    ) -> Position {
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

        to_safe_position(new_cursor)
    }

    fn reveal_recursively(&mut self, position: Position) {
        self.field.reveal(position);

        self.field
            .get_tile(to_unsafe_position(position))
            .unwrap()
            .neighbours
            .clone()
            .into_iter()
            .filter(|neighbour| neighbour.kind != TileKind::Bomb)
            .map(|neighbour| neighbour.position)
            .for_each(|position| self.reveal_recursively(position));
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
        let unsafe_sweeper_cursor: UnsafePosition = to_unsafe_position(sweeper_cursor);

        sweeper_cursor = match key {
            Key::Char('w') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Up),
            Key::Char('s') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Down),
            Key::Char('a') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Left),
            Key::Char('d') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Right),
            _ => sweeper_cursor,
        };

        self.field.select(sweeper_cursor);

        match key {
            Key::Char('f') => self.field.toggle_flag(sweeper_cursor),
            Key::Char('e') => {
                if self
                    .field
                    .tile_matrix
                    .clone()
                    .iter()
                    .all(|tiles| tiles.into_iter().all(|tile| tile.kind == TileKind::Empty))
                {
                    self.field.populate(sweeper_cursor);
                }

                self.reveal_recursively(sweeper_cursor);
            }
            _ => {}
        };

        let should_exit = match key {
            Key::Char('q') | Key::Ctrl('c') => true,
            _ => false,
        };

        (should_exit, sweeper_cursor)
    }
}

#[cfg(test)]
mod tests {
    use std::io::stdout;

    use termion::raw::IntoRawMode;

    use super::*;

    #[test]
    fn test_reveal_recursively() {
        let mut stdout = stdout().into_raw_mode().unwrap();

        let mut sweeper = Sweeper::new(Difficulty::Easy);

        let sweeper_cursor = (5, 5);

        if sweeper
            .field
            .tile_matrix
            .clone()
            .iter()
            .all(|tiles| tiles.into_iter().all(|tile| tile.kind == TileKind::Empty))
        {
            sweeper.field.populate(sweeper_cursor);
        }

        sweeper.reveal_recursively(sweeper_cursor);

        sweeper.display_field(&mut stdout)
    }
}
