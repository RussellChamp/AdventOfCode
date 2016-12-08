use std::io::Read;
use std::fs::File;

fn part_1(input: &String) {
    let total = input.lines()
        .map(|l| l
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .filter(|v| v.iter().fold(0, |sum, x| sum + x) > 2 * v.iter().max().unwrap() )
        .collect::<Vec<_>>()
        .len();
    println!("Part 1: Total was {}", total);
}

fn part_2(input: &String) {
    //I am not skill enough with iterator adaptors to use .as_slice().chunks(3)
    //so we will do things the lazy way. sad
    let mut triangles = vec![vec![0; 3]; 3];
    let mut read_lines = 3;
    let mut total = 0; //total good triangles
    for line in input.lines().map(|l| l.to_string()).collect::<Vec<String>>().iter() {
        let values = line.trim().split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
        //println!("Values are {:?}", values);
        triangles[0][3 - read_lines] = values[0];
        triangles[1][3 - read_lines] = values[1];
        triangles[2][3 - read_lines] = values[2];
        read_lines = read_lines - 1;
         if read_lines == 0 {
             //process our triangles
             let good = triangles
                .iter()
                .filter(|v| {
                    //println!("filtering {:?}", v);
                    v.iter().fold(0, |sum, x| sum + x) > 2 * v.iter().max().unwrap() }
                )
                .collect::<Vec<_>>()
                .len();
             //println!("Found {} good ones from this set", good);
             total = total + good;
             read_lines = 3; //reset
         }
    }
    println!("Part 2: there are {} good triangles", total);
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    part_1(&input);
    part_2(&input);
}
