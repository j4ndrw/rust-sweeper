use crate::sweeper::Position;

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
    pub selected: bool,
    pub position: Position,
}

impl Tile {
    fn new(kind: TileKind, neighbours: Vec<Tile>, revealed: bool, position: Position) -> Self {
        Self {
            kind: match (kind, neighbours.len()) {
                (TileKind::Safe, 0) => TileKind::Empty,
                _ => kind,
            },
            neighbours,
            revealed,
            flagged: false,
            selected: false,
            position,
        }
    }

    pub fn new_empty(position: Position) -> Self {
        Self::new(TileKind::Empty, vec![], false, position)
    }

    pub fn new_safe(position: Position, neighbours: Vec<Tile>) -> Self {
        Self::new(TileKind::Safe, neighbours, false, position)
    }

    pub fn new_bomb(position: Position) -> Self {
        Self::new(TileKind::Bomb, vec![], false, position)
    }

    pub fn set_neighbours(&self, neighbours: Vec<Tile>) -> Self {
        Self {
            neighbours,
            ..self.clone()
        }
    }

    pub fn reveal(&self) -> Self {
        Self {
            revealed: true,
            ..self.clone()
        }
    }

    pub fn flag(&self) -> Self {
        Self {
            flagged: true,
            ..self.clone()
        }
    }

    pub fn unflag(&self) -> Self {
        Self {
            flagged: false,
            ..self.clone()
        }
    }

    pub fn select(&self) -> Self {
        Self {
            selected: true,
            ..self.clone()
        }
    }

    pub fn deselect(&self) -> Self {
        Self {
            selected: false,
            ..self.clone()
        }
    }

    pub fn repr(&self) -> String {
        match self.flagged {
            true => "?".to_string(),
            _ => match self.revealed {
                false => "·".to_string(),
                true => match self.kind {
                    TileKind::Bomb => "&".to_string(),
                    TileKind::Empty => " ".to_string(),
                    TileKind::Safe => format!("{}", self.neighbours.len()),
                },
            },
        }
    }

    pub fn padded_repr(&self) -> String {
        format!(" {} ", self.repr())
    }
}
