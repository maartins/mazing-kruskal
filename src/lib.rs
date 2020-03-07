use std::error::Error;
use std::fmt;
use rand::Rng;

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
    pub id: String,
}

impl Cell {
    fn new(x: usize, y: usize, cell_type: u8, id: String) -> Cell {
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
        if self.id != "z".to_string() {
            write!(f, "{}", self.id)
        } else {
            write!(f, "#")
        }
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
    let mut map = vec![vec![Cell::new(0, 0, GROUND, "z".to_string()); config.size]; config.size];

    for x in 0..config.size {
        for y in 0..config.size {
            map[x][y].x = x;
            map[x][y].y = y;

            if x & 1 == 1 || y & 1 == 1 {
                map[x][y].cell_type = WALL;
                map[x][y].id = "z".to_string();
            } else {
                map[x][y].cell_type = GROUND;
                map[x][y].id = "a".to_string();
            }
        }
    }

    for (_, row) in map.iter().enumerate() {
        for (_, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }

    Ok(())
}
