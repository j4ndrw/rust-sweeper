use std::io::Stdout;

use termion;
use termion::event::Key;
use termion::raw::RawTerminal;

use std::io::Write;

use itertools::Itertools;

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

    fn reveal_recursively(
        &mut self,
        position: Position,
        positions_to_ignore: Option<Vec<Position>>,
        is_revealing_after_populating: bool,
    ) {
        let tile = self.field.get_tile(to_unsafe_position(position)).unwrap();

        if tile.revealed {
            return;
        }

        self.field.reveal(position);

        let neighbours = &self
            .field
            .get_neighbours(to_unsafe_position(position))
            .into_iter()
            .filter(|t| !t.revealed && t.kind != TileKind::Bomb);

        let positions_to_process = &neighbours
            .clone()
            .filter(|t| {
                !positions_to_ignore
                    .clone()
                    .unwrap_or_default()
                    .contains(&t.position)
                    || (is_revealing_after_populating && t.kind != TileKind::Bomb)
            })
            .map(|t| t.position);

        if positions_to_ignore.is_some()
            && positions_to_process
                .clone()
                .eq(neighbours.clone().map(|t| t.position).clone())
        {
            return;
        }

        let neighbour_positions_to_ignore: Vec<Position> = neighbours
            .clone()
            .flat_map(|t| t.neighbours)
            .map(|t| t.position)
            .chain(positions_to_ignore.clone().unwrap_or_default().into_iter())
            .unique()
            .collect();

        positions_to_process.clone().for_each(|pos| {
            self.reveal_recursively(
                pos,
                Some(neighbour_positions_to_ignore.clone()),
                is_revealing_after_populating,
            )
        });
    }

    pub fn display_field(&self, stdout: &mut RawTerminal<Stdout>) {
        stdout.suspend_raw_mode().unwrap();
        writeln!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            self.field,
            termion::cursor::Goto(1, 1),
            termion::cursor::SteadyBlock,
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
                    self.reveal_recursively(sweeper_cursor, None, true);
                } else {
                    self.reveal_recursively(sweeper_cursor, None, false);
                }
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

        let sweeper_cursor = (3, 3);

        if sweeper
            .field
            .tile_matrix
            .clone()
            .iter()
            .all(|tiles| tiles.into_iter().all(|tile| tile.kind == TileKind::Empty))
        {
            sweeper.field.populate(sweeper_cursor);
            sweeper.reveal_recursively(sweeper_cursor, None, true);
        } else {
            sweeper.reveal_recursively(sweeper_cursor, None, false);
        }

        sweeper.display_field(&mut stdout)
    }
}
