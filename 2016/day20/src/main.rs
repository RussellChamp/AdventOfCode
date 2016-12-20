use std::io::Read;
use std::fs::File;
use std::cmp;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Range {
    low: u32,
    high: u32,
}

fn between(low: u32, high: u32) -> u32 {
    if low >= high {
        return 0;
    }
    high - low - 1
}

fn process_addresses(file_name: &str, max_addr: u32) -> (u32, u32) {
    let mut file = File::open(file_name).expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");
    let mut lines = input.lines().map(|l| {
        let values: Vec<&str> = l.split("-").collect();
        Range { low: values[0].parse::<u32>().unwrap(), high: values[1].parse::<u32>().unwrap() }
    }).collect::<Vec<_>>();
    lines.sort();

    let mut last_high = 0; //since testable range starts at 0
    let mut first_good = 0;
    let mut good_count = 0;
    let first = lines.iter().next().unwrap();
    println!("{} ->", first.low);
    'outer: for line in lines.iter() {
        if last_high > line.high {
            continue 'outer; //and keep the old last_high
        }
        //println!("{:?}", line);
        if last_high + 2 > line.low {
            // it is contiguous or within range of the previous range and there are no new good addresses
            //println!("({} -> {})", last_high, line.high);
        } else {
            // there is a gap between las_high and our current low
            println!("{} ({}) {} -> ", last_high, between(last_high, line.low), line.low);
            if first_good == 0 {
                first_good = last_high + 1;
                //break 'outer;
            }
            //count the gap between the last range's high and our current last_high
            good_count = good_count + between(last_high, line.low);
            //print!("{} -> ", good_count);
        }
        last_high = cmp::max(last_high, line.high)
    }
    //let last = lines.iter().last().unwrap();
    println!("{} ({}) {}", last_high, between(last_high, max_addr), max_addr);
    if last_high < max_addr {
        good_count = good_count + between(last_high, max_addr) + 1; //inclusive of highest value
    }
    (first_good, good_count)
}

fn main() {
    let (first_good, good_count) = process_addresses("./data", 4294967288);
    println!("Part 1: First nonblocked address is {}", first_good);
    println!("Part 2: Counted {} valid addresses", good_count);
}

#[test]
fn test_between() {
    assert_eq!(8, between(1,10));
}

#[test]
fn small_test() {
    let (first_good, good_count) = process_addresses("./small_data", 11);
    assert_eq!(3, first_good);
    assert_eq!(4, good_count);
}