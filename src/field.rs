use crate::tile::{Tile, TileKind};
use rand::thread_rng;
use std::fmt;

use rand::Rng;

type TileMatrix = Vec<Vec<Tile>>;

trait TileMatrixTrait {
    fn create_empty(rows: usize, cols: usize) -> Self;
    fn populate_bombs(
        self,
        rows: usize,
        cols: usize,
        bomb_percentage: f32,
        total_num_of_bombs: usize,
        prev_tile_matrix: Option<TileMatrix>,
    ) -> Self;
    fn populate_neighbours(self) -> Self;
    fn check_bounds(self: Self, x: i32, y: i32) -> Option<TileMatrix>;
}

impl TileMatrixTrait for TileMatrix {
    fn check_bounds(self, x: i32, y: i32) -> Option<TileMatrix> {
        let lower_bound = 0;
        let upper_bound = (self.len() - 1) as i32;

        if x < lower_bound || y < lower_bound || x > upper_bound || y > upper_bound {
            None
        } else {
            Some(self)
        }
    }

    fn create_empty(rows: usize, cols: usize) -> Self {
        let make_row = || (0..cols).map(|_| Tile::new_empty()).collect();
        let field = (0..rows).map(|_| make_row()).collect();

        field
    }

    fn populate_bombs(
        self,
        rows: usize,
        cols: usize,
        bomb_percentage: f32,
        total_num_of_bombs: usize,
        prev_tile_matrix: Option<TileMatrix>,
    ) -> Self {
        let mut bombs_populated = 0;

        let tile_matrix = prev_tile_matrix
            .unwrap_or(Self::create_empty(rows, cols))
            .into_iter()
            .map(|tiles| {
                tiles
                    .into_iter()
                    .map(|tile| {
                        if total_num_of_bombs - bombs_populated == 0 {
                            tile
                        } else {
                            let is_bomb = (thread_rng().gen_range(0.0..1.0)) <= bomb_percentage;
                            if !is_bomb {
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

        if total_num_of_bombs - bombs_populated == 0 {
            tile_matrix
        } else {
            self.populate_bombs(
                rows,
                cols,
                bomb_percentage,
                total_num_of_bombs - bombs_populated,
                Some(tile_matrix),
            )
        }
    }

    fn populate_neighbours(self) -> Self {
        fn get_tile(tile_matrix: TileMatrix, x: i32, y: i32) -> Option<Tile> {
            tile_matrix
                .check_bounds(x, y)
                .and_then(|matrix| Some(matrix[x as usize][y as usize].clone()))
        }

        self.iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .map(|(col, tile)| match tile.kind {
                        TileKind::Empty => {
                            let (x, y) = (row as i32, col as i32);

                            let top_left = get_tile(self.clone(), x - 1, y - 1);
                            let top = get_tile(self.clone(), x - 1, y);
                            let top_right = get_tile(self.clone(), x - 1, y + 1);
                            let right = get_tile(self.clone(), x, y + 1);
                            let bottom_right = get_tile(self.clone(), x + 1, y + 1);
                            let bottom = get_tile(self.clone(), x + 1, y);
                            let bottom_left = get_tile(self.clone(), x + 1, y - 1);
                            let left = get_tile(self.clone(), x, y - 1);

                            let neighbouring_bombs: Vec<Tile> = vec![
                                top_left,
                                top,
                                top_right,
                                right,
                                bottom_right,
                                bottom,
                                bottom_left,
                                left,
                            ]
                            .into_iter()
                            .filter(|tile| tile.is_some())
                            .map(|tile| tile.unwrap())
                            .filter(|tile| tile.kind == TileKind::Bomb)
                            .collect();

                            if neighbouring_bombs.len() == 0 {
                                Tile::new_empty()
                            } else {
                                Tile::new_safe(neighbouring_bombs.into_iter().collect())
                            }
                        }
                        _ => tile.clone(),
                    })
                    .collect()
            })
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Field {
    rows: usize,
    cols: usize,
    field: TileMatrix,
}

#[allow(unused_must_use)]
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tiles in self.field.iter() {
            for tile in tiles {
                write!(f, "{}", &*tile.repr);
            }
            writeln!(f, "");
        }
        write!(f, "")
    }
}

#[allow(dead_code)]
impl Field {
    pub fn create(rows: usize, cols: usize) -> Self {
        let bomb_percentage: f32 = 0.2; // 35% chance the tile is a bomb
        let total_num_of_bombs = (((rows * cols) as f32) * bomb_percentage) as usize;

        Self {
            rows,
            cols,
            field: TileMatrix::new()
                .populate_bombs(rows, cols, bomb_percentage, total_num_of_bombs, None)
                .populate_neighbours(),
        }
    }
}
