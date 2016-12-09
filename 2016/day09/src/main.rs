extern crate regex;

use regex::Regex;
use std::io::Read;
use std::fs::File;

fn part_1(input: &String) {
    let set: Vec<char> = input.chars().collect();
    let mut idx: usize = 0;
    let mut output = String::new();
    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    //step by single characters
    //when you find an open paren, look for the next closing paren and match to a regex like (x,y)
    //if it does not match, add contents to output and continue
    //if it matches, add the next 'x' characters to the output 'y' times and increase idx by 'x'
    //if not an open paren, add to output
    while idx < input.len() {
        let c = set[idx];
        match c {
            '(' => {
                let old_idx: usize = idx; //for when we have to colleect the slice
                loop {
                    idx = idx + 1;
                    match set[idx] {
                        ')' => break,
                        _ => {},
                    }
                }
                let cap_slice = &input[old_idx..idx+1];
                let cap = re.captures(cap_slice).unwrap();

                let char_count: usize = cap.at(1).unwrap().parse().unwrap();
                let times = cap.at(2).unwrap().parse().unwrap();

                print!("({}x{}) ", char_count, times);
                //println!("({}x{}) adding slice from {} to {}", char_count, times, idx, idx+char_count);
                for time in 0..times {
                    output = output + &input[idx+1..idx+char_count+1]; //copy the next char_count characters
                }
                idx = idx + char_count; //skip over all the chars we added

            },
            _ => {
                output.push(c);
                //println!("{} input is len {}", idx, input.len());
                },
        }
        idx = idx + 1; //increment idx in every case
    }
    println!("Final size is {}", output.len());
}

fn get_count(input: &String) -> usize {
    let re_letters = Regex::new(r"^[^\(\)]*$").unwrap(); //I'd prefer to make this static but don't know how
    if re_letters.is_match(&input) { //if there are no parens
        return input.len(); //return the length
    }
    let re = Regex::new(r"^([:alpha:]*)\((\d+)x(\d+)\)(.*)$").unwrap();
    match re.captures(input) {
        None => println!("Found nothing!"),
        Some(cap) => {
            let letters = cap.at(1).unwrap().to_string();
            let count: usize = cap.at(2).unwrap().parse().unwrap();
            let times: usize = cap.at(3).unwrap().parse().unwrap();
            let subslice = &cap.at(4).unwrap()[0..count].to_string();
            let therest = &cap.at(4).unwrap()[count..].to_string();

            //println!("Found: {} - {},{}", cap.at(1).unwrap(), cap.at(2).unwrap(), cap.at(3).unwrap()),
            println!("({},{}) - {}x {}", count, times, times, subslice);
            return get_count(&letters) + times * get_count(subslice) + get_count(therest);
        }
    }
    0
}

fn part_2(input: &String) {
    println!("Part 2: Length is {}", get_count(input));
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    //part_1(&input);
    part_2(&input);
}
