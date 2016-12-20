extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

// If you are at the goal
//  return
// else
// For each possible direction you can go
//  step in that direction
//  loop

type Path = Vec<char>;

struct Pos {
    row: u32,
    col: u32
}

const WIDTH: u32 = 4;
const HEIGHT: u32 = 4;

static KEYS: [char; 5] = ['b', 'c', 'd', 'e', 'f'];

fn recursive_step(passcode: &str, pos: Pos, path: Path) -> Vec<Path> {
    if pos.row == 3 && pos.col == 3 {
        //println!("{}", path.iter().cloned().collect::<String>());
        return vec![path];
    }

    let mut paths: Vec<Path> = Vec::new();
    let mut hasher = Md5::new();
    hasher.input(format!("{}{}", passcode, path.iter().cloned().collect::<String>()).as_bytes());
    let result = hasher.result_str();


    if pos.row > 0 && KEYS.contains(&result.clone().chars().nth(0).unwrap()) {
        let mut rec_path = path.clone(); rec_path.push('U');
        paths.append(&mut recursive_step(passcode,  Pos { row: pos.row - 1, col: pos.col }, rec_path));
    }

    if pos.row + 1 < HEIGHT && KEYS.contains(&result.clone().chars().nth(1).unwrap()) {
        let mut rec_path = path.clone(); rec_path.push('D');
        paths.append(&mut recursive_step(passcode,  Pos { row: pos.row + 1, col: pos.col }, rec_path));
    }

    if pos.col > 0 && KEYS.contains(&result.clone().chars().nth(2).unwrap()) {
        let mut rec_path = path.clone(); rec_path.push('L');
        paths.append(&mut recursive_step(passcode,  Pos { row: pos.row, col: pos.col - 1 }, rec_path));
    }

    if pos.col + 1 < WIDTH && KEYS.contains(&result.clone().chars().nth(3).unwrap()) {
        let mut rec_path = path.clone(); rec_path.push('R');
        paths.append(&mut recursive_step(passcode,  Pos { row: pos.row, col: pos.col + 1 }, rec_path));
    }

    paths
}

fn get_paths(passcode: &str) -> Vec<Path> {
    let pos = Pos { row: 0, col: 0 };

    let paths: Vec<Path> = recursive_step(passcode, pos, Vec::new());
    paths
}

fn main() {
    let passcode = "rrrbmfta";
    let paths: Vec<Path> = get_paths(passcode);

    //find the shortest size then all paths that size since min_by does not work
    let shortest_dist = paths.clone().into_iter().map(|p| p.len()).min().unwrap(); //.min_by(|x, y| x.len().cmp(&y.len()) );
    let longest_dist =  paths.clone().into_iter().map(|p| p.len()).max().unwrap();

    let shortest_paths: Vec<Path> = paths.clone().into_iter().filter(|p| p.len() == shortest_dist).collect();
    //let longest_paths: Vec<Path> = paths.clone().into_iter().filter(|p| p.len() == longest_dist).collect();

    println!("Part 1: Shortest path {:?}", shortest_paths[0]);
    println!("Part 2: Longest path sized {}", longest_dist);
}
