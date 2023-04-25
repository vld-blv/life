extern crate rand;
extern crate termion;
use std::{env, thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::clear;
use termion::color;

const WORLD_SIZE: usize = 75;
const WORLD_SIZE_INDEX: usize = WORLD_SIZE - 1;
const GENERATIONS_COUNT: u16 = 100;

fn census(world: [[u8; WORLD_SIZE]; WORLD_SIZE]) -> u16 {
    let mut count = 0;

    for i in 0..WORLD_SIZE_INDEX {
        for j in 0..WORLD_SIZE_INDEX {
            if world[i][j] == 1 {
                count += 1
            }
        }
    }

    count
}

fn generation(_world: [[u8; WORLD_SIZE]; WORLD_SIZE]) -> [[u8; WORLD_SIZE]; WORLD_SIZE] {
    let mut new_world = [[0u8; WORLD_SIZE]; WORLD_SIZE];

    for i in 0..WORLD_SIZE_INDEX {
        for j in 0..WORLD_SIZE_INDEX {
            let mut count = 0;

            if i > 0 {
                count += _world[i-1][j];
            }
            if i > 0 && j > 0 {
                count += _world[i-1][j-1];
            }
            if i > 0 && j < WORLD_SIZE_INDEX {
                count += _world[i-1][j+1];
            }
            if i < WORLD_SIZE_INDEX && j > 0 {
                count += _world[i+1][j-1];
            }
            if i < WORLD_SIZE_INDEX {
                count += _world[i+1][j];
            }
            if i < WORLD_SIZE_INDEX && j < WORLD_SIZE_INDEX {
                count += _world[i+1][j+1];
            }
            if j > 0 {
                count += _world[i][j-1];
            }
            if j < WORLD_SIZE_INDEX {
                count = _world[i][j+1];
            }

            new_world[i][j] = 0;

            if (count <2) && (_world[i][j] == 1) {
                new_world[i][j] = 0;
            }
            if _world[i][j] == 1 && (count == 2 || count == 3) {
                new_world[i][j] = 1;
            }
            if (_world[i][j] == 0) && (count == 3) {
                new_world[i][j] = 1;
            }
        }
    }

    new_world
}

fn populate_from_file(filename: String) -> [[u8; WORLD_SIZE]; WORLD_SIZE] {
    let mut new_world = [[0u8; WORLD_SIZE]; WORLD_SIZE];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();
        let left = words.next().unwrap();
        let right = words.next().unwrap();
        pairs.push((left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap()));
    }

    for i in 0..WORLD_SIZE_INDEX {
        for j in 0..WORLD_SIZE_INDEX {
            new_world[i][j] = 0;
        }
    }

    for (x, y) in pairs {
        new_world[x][y] = 1;
    }

    new_world
}

fn display_world(world: [[u8; WORLD_SIZE]; WORLD_SIZE]) {
    for i in 0..WORLD_SIZE_INDEX {
        for j in 0..WORLD_SIZE_INDEX {
            if world[i][j] == 1 {
                print!("{red}*", red = color::Fg(color::Red))
            } else {
                print!(" ")
            }
        }

        println!("")
    }
}

fn main() {
    let mut world = [[0u8; WORLD_SIZE]; WORLD_SIZE];
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        for i in 0..WORLD_SIZE_INDEX {
            for j in 0..WORLD_SIZE_INDEX {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            } 
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename)
    }

    println!("Population at {} is {}", generations, census(world));
    for _gens in 0..GENERATIONS_COUNT {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("{}", clear::All);
        display_world(world);
        println!("{blue}Population at generation {g} is {c}", blue = color::Fg(color::Blue), g = generations, c = census(world));
        thread::sleep(time::Duration::from_secs(2))
    }
}