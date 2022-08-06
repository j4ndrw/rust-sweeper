pub type TileSignedPosition = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileKind {
    Safe,
    Bomb,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub neighbouring_bombs: Vec<Tile>,
    pub revealed: bool,
    pub flagged: bool,
}

impl Tile {
    fn new(kind: TileKind, neighbouring_bombs: Vec<Tile>, revealed: bool) -> Self {
        let checked_tile_kind = match (kind, neighbouring_bombs.len()) {
            (TileKind::Safe, 0) => TileKind::Empty,
            _ => kind,
        };

        Self {
            kind: checked_tile_kind,
            neighbouring_bombs,
            revealed,
            flagged: false,
        }
    }

    pub fn new_empty() -> Self {
        Self::new(TileKind::Empty, vec![], true)
    }

    pub fn new_safe(neighbouring_bombs: Vec<Tile>) -> Self {
        Self::new(TileKind::Safe, neighbouring_bombs, true)
    }

    pub fn new_bomb() -> Self {
        Self::new(TileKind::Bomb, vec![], true)
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

    pub fn repr(&self) -> String {
        match self.flagged {
            true => " ❔ ".to_string(),
            _ => match self.revealed {
                false => " · ".to_string(),
                true => match self.kind {
                    TileKind::Bomb => " ❌ ".to_string(),
                    TileKind::Empty => "   ".to_string(),
                    TileKind::Safe => format!(" {} ", self.neighbouring_bombs.len()),
                },
            }
        }
    }
}
