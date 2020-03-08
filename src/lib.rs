use std::error::Error;
use std::fmt;
use rand::seq::SliceRandom;
use std::io;
use std::io::prelude::*;

const WALL: u8 = 1;
const GROUND: u8 = 0;

#[derive(Debug)]
pub struct Config {
    pub size: usize,
    pub count: usize,
    pub mode: Mode
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let size = args[1].clone().parse().unwrap();
        let count = args[2].clone().parse().unwrap();
        let mode = match args[3].as_str() {
            "c" => Mode::Computer,
            "cs" => Mode::ComputerStep,
            "v" => Mode::Verbose,
            _ => return Err("not a valid mode")
        };

        if size <= 4 {
            return Err("size must be greater than 4");
        }

        if size & 1 != 1 {
            return Err("size must be odd number");
        }

        if count <= 0 {
            return Err("must specify positive count");
        }

        Ok(Config {
            size,
            count,
            mode
        })
    }
}

#[derive(Debug)]
pub enum Mode {
    Computer,
    ComputerStep,
    Verbose
}

#[derive(Clone, Debug)]
struct Cell {
    pub pos: Pos,
    pub cell_type: u8,
    pub id: usize,
}

impl Cell {
    fn new(x: isize, y: isize, cell_type: u8, id: usize) -> Cell {
        let pos = Pos{x , y};

        Cell {
            pos,
            cell_type,
            id
        }
    }

    fn get_top_pos(&self) -> Pos {
        Pos{x: self.pos.x, y: self.pos.y - 1}
    }

    fn get_bottom_pos(&self) -> Pos {
        Pos{x: self.pos.x, y: self.pos.y + 1}
    }

    fn get_right_pos(&self) -> Pos {
        Pos{x: self.pos.x + 1, y: self.pos.y}
    }

    fn get_left_pos(&self) -> Pos {
        Pos{x: self.pos.x - 1, y: self.pos.y}
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cell_type)
    }
}

#[derive(Clone, Debug)]
struct Pos {
    pub x: isize,
    pub y: isize
}

fn is_in_bounds(cell: &Pos, length: isize) -> bool {
    (cell.y >= 0 && cell.y <= length - 1) && (cell.x >= 0 && cell.x <= length - 1)
}

fn join_cells(wall: &Cell, map: &mut Vec<Vec<Cell>>) {
    let top_pos = wall.get_top_pos();
    let bottom_pos =  wall.get_bottom_pos();
    let right_pos = wall.get_right_pos();
    let left_pos = wall.get_left_pos();

    join_sides(&top_pos, &wall.pos, &bottom_pos, map);
    join_sides(&left_pos, &wall.pos, &right_pos, map);
}

fn join_sides(side: &Pos, center: &Pos, opposite: &Pos, map: &mut Vec<Vec<Cell>>) {
    if is_in_bounds(&side, map.len() as isize) && is_in_bounds(&opposite, map.len() as isize) {
        let new_id = map[side.y as usize][side.x as usize].id;
        let old_id = map[opposite.y as usize][opposite.x as usize].id;
        if old_id != new_id && old_id != 0 && new_id != 0 {
            map[center.y as usize][center.x as usize].id = new_id;
            map[center.y as usize][center.x as usize].cell_type = GROUND;
            map[opposite.y as usize][opposite.x as usize].id = new_id;

            for row in map.iter_mut() {
                for cell in row.iter_mut() {
                    if cell.id == old_id {
                        cell.id = new_id;
                    }
                }
            }
        }
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let mut map = vec![vec![Cell::new(0, 0, GROUND, 0); config.size]; config.size];
    let mut walls = Vec::new();

    let mut counter: usize = 1;
    for y in 0..map.len() {
        for x in 0..map.len() {
            map[y][x].pos.x = x as isize;
            map[y][x].pos.y = y as isize;

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

    match config.mode {
        Mode::Verbose => {
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
        },
        Mode::ComputerStep => {
            while walls.len() != 0 {
                walls.shuffle(&mut rand::thread_rng());
                join_cells(&walls.pop().unwrap(), &mut map);
                for (_, row) in map.iter().enumerate() {
                    for (_, col) in row.iter().enumerate() {
                        print!("{}", col);
                    }
                    println!()
                }
                pause();
            }
        },
        Mode::Computer => {
            while walls.len() != 0 {
                walls.shuffle(&mut rand::thread_rng());
                join_cells(&walls.pop().unwrap(), &mut map);
            }
    
            for (_, row) in map.iter().enumerate() {
                for (_, col) in row.iter().enumerate() {
                    print!("{}", col);
                }
            }
        }
    }

    Ok(())
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}