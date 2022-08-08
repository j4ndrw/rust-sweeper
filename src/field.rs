use crate::{
    sweeper::{Position, UnsafePosition},
    tile::{Tile, TileKind},
};
use rand::thread_rng;
use std::fmt;

use rand::Rng;

pub type TileMatrix = Vec<Vec<Tile>>;

trait TileMatrixTrait {
    fn create_empty(rows: usize, cols: usize) -> Self;
    fn check_bounds(&self, position: UnsafePosition) -> Option<TileMatrix>;
    fn get_tile(&self, position: &UnsafePosition) -> Option<Tile>;
    fn get_neighbours(&self, position: UnsafePosition) -> Vec<Tile>;
    fn populate_bombs(
        &self,
        selected_point: Position,
        rows: usize,
        cols: usize,
        bombs: usize,
        prev_tile_matrix: Option<TileMatrix>,
    ) -> Self;
    fn populate_neighbours(&self, with_bombs: bool) -> Self;
}

impl TileMatrixTrait for TileMatrix {
    fn create_empty(rows: usize, cols: usize) -> Self {
        (0..rows)
            .map(|row| {
                (0..cols)
                    .map(|col| Tile::new_empty(Position(row, col)))
                    .collect()
            })
            .collect::<Self>()
    }

    fn check_bounds(&self, position: UnsafePosition) -> Option<TileMatrix> {
        let (x, y) = (position.0, position.1);

        let lower_bound = 0;
        let row_upper_bound = (self.len() - 1) as i32;
        let col_upper_bound = (self[0].len() - 1) as i32;

        let is_x_within_bounds = lower_bound <= x && x <= row_upper_bound;
        let is_y_within_bounds = lower_bound <= y && y <= col_upper_bound;

        match is_x_within_bounds && is_y_within_bounds {
            true => Some(self.clone()),
            false => None,
        }
    }

    fn get_tile(&self, position: &UnsafePosition) -> Option<Tile> {
        self.to_vec()
            .check_bounds(position.clone())
            .and_then(|matrix| Some(matrix[position.0 as usize][position.1 as usize].clone()))
    }

    fn get_neighbours(&self, position: UnsafePosition) -> Vec<Tile> {
        let row_range = position.0 - 1..=position.0 + 1;
        let col_range = position.1 - 1..=position.1 + 1;

        row_range
            .flat_map(|row| {
                col_range
                    .clone()
                    .flat_map(move |col| self.get_tile(&UnsafePosition(row, col)))
            })
            .filter(|t| t.position != position.to_safe())
            .collect()
    }

    fn populate_bombs(
        &self,
        selected_point: Position,
        rows: usize,
        cols: usize,
        bombs: usize,
        prev_tile_matrix: Option<TileMatrix>,
    ) -> Self {
        let bomb_generation_frequency = 0.15; //15% frequency
        let mut bombs_populated = 0;

        let tile_matrix = prev_tile_matrix
            .unwrap_or(Self::create_empty(rows, cols))
            .into_iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .into_iter()
                    .enumerate()
                    .map(|(col, tile)| match bombs - bombs_populated {
                        0 => tile,
                        _ => {
                            let is_bomb =
                                (thread_rng().gen_range(0.0..1.0)) <= bomb_generation_frequency;

                            let is_selected_point = Position(row, col) == selected_point
                                || tile.neighbours.iter().any(|t| t.position == selected_point);

                            match is_selected_point {
                                true => Tile::new_empty(Position(row, col)),
                                false => match is_bomb {
                                    false => tile,
                                    true => {
                                        bombs_populated += 1;
                                        Tile::new_bomb(Position(row, col))
                                    }
                                },
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        match bombs - bombs_populated {
            0 => tile_matrix,
            _ => self.populate_bombs(
                selected_point,
                rows,
                cols,
                bombs - bombs_populated,
                Some(tile_matrix),
            ),
        }
    }

    fn populate_neighbours(&self, with_bombs: bool) -> Self {
        let replace_empty = |neighbours: Vec<Tile>, position: UnsafePosition| {
            let filtered_neighbours: Vec<Tile> = neighbours
                .into_iter()
                .filter(|tile| tile.kind == TileKind::Bomb)
                .collect();

            match filtered_neighbours.len() {
                0 => Tile::new_empty(position.to_safe()),
                _ => Tile::new_safe(
                    position.to_safe(),
                    filtered_neighbours.into_iter().collect(),
                ),
            }
        };

        self.iter()
            .enumerate()
            .map(|(row, tiles)| {
                let mapped_tiles = tiles.into_iter().enumerate().map(|(col, tile)| {
                    (
                        col,
                        tile.set_neighbours(self.get_neighbours(Position(row, col).to_unsafe())),
                    )
                });
                if !with_bombs {
                    mapped_tiles.map(|(_, tile)| tile).collect()
                } else {
                    mapped_tiles
                        .into_iter()
                        .map(|(col, tile)| match tile.kind {
                            TileKind::Empty => replace_empty(
                                tile.neighbours,
                                UnsafePosition(row as i32, col as i32),
                            ),
                            _ => tile.clone(),
                        })
                        .collect()
                }
            })
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Field {
    pub rows: usize,
    pub cols: usize,
    bombs: usize,
    pub tile_matrix: TileMatrix,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tiles in self.tile_matrix.iter() {
            for tile in tiles {
                match tile.selected {
                    true => write!(f, "[{}]", tile.repr()).unwrap(),
                    false => write!(f, "{}", tile.padded_repr()).unwrap(),
                };
            }
            writeln!(f, "").unwrap();
        }
        writeln!(f, "")
    }
}

#[allow(dead_code)]
impl Field {
    pub fn create(rows: usize, cols: usize, bombs: usize) -> Self {
        Self {
            rows,
            cols,
            bombs,
            tile_matrix: TileMatrix::create_empty(rows, cols),
        }
    }

    pub fn populate(&self, starting_point: &Position) -> Self {
        Self {
            tile_matrix: self
                .tile_matrix
                .clone()
                .populate_bombs(
                    starting_point.clone(),
                    self.rows,
                    self.cols,
                    self.bombs,
                    None,
                )
                .populate_neighbours(true),
            ..self.clone()
        }
    }

    pub fn get_tile(&self, position: UnsafePosition) -> Option<Tile> {
        self.tile_matrix.get_tile(&position)
    }

    pub fn get_neighbours(&self, position: UnsafePosition) -> Vec<Tile> {
        self.tile_matrix.get_neighbours(position)
    }

    pub fn apply_on_tile(
        &self,
        tile_position: Position,
        map_selected: &dyn Fn(Tile) -> Tile,
        optional_map_rest: Option<&dyn Fn(Tile) -> Tile>,
    ) -> TileMatrix {
        self.tile_matrix
            .clone()
            .into_iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .into_iter()
                    .enumerate()
                    .map(|(col, tile)| {
                        if tile_position == Position(row, col) {
                            return map_selected(tile);
                        }
                        if let Some(map_rest) = optional_map_rest {
                            return map_rest(tile);
                        }
                        tile
                    })
                    .collect()
            })
            .collect()
    }

    pub fn select(&self, tile_position: Position) -> Self {
        Self {
            tile_matrix: self.apply_on_tile(
                tile_position,
                &move |tile| tile.select(),
                Some(&move |tile| tile.deselect()),
            ),
            ..self.clone()
        }
    }

    pub fn toggle_flag(&self, tile_position: &Position) -> Self {
        Self {
            tile_matrix: self.apply_on_tile(
                tile_position.clone(),
                &move |tile| {
                    if tile.revealed {
                        return tile;
                    }
                    match tile.flagged {
                        true => tile.unflag(),
                        false => tile.flag(),
                    }
                },
                None,
            ),
            ..self.clone()
        }
    }

    pub fn reveal(&self, tile_position: &Position) -> Self {
        Self {
            tile_matrix: self.apply_on_tile(
                tile_position.clone(),
                &move |tile| {
                    if tile.revealed || tile.flagged {
                        return tile;
                    }

                    tile.reveal()
                },
                None,
            ),
            ..self.clone()
        }
    }

    pub fn game_over(&self) -> Self {
        Self {
            tile_matrix: self
                .tile_matrix
                .clone()
                .into_iter()
                .map(|tiles| tiles.into_iter().map(|t| t.unflag().reveal()).collect())
                .collect(),
            ..self.clone()
        }
    }
}
