use std::io::Read;
use std::fs::File;

fn part_1(input: &String) {
    let total = input.lines()
        .map(|l| l
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .filter(|v| v.iter().fold(0, |sum, x| sum + x) > 2 * v.iter().max().unwrap() )
        .collect::<Vec<_>>()
        .len();
    println!("Part 1: Total was {}", total);
}

fn part_2(input: &String) {
    ;
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    part_1(&input);
}
