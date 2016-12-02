use std::cmp;
use std::io::Read;
use std::fs::File;

fn clamp(val: i32, lb: i32, ub: i32) -> i32 {
    cmp::min(cmp::max(val, lb), ub)
}

fn state_to_val(state: (i32, i32)) -> i32 {
    state.0 + 3 * state.1 + 1
}

fn state_to_new_val(state: (i32, i32)) -> char {
    let val_map = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' ']
    ];
    val_map[state.1 as usize][state.0 as usize]
}

fn part_1(input: &String) {
    let mut state =  (1, 1);
    // model a 3x3 grid that looks like
    // 1 2 3
    // 4 5 6
    // 7 8 9
    // where (0,0) starts in the upper left

    //println!("Start at {}", state_to_val(state));
    let mut answer = String::new();

    for line in input.split('\n') {
        for letter in line.trim().chars() {
            match letter {
                'R' => { state.0 = clamp(state.0 + 1, 0, 2); },
                'L' => { state.0 = clamp(state.0 - 1, 0, 2); },
                'U' => { state.1 = clamp(state.1 - 1, 0, 2); },
                'D' => { state.1 = clamp(state.1 + 1, 0, 2); },
                _ => println!("Unknown!")
            }
            //println!("Moved {} to {}", letter, state_to_val(state));
        }
        //println!("End on {}", state_to_val(state));
        answer += &state_to_val(state).to_string();
    }
    println!("Part 1 answer is {}", answer);
}

fn part_2(input: &String) {
    let mut state = (0, 2);
    // model a 5x5 grid that looks like
    //     1
    //   2 3 4
    // 5 6 7 8 9
    //   A B C
    //     D
    // where blank spaces are invalid
    let valid_map = [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0]
    ];
    let mut answer = String::new();

    for line in input.split('\n') {
        for letter in line.trim().chars() {
            let mut new_state = state;
            match letter {
                'R' => { new_state.0 = clamp(state.0 + 1, 0, 4); },
                'L' => { new_state.0 = clamp(state.0 - 1, 0, 4); },
                'U' => { new_state.1 = clamp(state.1 - 1, 0, 4); },
                'D' => { new_state.1 = clamp(state.1 + 1, 0, 4); },
                _ => println!("Unknown!")
            }
            // if this is a valid move position
            if valid_map[new_state.1 as usize][new_state.0 as usize] == 1 {
                state = new_state;
                //println!("Moved {} to {}", letter, state_to_new_val(state));
            }
        }
        //println!("End on {}", state_to_new_val(state));
        answer.push(state_to_new_val(state));
    }
    println!("Part 2 answer is {}", answer);
}

fn main() {
    let mut file = File::open("./data").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    part_1(&input);
    part_2(&input);
}
