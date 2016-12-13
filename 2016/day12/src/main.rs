extern crate regex;

use regex::Regex;
use std::io::Read;
use std::fs::File;

// registers a, b, c, d indexed at 0
// cpy x y copies x (either an integer or the value of a register) into register y.
// inc x increases the value of register x by one.
// dec x decreases the value of register x by one.
// jnz x y jumps to an instruction y away (positive means forward; negative means backward), but only if x is not zero.

#[derive(Debug)]
enum Operator {
    CpyR { from: u8, dest: u8 }, //from register
    CpyV { value: i64, dest: u8}, //from value
    Inc { reg: u8},
    Dec { reg: u8},
    JnzR { reg: u8, dist: i32}, //from register
    JnzV { flag: i64, dist: i32}, //from value
}

fn reg_to_idx(name: &str) -> u8 {
    match name {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => 0,
    }
}

fn main() {
    let mut ops: Vec<Operator> = Vec::new();

    //**** Read in the file input and convert it to a Vec of Operations ****//
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    let re_cpy = Regex::new(r"^cpy (.*) (.*)$").unwrap();
    let re_inc = Regex::new(r"^inc (.*)$").unwrap();
    let re_dec = Regex::new(r"^dec (.*)$").unwrap();
    let re_jnz = Regex::new(r"^jnz (.*) (.*)$").unwrap();

    for line in input.lines() {
        if re_cpy.is_match(line) {
            let cap = re_cpy.captures(line).unwrap();
            let reg2 = reg_to_idx(cap.at(2).unwrap());
            match cap.at(1).unwrap().parse::<i64>() {
                Ok(v) => ops.push(Operator::CpyV { value: v, dest: reg2 }),
                Err(e) => ops.push(Operator::CpyR { from: reg_to_idx(cap.at(1).unwrap()), dest: reg2 }),
            };
        }
        else if re_inc.is_match(line) {
            let cap = re_inc.captures(line).unwrap();
            let value = cap.at(1).unwrap();
            ops.push(Operator::Inc { reg: reg_to_idx(value) });
        }
        else if re_dec.is_match(line) {
            let cap = re_dec.captures(line).unwrap();
            let value = cap.at(1).unwrap();
            ops.push(Operator::Dec { reg: reg_to_idx(value) });
        }
        else if re_jnz.is_match(line) {
            let cap = re_jnz.captures(line).unwrap();
            let val2 = cap.at(2).unwrap().parse::<i32>().unwrap();
            match cap.at(1).unwrap().parse::<i64>() {
                Ok(v) => ops.push(Operator::JnzV { flag: v, dist: val2 }),
                Err(e) => ops.push(Operator::JnzR { reg: reg_to_idx(cap.at(1).unwrap()), dist: val2 }),
            };
        }
    }
    //println!("{:?}", ops);
    println!("Converted input values");

    //**** loop through the Operations vector ****//
    //part1 started registers with all 0s, part 2 sets 'c' to 1
    let mut registers: Vec<i64> = vec![0, 0, 1, 0]; //registers 'a', 'b', 'c', and 'd'
    let mut idx = 0;
    println!("Processing {} operations", ops.len());
    'ops_loop: while idx < ops.len() {
        //print!("Run[{}] {:?}", idx, ops[idx]);
        match ops[idx] {
            Operator::CpyR{ from, dest } => {
                registers[dest as usize] = registers[from as usize];
            },
            Operator::CpyV{ value, dest } => {
                registers[dest as usize] = value;
            },
            Operator::Inc{ reg } => {
                registers[reg as usize] = registers[reg as usize] + 1;
            },
            Operator::Dec{ reg } => {
                registers[reg as usize] = registers[reg as usize] - 1;
            },
            Operator::JnzR{ reg, dist } => {
                if registers[reg as usize] != 0 {
                    idx = (idx as i32 + dist) as usize;
                    continue 'ops_loop;
                }
            },
            Operator::JnzV{ flag, dist } => {
                if flag != 0 {
                    idx = (idx as i32 + dist) as usize;
                    continue 'ops_loop;
                }
            },
        };
        //println!(" -> {:?}", registers);
        idx = idx + 1;
    }
    println!("Registers {:?}", registers);
}
