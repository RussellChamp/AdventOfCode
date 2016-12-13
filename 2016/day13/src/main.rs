#[derive(Debug)]
enum Space {
    Wall,
    Open,
}

fn get_space(x: u32, y: u32, favorite_number: u32) -> Space {
    let total: u32 = x*x + 3*x + 2*x*y + y + y*y + favorite_number;
    //println!("{}, {}", total, total.count_ones());
    if total.count_ones() % 2 == 0 { /*even*/
        Space::Open
    }
    else { /*odd*/
        Space::Wall
    }
}

// fn print_maze(width: u32, height: u32, key: u32) {
//     //print header
//     print!("  ");
//     for col in 0..width {
//         print!("{}", col % 10);
//     }
//     println!("");
//     for row in 0..height {
//         print!("{} ", row % 10);
//         for col in 0..width {
//             match get_space(col, row, key) {
//                 Space::Wall => print!("#"),
//                 Space::Open => print!("."),
//             }
//         }
//         println!("");
//     }
// }

const MAZE_WIDTH: usize = 40;
const MAZE_HEIGHT: usize = 50;
const MAX_STEPS: u32 = 50;
static mut COUNT: u32 = 0;

fn build_maze(maze: &mut Vec<Vec<(bool, u32)>>, key: u32) {
    for row in 0..MAZE_HEIGHT {
        for col in 0..MAZE_WIDTH {
            match get_space(col as u32, row as u32, key) {
                Space::Open => maze[row][col] = (true, u32::max_value()),
                Space::Wall => maze[row][col] = (false, 0),
            }
        }
    }
}

fn walk_maze(maze: &mut Vec<Vec<(bool, u32)>>, pos: (usize, usize), step: u32) {
    if maze[pos.0][pos.1].0 == false /* if position is a wall */
        || step >= maze[pos.0][pos.1].1 /* or position has a better step value */ {
        return; // abort now
    }
    // otherwise
    // if this space is within MAX_STEPS and hasn't yet been counted
    if maze[pos.0][pos.1].1 == u32::max_value() { //if we never visited this space before
        if step <= MAX_STEPS {
            unsafe {
                COUNT = COUNT + 1;
            }
        }
    }
    else if maze[pos.0][pos.1].1 > MAX_STEPS && step <= MAX_STEPS {
        unsafe {
            COUNT = COUNT + 1;
        }
    }

    // set the step value
    maze[pos.0][pos.1].1 = step;

    //check left
    if pos.1 > 0 {
        walk_maze(maze, (pos.0, pos.1-1), step+1);
    }

    //check right
    if pos.1 + 1 < MAZE_WIDTH {
        walk_maze(maze, (pos.0, pos.1+1), step+1);
    }

    //check up
    if pos.0 > 0 {
        walk_maze(maze, (pos.0-1, pos.1), step+1);
    }

    //check down
    if pos.0 + 1 < MAZE_HEIGHT {
        walk_maze(maze, (pos.0+1, pos.1), step+1);
    }
}

fn print_maze(maze: &Vec<Vec<(bool, u32)>>) {
    print!("    "); //first row padding
    for col in 0..MAZE_WIDTH {
        print!("{:02} ", col % 100);
    }
    println!(""); //newline
    for row in 0..MAZE_HEIGHT {
        print!("{:02}: ", row % 100);
        for col in 0..MAZE_WIDTH {
            if maze[row][col].0 == true {
                if maze[row][col].1 == u32::max_value() {
                    print!(".. "); //unpassable
                }
                else {
                    print!("{:02} ", maze[row][col].1 % 100); //step value
                }
            }
            else {
                print!("## "); //it's a wall
            }
        }
        println!(""); //print the new line
    }
}

fn main() {
    //let mut pos = (1, 1);
    //let goal = (31, 39);

    //println!("Answer is {:?}", get_space(1, 2, 3));
    let mut maze: Vec<Vec<(bool, u32)>> = vec![vec![(false, 0); MAZE_WIDTH]; MAZE_HEIGHT];
    //print_maze(32, 40, 1358);
    build_maze(&mut maze, 1358);
    walk_maze(&mut maze, (1, 1), 0);
    //print_maze(&maze);
    let end = (31, 39);
    println!("Part 1: It takes {} steps to get to ({}, {})", maze[end.1][end.0].1, end.0, end.1);
    unsafe {
        println!("Part 2: There are {} positions within {} steps", COUNT, MAX_STEPS);
    }
}
