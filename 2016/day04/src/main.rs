extern crate regex;

use std::io::Read;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

///// THIS IS NOT WORKING RIGHT. :(

// fn get_lines(input: &String) {
//     let lines: Vec<(&String, i32, &String)> = vec![];
//     lines.push(("Foo", 42, "bar"));
//     return lines;
// }

// fn part_1(input: &Vec<(String, i32, String)>) {
//     println!("Started part 1");
// }

#[derive(Debug)]
struct Room {
    name: String,
    sector: i32,
    check: String,
}

fn part_1(input: &String) {
    let re = Regex::new(r"([a-z\\-]+)-([:digit:]+)\[([:lower:]+)\]").unwrap();

    let sum = 0;
    for cap in re.captures_iter(input) {
        let room = Room {   name: String::from(cap.at(1).unwrap_or("")),
                            sector: cap.at(2).unwrap_or("0").parse().unwrap_or(0),
                            check: String::from(cap.at(3).unwrap_or("")),
                        };
        let mut charMap: HashMap<char, i32> = HashMap::new();
        for ch in room.name.replace("-", "").chars() {
            let l = charMap.entry(ch).or_insert(0);
            *l += 1;
        }
        println!("{:?}", charMap);
        //println!("{:?}", room);
        //let mut letters: Vec<char> = .collect();
        //println!("{:?}", letters.sort());
    }
    println!("Part 1: Total sum of valid rooms is {}", sum);

    // let sum = input
    // .lines()
    // .map(|l|
    //     re.captures(l).unwrap()
    //     //.split(&re)
    //     //.map(|&v| Room { name: v[0] + v[1] + v[2], sector: v[3].parse::<i32>().unwrap(), checksum: v[4] })
    //     //.collect::<Vec<_>>()
    //     //.turn into collection of items
    //     //.collect
    //     )
    // .map(|c| c.at(0))
    // .collect::<Vec<_>>()
    // // //.filter out decoy rooms
    // // //.collect
    // // //.fold and sum the sectors
    // // //.unwrap?
    //  .len();
    // println!("Part 1: Sum was {}", sum);
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    part_1(&input);
}
