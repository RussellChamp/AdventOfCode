// First game
// 1 1 -> 2 0
// 1 1 1 1 -> 2 0 2 0 -> 4 0 0 0 //first
// 1 1 1 1 1 1 -> 2 0 2 0 2 0 -> 0 0 0 0 6 0

// 1 -> 1
// 1 1 1                 -> 0 0 3 //last player
// 1 1 1 1 1             -> 0 0 2 0 3             -> 0 0 5 0 0 //middle player
// 1 1 1 1 1 1 1         -> 0 0 2 0 2 0 3         -> 0 0 0 0 0 0 7 //last player
// 1 1 1 1 1 1 1 1 1     -> 0 0 2 0 2 0 2 0 3     -> 0 0 4 0 0 0 5 0 0 -> 0 0 9 0 0 0 0 0 0 //third player
// 1 1 1 1 1 1 1 1 1 1 1 -> 0 0 2 0 2 0 2 0 2 0 3 -> 0 0 0 0 0 0 4 0 0 0 7 -> 0 0 0 0 0 0 11 0 0 0 0 //7th player

// generate list
//   filter based on size
//     filter_even: keep all with even idx
//     filter_odd: keep all with even idx except 0
//   repeat until one remains

fn pass_gifts(players: Vec<i64>) -> i64 {
    //represents the group of players left after a pass
    let is_odd = players.len() % 2 == 1;
    let new_players = players.iter().enumerate().filter(|e| !(is_odd && e.0 == 0) && e.0 % 2 == 0).map(|e| *e.1).collect::<Vec<_>>();
    //println!("{:?}", new_players);
    if new_players.len() == 1 {
        new_players[0]
    } else {
        pass_gifts(new_players)
    }
}

fn play_game(size: i64) -> i64 {
    //take the size of the group and returns the winner
    let players: Vec<i64> = (1..size+1).collect();
    //println!("{:?}", players);
    pass_gifts(players)
}


// Second game
// 1 2 3 4     -> 1 2 - 4   -> 1 2 - -   -> 1
// 1 2 3 4 5   -> 1 2 - 4 5 -> 1 2 - 4 - -> - 2 - 4 - -> 2
// 1 2 3 4 5 6 ->
//looping though all players, remove player at position  floor(players.len()/2) + player_idx

fn play_new_game(size: i64) -> i64 {
    let mut players = (1..size+1).collect::<Vec<i64>>();
    println!("Set up vector of {} players", size);
    let mut current_player_idx = 0;
    let mut player_idx_to_eliminate;
    let mut player_len = size as usize;

    while player_len > 1 {
        player_idx_to_eliminate = (player_len/2 /*drops fraction*/ + current_player_idx) % player_len;
        //println!("Removing idx {} from players (size {}) {:?}", player_idx_to_eliminate, players.len(), players);
        //players.remove(player_idx_to_eliminate);
        players = players.iter().enumerate().filter(|e| e.0 != player_idx_to_eliminate).map(|e| *e.1).collect::<Vec<_>>();
        player_len = player_len - 1;

        current_player_idx = current_player_idx + match player_idx_to_eliminate > current_player_idx {
             true => 1,
             false => 0
        } % player_len;
    }
    //println!("Returning {:?}", players);
    players[0]
}

fn main() {
    let winner = play_game(3014387);
    println!("Part 1: Winner is {}", winner);

    //Need to improve the algorithm so it finishes in a sane amount of time
    let new_winner = play_new_game(3014387);
    println!("Part 2: Winner is {}", new_winner);
}
