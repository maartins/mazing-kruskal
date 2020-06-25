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

fn is_in_bounds(cell: &maze_cell::Pos, length: isize) -> bool {
    (cell.y >= 0 && cell.y <= length - 1) && (cell.x >= 0 && cell.x <= length - 1)
}

fn join_cells(wall: &maze_cell::Cell, map: &mut Vec<Vec<maze_cell::Cell>>) {
    let top_pos = wall.get_top_pos();
    let bottom_pos =  wall.get_bottom_pos();
    let right_pos = wall.get_right_pos();
    let left_pos = wall.get_left_pos();

    join_sides(&top_pos, &wall.pos, &bottom_pos, map);
    join_sides(&left_pos, &wall.pos, &right_pos, map);
}

fn join_sides(side: &maze_cell::Pos, center: &maze_cell::Pos, opposite: &maze_cell::Pos, map: &mut Vec<Vec<maze_cell::Cell>>) {
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
        let mut map = vec![vec![maze_cell::Cell::new(0, 0, GROUND, 0); config.size]; config.size];
        let mut walls = Vec::new();

        let mut counter: usize = 1;
        for y in 0..map.len() {
            for x in 0..map.len() {
                map[y][x].pos = maze_cell::Pos{x: x as isize, y: y as isize};

                if x & 1 == 1 || y & 1 == 1 {
                    map[y][x].cell_type = WALL;
                    walls.push(map[y][x].clone());
                } else {
                    map[y][x].id = counter;
                }

                counter += 1;
            }
        }

        if let Mode::ComputerStep = config.mode {
            while walls.len() != 0 {
                walls.shuffle(&mut rand::thread_rng());
                join_cells(&walls.pop().unwrap(), &mut map);
                for (_, row) in map.iter().enumerate() {
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
                join_cells(&walls.pop().unwrap(), &mut map);
            }

            match config.mode {
                Mode::Verbose => {
                    for (_, row) in map.iter().enumerate() {
                        for (_, col) in row.iter().enumerate() {
                            print!("{}", col);
                        }
                        println!();
                    }
                },
                Mode::Computer => {
                    for (_, row) in map.iter().enumerate() {
                        for (_, col) in row.iter().enumerate() {
                            print!("{}", col);
                        }
                    }
                    println!();
                },
                Mode::ComputerStep => {},
                Mode::Print => {
                    let mut file = OpenOptions::new().write(true).append(true).open("mazes.txt").unwrap();
            
                    for (_, row) in map.iter().enumerate() {
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
