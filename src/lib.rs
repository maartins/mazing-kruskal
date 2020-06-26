mod maze_cell;

use std::error::Error;
use rand::seq::SliceRandom;
use std::io;
use std::io::prelude::*;
use std::time::Instant;
use std::fs::OpenOptions;
use std::fs::File;

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
            "p" => Mode::Print,
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
    Verbose,
    Print
}

#[derive(Clone, Debug)]
pub struct Maze {
    pub map: Vec<Vec<maze_cell::Cell>>,
    pub size: usize
}

impl Maze {
    pub fn new(size: usize) -> Maze {
        Maze {
            map: vec![vec![maze_cell::Cell::new(0, 0, GROUND, 0); size]; size],
            size
        }
    }

    pub fn get_cell(&self, pos: &maze_cell::Pos) -> maze_cell::Cell {
        return self.map[pos.y as usize][pos.x as usize].clone();
    }

    pub fn set_cell(&mut self, pos: &maze_cell::Pos, cell: &maze_cell::Cell) {
        self.map[pos.y as usize][pos.x as usize].id = cell.id;
        self.map[pos.y as usize][pos.x as usize].cell_type = cell.cell_type;
    }

    pub fn join_cells(&mut self, wall: &maze_cell::Cell) {
        if wall.is_verticaly_bound() {
            let top = &self.get_cell(wall.get_top_pos());
            let bottom = &self.get_cell(wall.get_bot_pos());
            self.join_sides(top, wall, bottom);
        }
        if wall.is_horizontaly_bound() {
            let left = &self.get_cell(wall.get_left_pos());
            let right = &self.get_cell(wall.get_right_pos());
            self.join_sides(left, wall, right);
        }
    }

    /*
    *  ---->
    *  side | center | opposite
    *  
    *  | side
    *  | ---
    *  ▼ center
    *    ---
    *    opposite
    */
    fn join_sides(&mut self, side: &maze_cell::Cell, center: &maze_cell::Cell, opposite: &maze_cell::Cell) {
        if opposite.id != side.id && opposite.cell_type != WALL && side.cell_type != WALL {
            self.set_cell(&center.pos, &side);
            self.set_cell(&opposite.pos, &side);

            for cell in self.map.iter_mut().flat_map(|r| r.iter_mut()) {
                if cell.id == opposite.id {
                    cell.id = side.id;
                }
            }
        }
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    match config.mode {
        Mode::Print => {
            File::create("mazes.txt")?;
        }
        _ => {}
    }

    let start_time = Instant::now();
    for _ in 0..config.count {
        let mut maze = Maze::new(config.size);
        let mut walls = Vec::new();

        let mut counter: usize = 1;
        for y in 0..maze.size {
            for x in 0..maze.size {
                maze.map[y][x].pos = maze_cell::Pos{x: x as isize, y: y as isize};

                /*
                *  0 1 2
                *  3 4 5
                *  6 7 8
                */
                for dir in 0..8 {
                    if dir == 1 || dir == 3 || dir == 5 || dir == 7 {
                        let n_pos = maze_cell::Pos{x: x as isize + (dir % 3) - 1, y: y as isize + (dir / 3) - 1};
        
                        if n_pos.x >= 0 && n_pos.x < maze.size as isize && n_pos.y >= 0 && n_pos.y < maze.size as isize {
                            match dir {
                                1 => { maze.map[y][x].neighbours.top = Some(n_pos); },
                                7 => { maze.map[y][x].neighbours.bot = Some(n_pos); },
                                3 => { maze.map[y][x].neighbours.left = Some(n_pos); },
                                5 => { maze.map[y][x].neighbours.right = Some(n_pos); },
                                _ => {}
                            }
                        }
                    } 
                }
                
                if x & 1 == 1 || y & 1 == 1 {
                    maze.map[y][x].cell_type = WALL;
                    walls.push(maze.map[y][x].clone());
                } else {
                    maze.map[y][x].id = counter;
                }

                counter += 1;
            }
        }

        if let Mode::ComputerStep = config.mode {
            while walls.len() != 0 {
                walls.shuffle(&mut rand::thread_rng());
                maze.join_cells(&walls.pop().unwrap());
                for (_, row) in maze.map.iter().enumerate() {
                    for (_, col) in row.iter().enumerate() {
                        print!("{}", col);
                    }
                    println!();
                }
                pause();
            }
        } else {
            while walls.len() != 0 {
                walls.shuffle(&mut rand::thread_rng());
                maze.join_cells(&walls.pop().unwrap());
            }

            match config.mode {
                Mode::Verbose => {
                    for (_, row) in maze.map.iter().enumerate() {
                        for (_, col) in row.iter().enumerate() {
                            print!("{}", col);
                        }
                        println!();
                    }
                },
                Mode::Computer => {
                    for (_, row) in maze.map.iter().enumerate() {
                        for (_, col) in row.iter().enumerate() {
                            print!("{}", col);
                        }
                    }
                    println!();
                },
                Mode::ComputerStep => {},
                Mode::Print => {
                    let mut file = OpenOptions::new().write(true).append(true).open("mazes.txt").unwrap();
            
                    for (_, row) in maze.map.iter().enumerate() {
                        for (_, col) in row.iter().enumerate() {
                            write!(file, "{}", col.cell_type)?;
                        }
                    }
                    writeln!(file, "")?;
                }
            }
        }
    }
    println!("Mazes generated: {}", config.count);
    println!("Total time run: {}ms", start_time.elapsed().as_millis());

    Ok(())
}
