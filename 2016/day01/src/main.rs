use std::io::Read;
use std::fs::File;

fn turn(state: &(i32, i32), direction: &String) -> (i32, i32) {
    match direction.as_ref() {
        "R" => { // clockwise motion
            (-1 * state.1, 1 * state.0)
        },
        "L" => { //rotate widdershins
            (1 * state.1, -1 * state.0)
        },
        _ => {
            println!("Invalid direction: {}", direction);
            (state.0, state.1)
        },
    }
}

fn step(state: &(i32, i32), pos: &(i32, i32), dist: i32) -> (i32, i32) {
    (pos.0 + state.0 * dist, pos.1 + state.1 * dist)
}

fn tiny_step(state: &(i32, i32), pos: &(usize, usize)) -> (usize, usize) {
    ((pos.0 as i32 + state.0) as usize, (pos.1 as i32 + state.1) as usize)
}

fn part_1(input: &String) {
    let mut state = (1, 0);
    let mut pos = (0, 0);
    for item in input.trim().split(", ") {
        let (dir, dist) = item.split_at(1);
        let dir: String = dir.to_string();
        let dist: i32 = dist.parse::<i32>().unwrap();

        state = turn(&state, &dir);
        pos = step(&state, &pos, dist);

        //println!("Moved {} {} to arrive at ({}, {})", dist, dir, pos.0, pos.1);
    }
    println!("Part 1: Ended up at ({}, {}), {} away from the start", pos.0, pos.1, pos.0.abs() + pos.1.abs());
}

fn part_2(input: &String) {
    const OFFSET: usize = 500; //we'll make a grid 1000x1000
    let mut grid = [[false; 2*OFFSET]; 2*OFFSET];

    let mut state = (1, 0);
    let mut pos = (OFFSET, OFFSET);
    grid[OFFSET][OFFSET] = true; //we are already at the axis

    'outer: for item in input.trim().split(", ") {
        let (dir, dist) = item.split_at(1);
        let dir: String = dir.to_string();
        let mut dist: i32 = dist.parse::<i32>().unwrap();

        state = turn(&state, &dir);
        while dist > 0 { //take a step
            pos = tiny_step(&state, &pos);
            match grid[pos.0][pos.1] {
                true => {
                    let real_pos = ((pos.0 as i32 - OFFSET as i32), (pos.1 as i32 - OFFSET as i32));
                    println!("Part 2: Returned to ({}, {}), {} away from the start", real_pos.0, real_pos.1, real_pos.0.abs() + real_pos.1.abs());
                    break 'outer;
                },
                false => { grid[pos.0][pos.1] = true; dist = dist - 1 },
            }
        }
    }
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    part_1(&input);
    part_2(&input);
}