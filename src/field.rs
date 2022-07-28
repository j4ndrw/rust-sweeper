use crate::tile::{Tile, TileKind};
use std::fmt;

#[derive(Debug)]
pub struct Field {
    rows: usize,
    cols: usize,
    field: Vec<Vec<Tile>>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tiles in self.field.iter() {
            for tile in tiles {
                match tile.kind {
                    TileKind::Bomb => write!(f, "[{}]", '*'),
                    TileKind::Empty => write!(f, "[{}]", ' '),
                    TileKind::Safe => write!(f, "[{}]", tile.neighbours.len()),
                };
            }
            writeln!(f, "");
        }
        write!(f, "")
    }
}

impl Field {
    pub fn create(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            field: vec![vec![Tile::make_random(); cols]; rows],
        }
    }
}
