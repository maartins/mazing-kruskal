use std::fmt;

#[derive(Clone, Debug)]
pub struct Cell {
    pub pos: Pos,
    pub cell_type: u8,
    pub id: usize,
    pub neighbours: Neighbours
}

impl Cell {
    pub fn new(x: isize, y: isize, cell_type: u8, id: usize) -> Cell {
        Cell {
            pos: Pos{x , y},
            cell_type,
            id,
            neighbours: Neighbours::new()
        }
    }

    pub fn get_top_pos(&self) -> Pos {
        Pos{x: self.pos.x, y: self.pos.y - 1}
    }

    pub fn get_bottom_pos(&self) -> Pos {
        Pos{x: self.pos.x, y: self.pos.y + 1}
    }

    pub fn get_right_pos(&self) -> Pos {
        Pos{x: self.pos.x + 1, y: self.pos.y}
    }

    pub fn get_left_pos(&self) -> Pos {
        Pos{x: self.pos.x - 1, y: self.pos.y}
    }

    pub fn is_in_bounds(&self, map_size: isize) -> bool {
        (self.pos.y >= 0 && self.pos.y <= map_size - 1) && (self.pos.x >= 0 && self.pos.x <= map_size - 1)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cell_type)
    }
}

#[derive(Clone, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize
}

#[derive(Clone, Debug)]
pub struct Neighbours {
    pub top: Option<Box<Cell>>,
    pub bot: Option<Box<Cell>>,
    pub left: Option<Box<Cell>>,
    pub right: Option<Box<Cell>>
}

impl Neighbours {
    pub fn new() -> Neighbours {
        Neighbours {
            top: None,
            bot: None,
            left: None,
            right: None
        }
    }

    pub fn count(&self) -> u8 {
        return 9;
    }
}