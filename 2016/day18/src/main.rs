// Trap rules
// * if 110 or 011, 100, 001
// where nonexistent sides are considered 0

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Trapped = 1,
    Safe = 0
}

type Row = Vec<Tile>;
type Grid = Vec<Row>;

//Part 1

fn add_row(grid: &mut Grid) {
    static TRAPS: [[Tile; 3]; 4] =
    [
        [Tile::Trapped, Tile::Trapped, Tile::Safe],
        [Tile::Safe,    Tile::Trapped, Tile::Trapped],
        [Tile::Trapped, Tile::Safe,    Tile::Safe],
        [Tile::Safe,    Tile::Safe,    Tile::Trapped],
    ];

    let last_row = grid.clone().into_iter().last().unwrap();
    //println!("Last row {:?}", last_row);
    let mut new_row: Row = vec![];

    for idx in 0..last_row.len() {
        let mut pre: [Tile; 3] = [Tile::Safe, Tile::Safe, Tile::Safe];
        if idx > 0 {
            pre[0] = last_row[idx - 1].clone();
        }
        pre[1] = last_row[idx].clone();
        if idx + 1 < last_row.len() {
            pre[2] = last_row[idx + 1].clone();
        }
        if TRAPS.contains(&pre) {
            new_row.push(Tile::Trapped);
        } else {
            new_row.push(Tile::Safe);
        }
    }
    //println!("New row: {:?}", new_row);

    grid.push(new_row);
}

fn create_grid(rows: usize) -> Grid {
    let first_row: Row =
        String::from(".^..^....^....^^.^^.^.^^.^.....^.^..^...^^^^^^.^^^^.^.^^^^^^^.^^^^^..^.^^^.^^..^.^^.^....^.^...^^.^.")
        .chars().map(|c| {
            match c {
                '^' => Tile::Trapped,
                 _  => Tile::Safe,
            }
        }).collect::<Row>();
    let mut grid: Grid = vec![first_row];
    while grid.len() < rows {
        add_row(&mut grid);
    }
    grid
}


// Part 2
fn step_row(last_row: &mut Row) -> i64 {
    static TRAPS: [[Tile; 3]; 4] =
    [
        [Tile::Trapped, Tile::Trapped, Tile::Safe],
        [Tile::Safe,    Tile::Trapped, Tile::Trapped],
        [Tile::Trapped, Tile::Safe,    Tile::Safe],
        [Tile::Safe,    Tile::Safe,    Tile::Trapped],
    ];

    let mut new_row: Row = vec![];
    let mut safe_tiles = 0;
    for idx in 0..last_row.len() {
        let mut pre: [Tile; 3] = [Tile::Safe, Tile::Safe, Tile::Safe];
        if idx > 0 {
            pre[0] = last_row[idx - 1].clone();
        }
        pre[1] = last_row[idx].clone();
        if idx + 1 < last_row.len() {
            pre[2] = last_row[idx + 1].clone();
        }
        if TRAPS.contains(&pre) {
            new_row.push(Tile::Trapped);
        } else {
            new_row.push(Tile::Safe);
            safe_tiles = safe_tiles + 1;
        }
    }
    *last_row = new_row;
    safe_tiles
}

fn main() {
    let grid = create_grid(40);
    let total_safe = grid.iter().fold(0, |sum, r| sum + r.iter().filter(|t| **t == Tile::Safe).count());
    let total_trap = grid.iter().fold(0, |sum, r| sum + r.iter().filter(|t| **t == Tile::Trapped).count());
    println!("Part 1: There are {} safe tiles and {} traps", total_safe, total_trap);

    //Need to improve the algorithm so it finishes in a sane amount of time
    let mut row: Row =
        String::from(".^..^....^....^^.^^.^.^^.^.....^.^..^...^^^^^^.^^^^.^.^^^^^^^.^^^^^..^.^^^.^^..^.^^.^....^.^...^^.^.")
        .chars().map(|c| {
            match c {
                '^' => Tile::Trapped,
                 _  => Tile::Safe,
            }
        }).collect::<Row>();
    let mut safe_tiles = 48; //from row 1
    for _ in 0..400000 - 1 {
        safe_tiles = safe_tiles + step_row(&mut row);
    }
    println!("Part 2: There are {} safe tiles", safe_tiles);
}