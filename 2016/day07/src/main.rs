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

fn uglier_match(text: &String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for idx in 0..text.len() - 2 {
        if text.chars().nth(idx).unwrap() == text.chars().nth(idx+2).unwrap() {
            ret.push(text[idx..idx+3].to_string());
        }
    }
    ret
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
        //println!("{}", line);
        let mut abas: Vec<String> = Vec::new();
        for cap in re.replace_all(line, ",").split(",") {
            for m in uglier_match(&cap.to_string()) {
                abas.push(m.to_string()); //add ABA to big list
            }
        }
        //check for related BAB
        for cap in re.captures_iter(line) {
            let text = cap.at(0).unwrap_or("");
            for aba in abas.iter() {
                let mut bab: String = String::new();
                bab.push(aba.chars().nth(1).unwrap());
                bab.push(aba.chars().nth(0).unwrap());
                bab.push(aba.chars().nth(1).unwrap());
                //println!("looking for {} in {}", bab, text);
                let bre = Regex::new(bab.as_str()).unwrap();
                if bre.is_match(text) {
                    //println!("{}", line);
                    count = count + 1;
                    continue 'outer;
                }
            }
        }
        //println!("{} - {:?}", line, abas);
    }
    println!("Part 2: Found {} matches", count);
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    part_1(&input);
    part_2(&input);
}
