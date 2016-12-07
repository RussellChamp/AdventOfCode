extern crate regex;

use std::io::Read;
use std::fs::File;
use regex::Regex;
//use std::collections::HashMap;

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

    let mut sum = 0;
    'outer: for cap in re.captures_iter(input) {
        let room = Room {   name: String::from(cap.at(1).unwrap_or("")),
                            sector: cap.at(2).unwrap_or("0").parse().unwrap_or(0),
                            check: String::from(cap.at(3).unwrap_or("")),
                        };
        //println!("{:?}", room);
        let mut accumulator = [0; 26];
        for letter in room.name.replace("-", "").chars() {
            let letter_idx = letter as u8 - 'a' as u8;
            accumulator[letter_idx as usize] = accumulator[letter_idx as usize] + 1;
            //println!("The letter is {} with a value {} - {:?}", letter, letter_idx, accumulator);
        }

        //println!("Accumulated {:?}", accumulator);

        let mut letters_to_match = room.check.len();
        let mut biggest_value = -1;
        let mut biggest_idxs: Vec<u8> = Vec::new();
        while letters_to_match > 0 {
            for idx in 0..accumulator.len() {
                if accumulator[idx] > biggest_value {
                    biggest_value = accumulator[idx];
                    biggest_idxs = vec![idx as u8];
                } else if accumulator[idx] == biggest_value {
                    biggest_idxs.push(idx as u8); //add it with the others
                }
            }
            biggest_idxs.reverse(); //we want to take items off in the order we put them on
            //println!("These letters had a score of {}: {:?}", biggest_value, biggest_idxs);
            while letters_to_match > 0 && biggest_idxs.len() > 0 {
                let letter_idx = biggest_idxs.pop().unwrap();
                let letter_value = (letter_idx + 'a' as u8) as char;
                if ! room.check.as_str().contains(letter_value) {
                    continue 'outer;
                }
                accumulator[letter_idx as usize] = 0; //nope this broke things
                biggest_value = -1;
                letters_to_match = letters_to_match - 1;
            }
        }
        //phew! we successfully checked this room. the sector is old, but it checks out
        println!("Valid! {:?}", room);
        sum = sum + room.sector;

        // let mut char_map: HashMap<char, i32> = HashMap::new();
        // for ch in room.name.replace("-", "").chars() {
        //     let l = char_map.entry(ch).or_insert(0);
        //     *l += 1;
        // }
        // println!("{:?}", char_map);
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
