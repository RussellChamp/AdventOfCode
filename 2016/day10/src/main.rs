extern crate regex;

use regex::Regex;
use std::io::Read;
use std::fs::File;


//I'd like to solve this with a tree or node of sorts but don't know how
enum Pass {
    Bot,
    Output
}

struct Bot {
    id: i32, //the identifier for this bot
    chips: Vec<i32>, //the value of the chips a bot can receive
    low: i32, //where to pass the low chip
    high: i32, //where to pass the high chip
}

struct Output {
    id: i32, //the identifier for this output
    chips: Vec<i32>,
}

fn pass_chip(all_bots: Vec<Bot>, bot_id: i32, value: i32) {
    let bot: Bot = all_bots.iter().find(|&b| b.id == bot_id).unwrap();
    bot.chips.push(value);
    if bot.chips.len() == 2 {
        pass_chip(all_bots, bot.low, bot.chips.iter().min().unwrap());
        pass_chip(all_bots, bot.high, bot.chips.iter().max().unwrap());
    }
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    let re_bot = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();
    let re_val = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();

    let bots: Vec<Bot> = Vec::new();

    for line in input.lines() {
        if re_bot.is_match(line) {
            let cap = re_bot.captures(line).unwrap();
            let bot = Bot { id: cap.at(1).unwrap().parse().unwrap(), chips: vec![], low: None, high: None };
            println!("This dude {} will give to {} and {}", cap.at(1).unwrap(), cap.at(3).unwrap(), cap.at(5).unwrap());
        }
        else if re_val.is_match(line) {
            let cap = re_val.captures(line).unwrap();

        }
    }

    //for each line of input
    // either add a new bot "node" to the tree
    // or give a chip to a bot and let it propogate through the tree
}
