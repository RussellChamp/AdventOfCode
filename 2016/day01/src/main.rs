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
    let mut state = (1, 0);
    let mut positions: Vec<(i32, i32)> = vec![(0, 0)];

    for item in input.trim().split(", ") {
        let (dir, dist) = item.split_at(1);
        let dir: String = dir.to_string();
        let dist: i32 = dist.parse::<i32>().unwrap();

        state = turn(&state, &dir);
        let pos = step(&state, positions.last().unwrap(), dist);
        print!("{}: {}{} -> ({}, {}), ", positions.len(), dist, dir, pos.0, pos.1);
        match positions.iter().position(|&p| p == pos) {
            Some(v) => {
                println!("Part 2: Stopped twice at ({}, {}) on element {}, {} away from the start", pos.0, pos.1, v, pos.0.abs() + pos.1.abs());
                break;
            },
            None => {
                //println!("Did not match {} locations", positions.len());
                positions.push(pos);
            },
        }
    }
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    //part_1(&input);
    part_2(&input);
}