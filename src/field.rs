use crate::tile::{Tile, TileKind, TileSignedPosition};
use rand::thread_rng;
use std::fmt;

use rand::Rng;

type TileMatrix = Vec<Vec<Tile>>;
type Position = (usize, usize);

trait TileMatrixTrait {
    fn create_empty(rows: usize, cols: usize) -> Self;
    fn check_bounds(self: Self, x: i32, y: i32) -> Option<TileMatrix>;
    fn populate_bombs(
        &self,
        empty_point: Position,
        rows: usize,
        cols: usize,
        bombs: usize,
        prev_tile_matrix: Option<TileMatrix>,
    ) -> Self;
    fn populate_neighbours(&self) -> Self;
}

impl TileMatrixTrait for TileMatrix {
    fn create_empty(rows: usize, cols: usize) -> Self {
        let make_row = || (0..cols).map(|_| Tile::new_empty()).collect();
        let field = (0..rows).map(|_| make_row()).collect();

        field
    }

    fn check_bounds(self, x: i32, y: i32) -> Option<TileMatrix> {
        let lower_bound = 0;
        let row_upper_bound = (self.len() - 1) as i32;
        let col_upper_bound = (self[0].len() - 1) as i32;

        let is_x_within_bounds = lower_bound <= x && x <= row_upper_bound;
        let is_y_within_bounds = lower_bound <= y && y <= col_upper_bound;

        match is_x_within_bounds && is_y_within_bounds {
            true => Some(self),
            false => None,
        }
    }

    fn populate_bombs(
        &self,
        empty_point: Position,
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
                            if !is_bomb || (row, col) == empty_point {
                                tile
                            } else {
                                bombs_populated += 1;
                                Tile::new_bomb()
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        match bombs - bombs_populated {
            0 => tile_matrix,
            _ => self.populate_bombs(
                empty_point,
                rows,
                cols,
                bombs - bombs_populated,
                Some(tile_matrix),
            ),
        }
    }

    fn populate_neighbours(&self) -> Self {
        let get_tile = |(x, y): TileSignedPosition| -> Option<Tile> {
            self.to_vec()
                .check_bounds(x, y)
                .and_then(|matrix| Some(matrix[x as usize][y as usize].clone()))
        };

        let replace_empty = |(x, y): TileSignedPosition| -> Tile {
            let neighbouring_bombs: Vec<Tile> = (x - 1..=x + 1)
                .flat_map(|i| (y - 1..=y + 1).flat_map(move |j| get_tile((i, j))))
                .filter(|tile| tile.kind == TileKind::Bomb)
                .collect();

            match neighbouring_bombs.len() {
                0 => Tile::new_empty(),
                _ => Tile::new_safe(neighbouring_bombs.into_iter().collect()),
            }
        };

        self.iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .map(|(col, tile)| match tile.kind {
                        TileKind::Empty => replace_empty((row as i32, col as i32)),
                        _ => tile.clone(),
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Field {
    rows: usize,
    cols: usize,
    bombs: usize,
    field: TileMatrix,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tiles in self.field.iter() {
            for tile in tiles {
                write!(f, "{}", &*tile.repr());
            }
            writeln!(f, "");
        }
        write!(f, "")
    }
}

impl Field {
    pub fn create(rows: usize, cols: usize, bombs: usize) -> Self {
        Self {
            rows,
            cols,
            bombs,
            field: TileMatrix::new(),
        }
    }

    pub fn populate(&mut self, starting_point: Position) {
        self.field = self
            .field
            .populate_bombs(starting_point, self.rows, self.cols, self.bombs, None)
            .populate_neighbours()
    }
}
