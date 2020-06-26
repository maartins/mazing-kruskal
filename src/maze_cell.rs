use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub pos: Pos,
    pub cell_type: u8,
    pub id: usize,
    pub neighbours: Neighbours,
    pub parent: Option<Box<Cell>>
}

impl  Cell {
    pub fn new(x: isize, y: isize, cell_type: u8, id: usize) -> Cell {
        Cell {
            pos: Pos{x , y},
            cell_type,
            id,
            neighbours: Neighbours::new(),
            parent: None
        }
    }

    pub fn get_top_pos(&self) -> &Pos {
        self.neighbours.top.as_ref().unwrap()
    }

    pub fn get_bot_pos(&self) -> &Pos {
        self.neighbours.bot.as_ref().unwrap()
    }

    pub fn get_left_pos(&self) -> &Pos {
        self.neighbours.left.as_ref().unwrap()
    }

    pub fn get_right_pos(&self) -> &Pos {
        self.neighbours.right.as_ref().unwrap()
    }

    pub fn is_verticaly_bound(&self) -> bool {
        self.neighbours.top != None && self.neighbours.bot != None
    }

    pub fn is_horizontaly_bound(&self) -> bool {
        self.neighbours.left != None && self.neighbours.right != None
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cell_type)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pos {
    pub x: isize,
    pub y: isize
}

#[derive(Clone, Debug, PartialEq)]
pub struct Neighbours {
    pub top: Option<Pos>,
    pub bot: Option<Pos>,
    pub left: Option<Pos>,
    pub right: Option<Pos>
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
}
