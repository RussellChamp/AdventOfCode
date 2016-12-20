extern crate crypto;
//extern crate regex;

use crypto::md5::Md5;
use crypto::digest::Digest;
//use regex::Regex;

#[derive(Debug)]
struct Key {
    idx: u64,
    letters: Vec<char>,
    is_valid: bool,
    hash: String,
}

fn repeat_letters(line: &String, size: u64, only_first: bool) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    let mut count = 1; //we always have 1-in-a-row
    let bytes = line.as_bytes();

    'outer: for idx in 0..line.len() - 2 {
        if bytes[idx] == bytes[idx + 1] {
            count = count + 1;
            if count == size && result.iter().position(|&c| c == (bytes[idx] as char)) == None {
                result.push(bytes[idx] as char);
                if only_first {
                    break 'outer;
                }
            }
        } else {
            count = 1;
        }
    }
    result
}

fn purge_keys(keys: Vec<Key>, idx: u64, threshold: u64) -> Vec<Key> {
    //remove all invalid keys with idx lower than threshold
    //println!("Purging keys older than {} - {}", idx,  threshold);
    let old_len = keys.len();
    let new_keys: Vec<Key> =
        keys.into_iter()
        .filter(|key| key.is_valid || key.idx + threshold > idx)
        .collect::<Vec<_>>();
    if old_len > new_keys.len() {
        //println!("Purged {} old keys with an idx < {}", old_len - new_keys.len(), idx - threshold);
    }
    new_keys
}

fn validate_keys(keys: &mut Vec<Key>, key_char: char, idx: u64, threshold: u64) {
    //  will validate any unvalidated keys within the last *1000* that share the repeated char
    let mut count = 0;
    keys.into_iter()
        .map(|key| {
            if !key.is_valid && key.idx + threshold > idx && key.letters.iter().position(|&c| c == key_char) != None {
                key.is_valid = true;
                count = count + 1;
            }
            key
        })
        .collect::<Vec<_>>();
    if count > 0 {
        //println!("On key {} at {} keys, validated {} keys using '{}'", idx, keys.len(), count, key_char);
    }
}

fn we_are_done(keys: &Vec<Key>, pad_size: usize) -> bool {
    //we are done when we have 'pad_size' valid keys in a row
    keys.iter().take_while(|key| key.is_valid ).collect::<Vec<_>>().len() >= pad_size
}

const PAD_SIZE: usize = 64;
const THRESHOLD: u64 = 1000;

fn main() {

    println!("I did something wrong on this one and am not getting the correct answer. :(");

    let mut keys: Vec<Key> = vec![];
    let mut hasher = Md5::new();

    let salt = "ahsbgdzn";

    //let values: Vec<u64> = vec![18, 39, 816];
    for idx in 0..std::u64::MAX {
        hasher.input(format!("{}{}", salt, idx).as_bytes());
        //print!("{}{} -> ", salt, idx);

        let mut output = [0; 16]; //represents MD5 string
        hasher.result(&mut output);
        let output_str: String = output.iter().map(|&c| format!("{:x}", c)).collect();
        //println!("{}", output_str);

        for r in repeat_letters(&output_str, 5, false) {
            //println!("Found a super key for '{}'", r);
            keys = purge_keys(keys, idx, THRESHOLD);
            validate_keys(&mut keys, r, idx, THRESHOLD);
            //println!("  from {}", output_str);
        }

        //println!("{} - {} - {:?}", idx, output_str, repeat_letters(&output_str, 3));
        let threepeats = repeat_letters(&output_str, 3, true);
        if threepeats.len() > 0 {
            //ONLY CONSIDER FIRST TRIPPLET
            let new_key = Key { idx: idx, letters: threepeats, is_valid: false, hash: String::from(output_str) };
            //println!("Added {:?}", new_key);
            keys.push(new_key);
        }

        if we_are_done(&keys, PAD_SIZE) {
            if keys.len() >= PAD_SIZE {
                println!("We're done! Finished on idx {}", idx);
                println!("The {}th key is {:?}", PAD_SIZE, keys.iter().nth(PAD_SIZE).unwrap());
                //println!("The {}th key is {:?}", pad_size, keys[pad_size]);
            } else {
                println!("Uh oh! We didn't find enough keys. :(");
            }
            break;
        }

        hasher.reset();
    }
}
