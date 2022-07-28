#[derive(Debug, Clone, PartialEq)]
pub enum TileKind {
    Safe,
    Bomb,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub neighbours: Vec<Tile>,
    pub revealed: bool,
}

impl Tile {
    pub fn new(kind: TileKind, neighbours: Vec<Tile>, revealed: bool) -> Self {
        let checked_tile_kind: TileKind = if kind == TileKind::Safe && neighbours.len() == 0 {
            TileKind::Empty
        } else {
            kind
        };

        Self {
            kind: checked_tile_kind,
            neighbours,
            revealed,
        }
    }

    pub fn make_random() -> Self {
        Self::new(
            TileKind::Safe,
            vec![Tile::new(TileKind::Bomb, vec![], false)],
            false,
        )
    }
}
