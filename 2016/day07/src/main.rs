extern crate regex;

use regex::Regex;
use std::io::Read;
use std::fs::File;

fn ugly_match(text: &String) -> bool {
    if text.len() < 4 {
        return false
    }
    //check for ABBA pattern
    for idx in 0..text.len() - 3 {
        if text.chars().nth(idx).unwrap() == text.chars().nth(idx+3).unwrap() &&
            text.chars().nth(idx+1).unwrap() == text.chars().nth(idx+2).unwrap() &&
            text.chars().nth(idx).unwrap() != text.chars().nth(idx+1).unwrap() {
            return true
        }
    }
    return false
}

fn part_1(input: &String) {

    let re = Regex::new(r"\[([:lower:]+)\]").unwrap();
    let mut count = 0;
    let mut rejected = 0;
    let mut none = 0;

    'outer: for line in input.lines() {

        //hypernet sequences
        for cap in re.captures_iter(line) {
            let cap = cap.at(1).unwrap_or("");
            //reject if cap contains an ABBA
            if ugly_match(&cap.to_string()) {
                //println!("- {}", cap);
                rejected = rejected + 1;
                continue 'outer;
            }
        }

        //addresses
        for cap in re.replace_all(line, ",").split(",") {
            //println!("Matched bit {}", cap);
            if ugly_match(&cap.to_string()) {
                //println!("+ {}", cap);
                count = count + 1;
                continue 'outer;
            }
        }

        none = none + 1;
    }
    println!("Count: {} Rejected: {} None: {}", count, rejected, none);
    println!("Part 1 Answer: {}", count);
}

fn part_2(input: &String) {
    let re = Regex::new(r"\[([:lower:]+)\]").unwrap();
    let mut count = 0;

    'outer: for line in input.lines() {

        for cap in re.replace_all(line, ",").split(",") {
            let bab: String = format!(cap.chars().nth(1).unwrap(), cap.chars().nth(0).unwrap(), cap.chars().nth(1).unwrap());
            println!("Generated bab {}", bab);
            let bre = Regex::new(&format!("{}{}{}", r"\[(", bab, r")\]")).unwrap();
            //check for ABAs
            //check for related BAB
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
