extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

//This was mostly stolen from a github gist that I found while looking for a Rust MD5 library
fn part_1() {
    let mut hasher = Md5::new();

    let key = "reyedfim".as_bytes();
    let mut answer: Vec<u8> = vec![];

    let mut count = 8;
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16]; //represents MD5 string
        hasher.result(&mut output);

        //check the first two and a half bytes of the output
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            let output_str: String = output.iter().map(|&c| format!("{:x}{:x}", c >> 4, c)).collect();
            println!("hashed {} to get {}", i, output_str);
            answer.push(output[2]);
            match count > 1 {
                true => count -= 1,
                false => break,
            }
        }
        hasher.reset();
    }
    let answer_str: String = answer.iter().map(|&c| format!("{:x}", c)).collect();
    println!("Part 1: Answer is {}", answer_str);
}

fn part_2() {
    let mut hasher = Md5::new();

    let key = "reyedfim".as_bytes();
    let mut answer = [0; 8];
    let mut answer_mask = [true; 8];

    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16]; //represents MD5 string
        hasher.result(&mut output);

        //check the first two and a half bytes of the output
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            let output_str: String = output.iter().map(|&c| format!("{:x}{:x}", c >> 4, c)).collect();
            println!("hashed {} to get {}", i, output_str);
            //answer.push(output[2]);
            let idx = output[2] as usize;
            let val = output[3] >> 4;
            if idx < 8 && answer_mask[idx] {
                answer[idx] = val;
                answer_mask[idx] = false; //we filled this slot
                println!("Values are {} and {}", idx, val);
                match answer_mask.iter().fold(false, |f, &i| f || i) {
                    true => {}, //keep going
                    false => break, //we're all done here
                }
            }
        }
        hasher.reset();
    }
    let answer_str: String = answer.iter().map(|&c| format!("{:x}", c)).collect();
    println!("Part 1: Answer is {}", answer_str);
}

fn main() {
    //part_1();
    part_2();
}
