
type Bits = Vec<bool>;

fn gen_data(bits: &mut Bits, size: usize) {
    //println!("[{:03}] => {}", bits.len(), bit_string(&bits));
    if bits.len() >= size {
        //println!("Drained ->");
        bits.drain(size..).collect::<Vec<_>>();
        //println!("[{:03}] => {}", bits.len(), bit_string(&bits));
    }
    else {
        let cur_len = bits.len();
        //add a buffer 0
        bits.push(false);
        //add the reverse copy
        for i in (0..cur_len).rev() {
            let bit = bits[i];
            //print!("{}.", bits[i]);
            bits.push(!bit);
        }
        gen_data(bits, size); //until we get the appropriate size
    }
}

fn get_hash(bits: Bits) -> Bits {
    let mut hash: Bits = vec![];
    for pair in bits.as_slice().chunks(2) {
        if pair.len() > 1 {
            //println!("Pair: ({}, {})", pair[0], pair[1]);
            hash.push(pair[0] == pair[1]);
        }
    }
    //println!("Hash => {}", bit_string(&hash));
    match hash.len() % 2 {
        0 /*even*/ => get_hash(hash),
        _ /*odd*/  => hash,
    }
}

trait ToBits {
    fn to_bits(&self) -> Bits;
}

impl ToBits for String {
    fn to_bits(&self) -> Bits {
        let mut bits = vec![];
        for c in self.chars() {
            match c {
                '0' => bits.push(false),
                '1' => bits.push(true),
                _ => {}, //toss it
            }
        }
        bits
    }
}

fn bit_string(bits: &Vec<bool>) -> String {
    bits.into_iter()
        .map(|b|
            match *b {
                true => '1',
                false => '0'
            }
        )
        .collect::<String>()
}

fn calc_answer(input: &str, length: usize) {
    let mut bits: Bits = String::from(input).to_bits();
    gen_data(&mut bits, length);
    //println!("Using {}", bit_string(&bits));
    let hash: Bits = get_hash(bits);
    println!("Hash is {}", bit_string(&hash));
}

fn main() {
    // Part 1
    print!("Part 1: ");
    calc_answer("01110110101001000", 272);

    // Part 2
    print!("Part 2: ");
    calc_answer("01110110101001000", 35651584);
}
