use std::io::Stdout;

use termion;
use termion::event::Key;
use termion::raw::RawTerminal;

use std::io::Write;

use crate::field::Field;
use crate::tile::{Tile, TileKind};

pub type UnsafePosition = (i32, i32);
pub type Position = (usize, usize);

pub fn to_unsafe_position(pos: Position) -> UnsafePosition {
    (pos.0.try_into().unwrap(), pos.1.try_into().unwrap())
}
pub fn to_safe_position(pos: UnsafePosition) -> Position {
    (pos.0.try_into().unwrap(), pos.1.try_into().unwrap())
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Nightmare
}

#[allow(dead_code)]
enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Sweeper {
    pub difficulty: Difficulty,
    pub field: Field,
}

#[allow(dead_code)]
impl Sweeper {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            field: {
                match difficulty {
                    Difficulty::Easy => Field::create(9, 9, 10),
                    Difficulty::Medium => Field::create(16, 16, 40),
                    Difficulty::Hard => Field::create(16, 30, 99),
                    Difficulty::Nightmare => Field::create(25, 55, 500),
                }
            },
        }
    }

    fn move_cursor(
        &self,
        current_cursor: UnsafePosition,
        direction: CursorDirection,
        step: i32,
    ) -> Position {
        let mut new_cursor = match direction {
            CursorDirection::Up => (current_cursor.0 - step, current_cursor.1),
            CursorDirection::Down => (current_cursor.0 + step, current_cursor.1),
            CursorDirection::Left => (current_cursor.0, current_cursor.1 - step),
            CursorDirection::Right => (current_cursor.0, current_cursor.1 + step),
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

    fn reveal_recursively(
        &mut self,
        position: Position,
        is_revealing_after_populating: bool,
        _max_depth: u8,
    ) {
        if _max_depth == 0 {
            return;
        }

        let tile = self.field.get_tile(to_unsafe_position(position)).unwrap();
        let neighbours = self.field.get_neighbours(to_unsafe_position(position));
        let safe_neighbours = neighbours
            .iter()
            .filter(|t| t.kind != TileKind::Bomb && !t.flagged && !t.revealed);
        let flagged_neighbours = neighbours
            .iter()
            .filter(|t| t.flagged)
            .collect::<Vec<&Tile>>();

        self.field = self.field.reveal(position);

        match tile.revealed {
            true => match tile.kind {
                TileKind::Empty => safe_neighbours
                    .for_each(|t| self.reveal_recursively(t.position, false, _max_depth - 1)),
                TileKind::Safe(bomb_count) => {
                    match flagged_neighbours.len() == bomb_count.try_into().unwrap() {
                        true => neighbours
                            .clone()
                            .iter()
                            .filter(|t| !t.flagged)
                            .for_each(|t| self.field = self.field.reveal(t.position)),
                        false => {}
                    }
                }
                TileKind::Bomb => self.field = self.field.game_over(),
            },
            false => match is_revealing_after_populating {
                true => safe_neighbours
                    .for_each(|t| self.reveal_recursively(t.position, false, _max_depth - 1)),
                false => match tile.kind {
                    TileKind::Safe(bomb_count) => {
                        match flagged_neighbours.len() == bomb_count.try_into().unwrap() {
                            true => neighbours
                                .clone()
                                .iter()
                                .filter(|t| !t.flagged)
                                .for_each(|t| self.reveal_recursively(t.position, false, 1)),
                            false => {}
                        }
                    }
                    TileKind::Empty => safe_neighbours
                        .for_each(|t| self.reveal_recursively(t.position, false, _max_depth - 1)),
                    TileKind::Bomb => self.field = self.field.game_over(),
                },
            },
        }
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

    pub fn tick(&mut self, key: &Key, mut sweeper_cursor: Position) -> (bool, bool, Position) {
        let unsafe_sweeper_cursor: UnsafePosition = to_unsafe_position(sweeper_cursor);

        sweeper_cursor = match key {
            Key::Char('w') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Up, 1),
            Key::Char('s') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Down, 1),
            Key::Char('a') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Left, 1),
            Key::Char('d') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Right, 1),
            Key::Alt('w') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Up, 2),
            Key::Alt('s') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Down, 2),
            Key::Alt('a') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Left, 2),
            Key::Alt('d') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Right, 2),
            Key::Char('i') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Up, 1),
            Key::Char('k') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Down, 1),
            Key::Char('j') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Left, 1),
            Key::Char('l') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Right, 1),
            Key::Alt('i') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Up, 2),
            Key::Alt('k') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Down, 2),
            Key::Alt('j') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Left, 2),
            Key::Alt('l') => self.move_cursor(unsafe_sweeper_cursor, CursorDirection::Right, 2),
            _ => sweeper_cursor,
        };

        self.select(sweeper_cursor);

        match key {
            Key::Char('f') => {
                self.field = self.field.toggle_flag(sweeper_cursor);
                ()
            }
            Key::Char(' ') | Key::Char('e') => {
                let are_all_fields_empty = self
                    .field
                    .tile_matrix
                    .clone()
                    .iter()
                    .flatten()
                    .all(|tile| tile.kind == TileKind::Empty);

                if are_all_fields_empty {
                    self.field = self.field.populate(sweeper_cursor);
                }
                self.reveal_recursively(sweeper_cursor, are_all_fields_empty, 5);
            }
            _ => {}
        };

        let should_exit = match key {
            Key::Char('q') | Key::Ctrl('c') => true,
            _ => false,
        };

        let should_restart = match key {
            Key::Char('r') => true,
            _ => false,
        };

        (should_exit, should_restart, sweeper_cursor)
    }

    pub fn select(&mut self, position: Position) {
        self.field = self.field.select(position);
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

        let sweeper_cursor = (3, 3);

        let are_all_fields_empty = sweeper
            .field
            .tile_matrix
            .clone()
            .iter()
            .flatten()
            .all(|tile| tile.kind == TileKind::Empty);

        if are_all_fields_empty {
            sweeper.field = sweeper.field.populate(sweeper_cursor);
        }
        sweeper.reveal_recursively(sweeper_cursor, are_all_fields_empty, 5);

        sweeper.display_field(&mut stdout)
    }
}
