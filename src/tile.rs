#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub flagged: bool,
    pub repr: String,
}

impl Tile {
    pub fn new(kind: TileKind, neighbours: Vec<Tile>, revealed: bool) -> Self {
        let checked_tile_kind: TileKind = if kind == TileKind::Safe && neighbours.len() == 0 {
            TileKind::Empty
        } else {
            kind
        };

        let num_neighbours = neighbours.clone().len();

        Self {
            kind: checked_tile_kind,
            neighbours,
            revealed,
            flagged: false,
            repr: if revealed {
                match checked_tile_kind {
                    TileKind::Bomb => "[💣]".to_string(),
                    TileKind::Empty => "[ ]".to_string(),
                    TileKind::Safe => format!("[{}]", num_neighbours)
                }
            } else {
                "[·]".to_string()
            },
        }
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }

    pub fn hide(&mut self) {
        self.revealed = false;
    }

    pub fn flag(&mut self) {
        self.flagged = true;
    }

    pub fn unflag(&mut self) {
        self.flagged = false;
    }

    pub fn make_random() -> Self {
        Self::new(
            TileKind::Safe,
            vec![Tile::new(TileKind::Bomb, vec![], false)],
            false,
        )
    }
}
