extern crate regex;

use std::io::Read;
use std::fs::File;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct FileSys {
    x: i32,
    y: i32,
    size: i32,
    used: i32,
    avail: i32
}

fn main() {
    //Filesystem              Size  Used  Avail  Use%
    let re_filesys = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();

    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    let mut file_systems: Vec<FileSys> = Vec::new();

    for line in input.lines() {
        match re_filesys.captures(line) {
            Some(cap) => file_systems.push(FileSys {
                x: cap.at(1).unwrap().parse().unwrap(),
                y: cap.at(2).unwrap().parse().unwrap(),
                size: cap.at(3).unwrap().parse().unwrap(),
                used: cap.at(4).unwrap().parse().unwrap(),
                avail: cap.at(5).unwrap().parse().unwrap()
            }),
            None => panic!("Unknown data in input"),
        };
    }

    let mut count = 0;
    'source_loop: for source in file_systems.clone() {

        if source.used == 0 {
            continue 'source_loop;
        }
        'dest_loop: for dest in file_systems.clone() {
            if source.x == dest.x && source.y == dest.y {
                continue 'dest_loop;
            }
            if source.used <= dest.avail {
                count = count + 1;
            }
        }
    }
    println!("Count is {}", count);
}
