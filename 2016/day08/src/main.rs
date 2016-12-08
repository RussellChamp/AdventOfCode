extern crate regex;

use regex::Regex;
use regex::RegexSet;
use std::io::Read;
use std::fs::File;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 6;

fn part_1_2(input: &String) {
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT]; //grid of bools, 50 across by 6 down

    let set = RegexSet::new(&[
        r"rect (\d+)x(\d+)",
        r"rotate row y=(\d+) by (\d+)",
        r"rotate column x=(\d+) by (\d+)",
    ]).unwrap();

    let re_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let re_rot_r = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let re_rot_c = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    for line in input.lines() {
        let match_set: Vec<_> = set.matches(line.trim()).into_iter().collect();
        match match_set[0] {
            0 /*rect*/ => {
                let cap = re_rect.captures(line).unwrap();
                let width = cap.at(1).unwrap().parse().unwrap();
                let height = cap.at(2).unwrap().parse().unwrap();
                //println!("rect ({}, {})", width, height);
                for row in 0..height {
                    for col in 0..width {
                        grid[row][col] = true;
                    }
                }
            },
            1 /*rotate row*/ => {
                let cap = re_rot_r.captures(line).unwrap();
                let y: usize = cap.at(1).unwrap().parse().unwrap();
                let by: usize = cap.at(2).unwrap().parse().unwrap();
                //println!("row ({} by {})", y, by);
                //copy all the values off from this row, then we'll shift the values
                let mut from = [false; GRID_WIDTH];
                for col in 0..GRID_WIDTH {
                    from[col] = grid[y][col];
                    grid[y][col] = false;
                }
                for col in 0..GRID_WIDTH {
                    let new_col: usize = (col + by) % GRID_WIDTH;
                    grid[y][new_col] = from[col];
                }
            },
            2 /*rotate column*/=> {
                let cap = re_rot_c.captures(line).unwrap();
                let x: usize = cap.at(1).unwrap().parse().unwrap();
                let by: usize = cap.at(2).unwrap().parse().unwrap();
                //println!("col ({} by {})", x, by);
                //copy all the values off from this column, then we'll shift the values
                let mut from = [false; GRID_HEIGHT];
                for row in 0..GRID_HEIGHT {
                    from[row] = grid[row][x];
                    grid[row][x] = false;
                }
                for row in 0..GRID_HEIGHT {
                    let new_row: usize = (row + by) % GRID_HEIGHT;
                    grid[new_row][x] = from[row];
                }
            },
            _ /*no match*/=> {}, //what happened?
        }
    }
    let mut sum = 0;
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            match grid[row][col] {
                true => { print!("#"); sum = sum + 1; },
                false => print!("."),
            }
        }
        println!("");
    }
    println!("Total pixels: {}", sum);
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    part_1_2(&input);
}
