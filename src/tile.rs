use crate::sweeper::Position;
use termion::color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileKind {
    Safe(u8),
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

#[allow(dead_code)]
impl Tile {
    fn new(kind: TileKind, neighbours: Vec<Tile>, position: Position) -> Self {
        Self {
            kind: match (kind, neighbours.len()) {
                (TileKind::Safe(0), 0) => TileKind::Empty,
                _ => kind,
            },
            neighbours,
            position,
            revealed: false,
            flagged: false,
            selected: false,
        }
    }

    pub fn new_empty(position: Position) -> Self {
        Self::new(TileKind::Empty, vec![], position)
    }

    pub fn new_safe(position: Position, neighbours: Vec<Tile>) -> Self {
        Self::new(
            TileKind::Safe(neighbours.len().try_into().unwrap()),
            neighbours,
            position,
        )
    }

    pub fn new_bomb(position: Position) -> Self {
        Self::new(TileKind::Bomb, vec![], position)
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

    pub fn is_empty(&self) -> bool {
        self.kind == TileKind::Empty
    }

    pub fn is_safe(&self) -> bool {
        match self.kind {
            TileKind::Safe(_) => true,
            _ => false,
        }
    }
    pub fn is_bomb(&self) -> bool {
        self.kind == TileKind::Bomb
    }

    pub fn repr(&self) -> String {
        match self.flagged {
            true => format!("{}{}?", color::Bg(color::Black), color::Fg(color::White)),
            _ => match self.revealed {
                false => format!("{}{}·", color::Bg(color::Black), color::Fg(color::White)),
                true => match self.kind {
                    TileKind::Bomb => {
                        format!("{}{}◆", color::Bg(color::Black), color::Fg(color::Red))
                    }
                    TileKind::Empty => format!("{} ", color::Bg(color::Black)),
                    TileKind::Safe(bombs) => format!(
                        "{}{}{}",
                        color::Bg(color::Black),
                        match bombs {
                            1 => color::Fg(color::Blue).to_string(),
                            2 => color::Fg(color::LightGreen).to_string(),
                            3 => color::Fg(color::Red).to_string(),
                            4 => color::Fg(color::Magenta).to_string(),
                            5 => color::Fg(color::LightYellow).to_string(),
                            6 => color::Fg(color::LightCyan).to_string(),
                            7 => color::Fg(color::LightBlack).to_string(),
                            8 => color::Fg(color::LightBlue).to_string(),
                            _ => "".to_string(),
                        },
                        bombs
                    ),
                },
            },
        }
    }

    pub fn padded_repr(&self) -> String {
        format!(" {} ", self.repr())
    }
}
