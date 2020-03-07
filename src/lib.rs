use std::error::Error;
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

pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
    let mut map = vec![vec![GROUND; config.size]; config.size];

    for (_, row) in map.iter().enumerate() {
        for (_, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }

    Ok(())
}
