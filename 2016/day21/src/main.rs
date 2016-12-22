extern crate regex;

use std::io::Read;
use std::fs::File;
use regex::Regex;
use std::cmp;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left
}

#[derive(Debug)]
enum Step {
    SwapPos { from: usize, to: usize },
    SwapLet { from: char, to: char },
    RotatePos { dir: Direction, value: usize },
    RotateLet { from: char },
    Reverse { from: usize, to: usize },
    MovePos { from: usize, to: usize },
}

fn swap_position(passcode: String, from: usize, to: usize) -> String {
    let (low, high) = (cmp::min(from, to), cmp::max(from, to));
    let mut new_passcode = passcode[0..low].to_string();

    let low_char = passcode.chars().clone().nth(low).unwrap();
    let high_char = passcode.chars().clone().nth(high).unwrap();

    new_passcode.push(high_char);
    if low + 1 < high {
        new_passcode.push_str(&passcode[low+1..high]);
    }
    new_passcode.push(low_char);
    if high < passcode.len() {
        new_passcode.push_str(&passcode[high+1..]);
    }
    //println!("{} -> {}", passcode, new_passcode);
    new_passcode
}

fn swap_letter(passcode: String, from: char, to: char) -> String {
    let pos_from = passcode.chars().position(|c| c == from).unwrap();
    let pos_to = passcode.chars().position(|c| c == to).unwrap();
    swap_position(passcode, pos_from, pos_to)
}

fn rotate_position(passcode: String, dir: Direction, value: usize) -> String {
    let mut new_passcode = String::new();
    let shift = value % passcode.len(); //unless we shift by too much
    match dir {
        Direction::Right => {
            //the last 'value' characters and then the rest
            let split_idx = passcode.len()-shift;
            new_passcode.push_str(&passcode[split_idx..]);
            new_passcode.push_str(&passcode[..split_idx]);
        },
        Direction::Left => {
            new_passcode.push_str(&passcode[shift..]);
            new_passcode.push_str(&passcode[..shift]);
        },
    }
    //println!("{} -({} {:?})-> {}", passcode, value, dir, new_passcode);
    new_passcode
}

fn rotate_letter(passcode: String, from: char) -> String {
    //rotate to the right based on 1 + idx of letter + 1 if idx > 3
    let pos = passcode.chars().position(|c| c == from).unwrap();
    if pos > 3 {
        return rotate_position(passcode, Direction::Right, 2 + pos);
    } else {
        return rotate_position(passcode, Direction::Right, 1 + pos);
    }
}

fn un_rotate_letter(passcode: String, from: char) -> String {
    //find the letter
    passcode
}

fn reverse(passcode: String, from: usize, to: usize) -> String {
    let mut new_passcode = String::from(&passcode[..from]);
    for c in passcode[from..to+1].chars().rev() {
        new_passcode.push(c);
    }
    new_passcode.push_str(&passcode[to+1..]);
    //println!("{} -> {}", passcode, new_passcode);
    new_passcode
}

fn move_position(passcode: String, from: usize, to: usize) -> String {
    let mut new_passcode = String::new();
    if from < to {
        // [..from][from+1..to+1][from][to.1..]
        new_passcode.push_str(&passcode[..from]);
        new_passcode.push_str(&passcode[from+1..to+1]);
        new_passcode.push_str(&passcode[from..from+1]);
        new_passcode.push_str(&passcode[to+1..]);
    } else {
        // [..to][from][to..from][from+1..]
        new_passcode.push_str(&passcode[..to]);
        new_passcode.push_str(&passcode[from..from+1]);
        new_passcode.push_str(&passcode[to..from]);
        new_passcode.push_str(&passcode[from+1..]);
    }
    //println!("{} -> {}", passcode, new_passcode);
    new_passcode
}

fn main() {
    let re_swap_pos = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let re_swap_let = Regex::new(r"swap letter ([:alpha:]) with letter ([:alpha:])").unwrap();
    let re_rotate_pos = Regex::new(r"rotate (left|right) (\d+) step[s]{0,1}").unwrap();
    let re_rotate_let = Regex::new(r"rotate based on position of letter ([:alpha:])").unwrap();
    let re_reverse = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let re_move_pos = Regex::new(r"move position (\d+) to position (\d)").unwrap();

    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    let mut steps: Vec<Step> = Vec::new(); //translate all the instructions

    for line in input.lines() {
        if re_swap_pos.is_match(line) {
            let cap = re_swap_pos.captures(line).unwrap();
            steps.push(Step::SwapPos { from: cap.at(1).unwrap().parse().unwrap(), to: cap.at(2).unwrap().parse().unwrap() });
        }
        else if re_swap_let.is_match(line) {
            let cap = re_swap_let.captures(line).unwrap();
            steps.push(Step::SwapLet { from: cap.at(1).unwrap().chars().next().unwrap(), to: cap.at(2).unwrap().chars().next().unwrap() });
        }
        else if re_rotate_pos.is_match(line) {
            let cap = re_rotate_pos.captures(line).unwrap();
            let direction = match cap.at(1).unwrap() {
                "left" => Direction::Left,
                _ => Direction::Right,
            };
            let value = cap.at(2).unwrap().parse().unwrap();

            steps.push(Step::RotatePos { dir: direction, value: value });
        }
        else if re_rotate_let.is_match(line) {
            let cap = re_rotate_let.captures(line).unwrap();
            steps.push(Step::RotateLet { from: cap.at(1).unwrap().chars().next().unwrap() });
        }
        else if re_reverse.is_match(line) {
            let cap = re_reverse.captures(line).unwrap();
            steps.push(Step::Reverse { from: cap.at(1).unwrap().parse().unwrap(), to: cap.at(2).unwrap().parse().unwrap() });
        }
        else if re_move_pos.is_match(line) {
            let cap = re_move_pos.captures(line).unwrap();
            steps.push(Step::MovePos { from: cap.at(1).unwrap().parse().unwrap(), to: cap.at(2).unwrap().parse().unwrap() });
        }
        else {
            panic!("Unknown pattern! {}", line);
        }
    }

    // let mut passcode = String::from("abcdefgh");

    // println!("All Steps: ");
    // for step in steps {
    //     passcode = match step {
    //         Step::SwapPos { from, to } => { swap_position(passcode, from, to) },
    //         Step::SwapLet { from, to } => { swap_letter(passcode, from, to) },
    //         Step::RotatePos { dir, value } => { rotate_position(passcode, dir, value) },
    //         Step::RotateLet { from } => { rotate_letter(passcode, from) },
    //         Step::Reverse { from, to } => { reverse(passcode, from, to) },
    //         Step::MovePos { from, to } => { move_position(passcode, from, to) },
    //     };
    //     //println!("{} -> ", passcode);
    //     //println!("{:?}", step);
    // }
    // println!("Part 1: Passcode is {}", passcode);

    let mut existing = String::from("fbgdceah");
    for step in steps.iter().rev() {
        existing = match *step {
            Step::SwapPos { from, to } => { swap_position(existing, from, to) }, //unaffected
            Step::SwapLet { from, to } => { swap_letter(existing, from, to) }, //unaffected
            Step::RotatePos { dir, value } => {
                let undir = match dir {
                    Direction::Right => Direction::Left,
                    Direction::Left => Direction::Right
                };
                rotate_position(existing, undir, value)
                }, //mostly unaffected
            Step::RotateLet { from } => { un_rotate_letter(existing, from) },
            Step::Reverse { from, to } => { reverse(existing, from, to) }, //unaffected
            Step::MovePos { from, to } => { move_position(existing, to, from) }, //mostly unaffected
        }
    }
    println!("Part 2: Decyphered passcode is {}", existing);

}

#[test]
fn test_swap_position() {
    assert_eq!(swap_position("abcd".to_string(), 0, 2), "cbad");
}

#[test]
fn test_swap_letter() {
    assert_eq!(swap_letter("abcd".to_string(), 'b', 'd'), "adcb");
}

#[test]
fn test_rotate_position() {
    assert_eq!(rotate_position("abcd".to_string(), Direction::Right, 2), "cdab");
    assert_eq!(rotate_position("abcd".to_string(), Direction::Left, 1), "bcda");
    assert_eq!(rotate_position("abcd".to_string(), Direction::Right, 8), "abcd");
}

#[test]
fn test_rotate_letter() {
    assert_eq!(rotate_letter("abcd".to_string(), 'b'), "cdab");
    assert_eq!(rotate_letter("abcd".to_string(), 'b'), rotate_position("abcd".to_string(), Direction::Right, 2));
    assert_eq!(rotate_letter("abcdefgh".to_string(), 'f'), "bcdefgha");
}

#[test]
fn test_reverse() {
    assert_eq!(reverse("abcd".to_string(), 1, 2), "acbd");
}

#[test]
fn test_move_position() {
    assert_eq!(move_position("abcdefg".to_string(), 1, 4), "acdebfg");
    assert_eq!(move_position("abcdefg".to_string(), 4, 1), "aebcdfg");
}

#[test]
fn test_undo_move_position() {
    assert_eq!(move_position(move_position("abcdefg".to_string(), 1, 4), 4, 1), "abcdefg");
    assert_eq!(move_position(move_position("abcdefg".to_string(), 4, 1), 1, 4), "abcdefg");
}

#[test]
fn test_un_rotate_letter() {
    assert_eq!(un_rotate_letter("abcdefg".to_string(), 'c'), "abcdefg");
    assert_eq!(un_rotate_letter(rotate_letter("abcdefg".to_string(), 'c'), 'c'), "abcdefg");
}