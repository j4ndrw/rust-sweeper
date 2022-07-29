use crate::tile::Tile;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Field {
    rows: usize,
    cols: usize,
    field: Vec<Vec<Tile>>,
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
        Self {
            rows,
            cols,
            field: vec![vec![Tile::make_random(); cols]; rows],
        }
    }
}
