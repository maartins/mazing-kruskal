use std::error::Error;
use std::fmt;
use rand::Rng;
use rand::seq::SliceRandom;

const WALL: u8 = 1;
const GROUND: u8 = 0;

#[derive(Debug)]
pub struct Config {
    pub size: usize,
    pub count: usize,
    pub verbose: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let size = args[1].clone().parse().unwrap();
        let count = args[2].clone().parse().unwrap();
        let verbose = args[3].clone();

        if size <= 4 {
            return Err("size must be greater than 4");
        }

        if count <= 0 {
            return Err("must specify positive count");
        }

        Ok(Config {
            size,
            count,
            verbose
        })
    }
}

#[derive(Clone, Debug)]
struct Cell {
    pub x: usize,
    pub y: usize,
    pub cell_type: u8,
    pub id: u8,
}

impl Cell {
    fn new(x: usize, y: usize, cell_type: u8, id: u8) -> Cell {
        Cell {
            x,
            y,
            cell_type,
            id
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.id != 0 {
            write!(f, "_")
        } else {
            write!(f, "#")
        }
    }
}

fn join_cells(wall: &Cell, map: &mut Vec<Vec<Cell>>) {
    let mut top: Option<u8> = None;
    let mut bottom: Option<u8> = None;
    let mut right: Option<u8> = None;
    let mut left: Option<u8> = None;

    if wall.y as i8 - 1 >= 0 {
        top = Some(1);
    }
    if wall.y + 1 <= map.len() - 1 {
        bottom = Some(1);
    }
    if wall.x + 1 <= map.len() - 1 {
        right = Some(1);
    }
    if wall.x as i8 - 1 >= 0 {
        left = Some(1);
    }

    if let Some(1) = top {
        if let Some(1) = bottom {
            let new_id = map[wall.y - 1][wall.x].id;
            map[wall.y][wall.x].id = new_id;
            map[wall.y + 1][wall.x].id = new_id;
        }
    }
    if let Some(1) = right {
        if let Some(1) = left {
            let new_id = map[wall.y][wall.x - 1].id;
            map[wall.y][wall.x].id = new_id;
            map[wall.y][wall.x + 1].id = new_id;
        }
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
    let mut map = vec![vec![Cell::new(0, 0, GROUND, 0); config.size]; config.size];
    let mut walls = Vec::new();

    let mut counter = 1;
    for y in 0..config.size {
        for x in 0..config.size {
            map[y][x].x = x;
            map[y][x].y = y;

            if x & 1 == 1 || y & 1 == 1 {
                map[y][x].cell_type = WALL;
            } else {
                map[y][x].cell_type = GROUND;
                map[y][x].id = counter;
            }

            counter += 1;
        }
    }

    for row in map.iter() {
        for cell in row.iter() {
            if cell.cell_type == WALL {
                walls.push(cell.clone());
            }
        }
    }

    while walls.len() != 0 {
        walls.shuffle(&mut rand::thread_rng());
        join_cells(&walls.pop().unwrap(), &mut map);
    }

    for (_, row) in map.iter().enumerate() {
        for (_, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }

    Ok(())
}
