use std::io::Read;
use std::fs::File;
//use std::collections::HashMap;

fn part_1(input: &String) {
    // This would have been great if it actually worked.
    // let values = input.lines()
    //     .map(|l| l
    //         .trim()
    //         .chars()
    //         .collect::<Vec<_>>()
    //         )
    //     .collect::<Vec<_>>();
    // println!("first item {:?}", values[0]);
    // println!("second item {:?}", values[1]);

    // //let mut letters: Vec<String> = vec![String::new(); values[0].len()];
    // let mut letters: Vec<Vec<char>> = vec![vec![]; values[0].len()];
    // for v in values {
    //     for (idx, l) in v.iter().enumerate() {
    //         letters[idx].push(*l);
    //     }
    // }
    // for (idx, line) in letters.iter().enumerate() {
    //     //println!("Letter {}: {:?}", idx, s);
    //     let mut hash: HashMap<char, i32> = HashMap::new();
    //     let mut largest: (char, i32) = ('a', -1);
    //     for l in line {
    //         match hash.get(l) {
    //             Some(v) => {
    //                 v = &(v + 1);
    //                 // if v > largest.1 {
    //                 //     largest = (l, v);
    //                 // }
    //             },
    //             None => hash.insert(*l, 1),
    //         }
    //     }

    //     //let s: String = l.iter().cloned().collect();
    // }

    let size = input.lines().nth(0).unwrap().len();
    print!("Part 1: Answer ");
    for idx in 0..size {
        let mut accumulator = [0; 26];
        for line in input.lines() {
            let letter = line.chars().nth(idx).unwrap();
            let letter_idx = letter as u8 - 'a' as u8;
            accumulator[letter_idx as usize] = accumulator[letter_idx as usize] + 1;
            //println!("The {}th letter is {} with a value {} - {:?}", idx, letter, letter_idx, accumulator);
        }
        let mut biggest = (0, -1); //idx and value
        for idx in 0..accumulator.len() {
            if accumulator[idx] > biggest.1 {
                biggest = (idx, accumulator[idx]);
            }
        }
        let biggest_letter = (biggest.0 as u8 + 'a' as u8) as char;
        // println!("Accumulator is {:?}", accumulator);
        // println!("Biggest is {:?} which is {}", biggest, biggest_letter);
        print!("{}", biggest_letter);
    }
    println!(""); //newline
}

fn part_2(input: &String) {
    let size = input.lines().nth(0).unwrap().len();
    print!("Part 2: Answer ");
    for idx in 0..size {
        let mut accumulator = [0; 26];
        for line in input.lines() {
            let letter = line.chars().nth(idx).unwrap();
            let letter_idx = letter as u8 - 'a' as u8;
            accumulator[letter_idx as usize] = accumulator[letter_idx as usize] + 1;
            //println!("The {}th letter is {} with a value {} - {:?}", idx, letter, letter_idx, accumulator);
        }
        let mut smallest = (0, <i32>::max_value()); //idx and value
        for idx in 0..accumulator.len() {
            if accumulator[idx] > 0 && accumulator[idx] < smallest.1 {
                smallest = (idx, accumulator[idx]);
            }
        }
        let smallest_letter = (smallest.0 as u8 + 'a' as u8) as char;
        // println!("Accumulator is {:?}", accumulator);
        // println!("Smallest is {:?} which is {}", smallest, smallest_letter);
        print!("{}", smallest_letter);
    }
    println!(""); //newline
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    part_1(&input);
    part_2(&input);
}
