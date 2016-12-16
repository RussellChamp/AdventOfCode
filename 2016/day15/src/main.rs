extern crate regex;
extern crate num;

use regex::Regex;
use num::Integer;

use std::io::Read;
use std::fs::File;

#[derive(Debug, Clone, Copy)]
struct Disc {
    id: u32,
    pos: u32,
    start: u32
}

fn is_valid(disc: &Disc, time: u32) -> bool {
    // a disk starts at position 'start'
    // and ticks up to maximum position 'pos' then loops back to the first position
    // a disk will be valid if it is at position 0 when hit
    // there is a delay of seconds equal to the disc layer, or "id"
    (disc.start + time + disc.id) % disc.pos == 0
}

fn get_discs(input: &String) -> Vec<Disc> {
    let mut discs: Vec<Disc> = vec![];

    let re = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)\.").unwrap();
    //read all discs
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        discs.push(Disc {
            id: cap.at(1).unwrap().parse().unwrap(),
            pos: cap.at(2).unwrap().parse().unwrap(),
            start: cap.at(3).unwrap().parse().unwrap()
            });
    }
    //println!("Discs: {:?}", discs);
    discs
}

fn cycle_discs(discs: &Vec<Disc>) {
    //find full cycle length (through lowest common multiple of all disc positions)
    let full_cycle = discs.iter().fold(1, |total, x| total.lcm(&x.pos));
    //println!("LCM {}", full_cycle);
    //iterate through each cycle until you find a match
    'time_loop: for time in 0..full_cycle {
        for disc in discs {
            if !is_valid(disc, time) {
                continue 'time_loop;
            }
        }
        //we made it all the way through the discs!
        println!("We made it at time={}", time);
        break 'time_loop;
    }
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    // Part 1
    print!("Part 1: ");
    let mut discs = get_discs(&input);
    cycle_discs(&discs);

    // Part 2
    print!("Part 2: ");
    let new_id = (discs.len() + 1) as u32;
    discs.push(Disc {id: new_id, pos: 11, start: 0 });
    cycle_discs(&discs);

}
