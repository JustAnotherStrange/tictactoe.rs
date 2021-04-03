// TODO
// center on terminal using https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
// add stats after winning

// for random
use rand::{thread_rng, Rng};
// for time stuff
use std::{thread, time};
// for usize to i32 convert
use std::convert::TryInto;
// define tile (E for empty)
#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    X,
    O,
    E,
}

fn clear() {
    // clears the terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
fn print_tile(t: Tile) -> char {
    // formats the tile enum into a char
    return match t {
        Tile::X => 'X',
        Tile::O => 'O',
        _ => '-',
    };
}

fn print_board(board: &mut [Tile; 9]) {
    // prints board with nice formatting and a key off to the side
    println!(
        "{} | {} | {}                    1 | 2 | 3",
        print_tile(board[0]),
        print_tile(board[1]),
        print_tile(board[2])
    );
    println!("----------                   ----------");
    println!(
        "{} | {} | {}              Key:  4 | 5 | 6",
        print_tile(board[3]),
        print_tile(board[4]),
        print_tile(board[5])
    );
    println!("----------                   ----------");
    println!(
        "{} | {} | {}                    7 | 8 | 9",
        print_tile(board[6]),
        print_tile(board[7]),
        print_tile(board[8])
    );
}

fn win_check(board: &mut [Tile; 9]) -> i32 {
    // hardcoded all possible winning positions
    // returns 0 if nothing, 1 if someone won, and 2 if its a tie
    if (board[0] == Tile::X && board[1] == Tile::X && board[2] == Tile::X)
        || (board[3] == Tile::X && board[4] == Tile::X && board[5] == Tile::X)
        || (board[6] == Tile::X && board[7] == Tile::X && board[8] == Tile::X)
        || (board[0] == Tile::X && board[3] == Tile::X && board[6] == Tile::X)
        || (board[1] == Tile::X && board[4] == Tile::X && board[7] == Tile::X)
        || (board[2] == Tile::X && board[5] == Tile::X && board[8] == Tile::X)
        || (board[0] == Tile::X && board[4] == Tile::X && board[8] == Tile::X)
        || (board[2] == Tile::X && board[4] == Tile::X && board[6] == Tile::X)
    {
        return 1;
    }

    if board[0] == Tile::O && board[1] == Tile::O && board[2] == Tile::O
        || (board[3] == Tile::O && board[4] == Tile::O && board[5] == Tile::O)
        || (board[6] == Tile::O && board[7] == Tile::O && board[8] == Tile::O)
        || (board[0] == Tile::O && board[3] == Tile::O && board[6] == Tile::O)
        || (board[1] == Tile::O && board[4] == Tile::O && board[7] == Tile::O)
        || (board[2] == Tile::O && board[5] == Tile::O && board[8] == Tile::O)
        || (board[0] == Tile::O && board[4] == Tile::O && board[8] == Tile::O)
        || (board[2] == Tile::O && board[4] == Tile::O && board[6] == Tile::O)
    {
        return 1;
    }

    // checks 0 thru 8 board pieces, and if one of them is empty, it breaks
    // therefore, if all of them are full, it'll return 2 for tie.
    for i in 0..9 {
        if board[i] == Tile::E {
            return 0;
        }
    }
    return 2;
}

// this is the two player code
fn turn(isx: bool, board: &mut [Tile; 9]) -> usize {
    loop {
        let mut input = String::new();
        if isx == true {
            println!("player 1: what is your move?");
        } else {
            println!("player 2: what is your move?");
        };
        std::io::stdin().read_line(&mut input).unwrap();
        // remove newline character and turn into an integer from string
        let mut input_int: usize = input.trim().parse().unwrap();
        // off by one error prevention
        input_int = input_int - 1;
        if board[input_int] == Tile::E {
            return input_int;
        } else {
            println!("someone has already gone there!");
            thread::sleep(time::Duration::from_secs(1));
            clear();
            print_board(board);
        }
    }
}

fn twoplayer() {
    // gen board
    let mut board: [Tile; 9] = [Tile::E; 9];
    // player input
    clear();
    println!("1 | 2 | 3");
    println!("----------");
    println!("4 | 5 | 6");
    println!("----------");
    println!("7 | 8 | 9");
    loop {
        // x's turn
        board[turn(true, &mut board)] = Tile::X;
        clear();
        print_board(&mut board);
        let win = win_check(&mut board);
        if win == 1 {
            println!("X Wins!");
            break;
        } else if win == 2 {
            println!("Tie!");
            break;
        }
        // o's turn
        board[turn(false, &mut board)] = Tile::O;
        clear();
        print_board(&mut board);
        let win = win_check(&mut board);
        if win == 1 {
            println!("O wins!");
            break;
        } else if win == 2 {
            println!("Tie!");
            break;
        }
    }
}

fn go_two_os(board: &mut [Tile; 9]) -> (usize, bool) {
    if (board[1] == Tile::O && board[2] == Tile::O && board[0] == Tile::E)
        || (board[3] == Tile::O && board[6] == Tile::O && board[0] == Tile::E)
        || (board[4] == Tile::O && board[8] == Tile::O && board[0] == Tile::E)
    {
        return (0, true);
    }
    if (board[0] == Tile::O && board[2] == Tile::O && board[1] == Tile::E)
        || (board[7] == Tile::O && board[4] == Tile::O && board[1] == Tile::E)
    {
        return (1, true);
    }
    if (board[0] == Tile::O && board[1] == Tile::O && board[2] == Tile::E)
        || (board[8] == Tile::O && board[5] == Tile::O && board[2] == Tile::E)
        || (board[4] == Tile::O && board[6] == Tile::O && board[2] == Tile::E)
    {
        return (2, true);
    }
    if (board[0] == Tile::O && board[6] == Tile::O && board[3] == Tile::E)
        || (board[4] == Tile::O && board[5] == Tile::O && board[3] == Tile::E)
    {
        return (3, true);
    }
    if (board[0] == Tile::O && board[8] == Tile::O && board[4] == Tile::E)
        || (board[1] == Tile::O && board[7] == Tile::O && board[4] == Tile::E)
        || (board[2] == Tile::O && board[6] == Tile::O && board[4] == Tile::E)
        || (board[3] == Tile::O && board[5] == Tile::O && board[4] == Tile::E)
    {
        return (4, true);
    }
    if (board[3] == Tile::O && board[4] == Tile::O && board[5] == Tile::E)
        || (board[2] == Tile::O && board[8] == Tile::O && board[5] == Tile::E)
    {
        return (5, true);
    }
    if (board[7] == Tile::O && board[8] == Tile::O && board[6] == Tile::E)
        || (board[0] == Tile::O && board[3] == Tile::O && board[6] == Tile::E)
        || (board[2] == Tile::O && board[4] == Tile::O && board[6] == Tile::E)
    {
        return (6, true);
    }
    if (board[6] == Tile::O && board[8] == Tile::O && board[7] == Tile::E)
        || (board[1] == Tile::O && board[4] == Tile::O && board[7] == Tile::E)
    {
        return (7, true);
    }
    if (board[6] == Tile::O && board[7] == Tile::O && board[8] == Tile::E)
        || (board[2] == Tile::O && board[5] == Tile::O && board[8] == Tile::E)
        || (board[0] == Tile::O && board[4] == Tile::O && board[8] == Tile::E)
    {
        return (8, true);
    }
    return (0, false);
}

fn go_two_xs(board: &mut [Tile; 9]) -> (usize, bool) {
    if (board[1] == Tile::X && board[2] == Tile::X && board[0] == Tile::E)
        || (board[4] == Tile::X && board[8] == Tile::X && board[0] == Tile::E)
        || (board[3] == Tile::X && board[6] == Tile::X && board[0] == Tile::E)
    {
        return (0, true);
    }
    if (board[0] == Tile::X && board[2] == Tile::X && board[1] == Tile::E)
        || (board[7] == Tile::X && board[4] == Tile::X && board[1] == Tile::E)
    {
        return (1, true);
    }
    if (board[0] == Tile::X && board[1] == Tile::X && board[2] == Tile::E)
        || (board[8] == Tile::X && board[5] == Tile::X && board[2] == Tile::E)
        || (board[4] == Tile::X && board[6] == Tile::X && board[2] == Tile::E)
    {
        return (2, true);
    }
    if (board[4] == Tile::X && board[5] == Tile::X && board[3] == Tile::E)
        || (board[0] == Tile::X && board[6] == Tile::X && board[3] == Tile::E)
    {
        return (3, true);
    }
    if (board[3] == Tile::X && board[5] == Tile::X && board[4] == Tile::E)
        || (board[2] == Tile::X && board[6] == Tile::X && board[4] == Tile::E)
        || (board[0] == Tile::X && board[8] == Tile::X && board[4] == Tile::E)
        || (board[1] == Tile::X && board[7] == Tile::X && board[4] == Tile::E)
    {
        return (4, true);
    }
    if (board[2] == Tile::X && board[8] == Tile::X && board[5] == Tile::E)
        || (board[3] == Tile::X && board[4] == Tile::X && board[5] == Tile::E)
    {
        return (5, true);
    }
    if (board[7] == Tile::X && board[8] == Tile::X && board[6] == Tile::E)
        || (board[2] == Tile::X && board[4] == Tile::X && board[6] == Tile::E)
        || (board[0] == Tile::X && board[3] == Tile::X && board[6] == Tile::E)
    {
        return (6, true);
    }
    if (board[6] == Tile::X && board[8] == Tile::X && board[7] == Tile::E)
        || (board[1] == Tile::X && board[4] == Tile::X && board[7] == Tile::E)
    {
        return (7, true);
    }
    if (board[6] == Tile::X && board[7] == Tile::X && board[8] == Tile::E)
        || (board[2] == Tile::X && board[5] == Tile::X && board[8] == Tile::E)
        || (board[0] == Tile::X && board[4] == Tile::X && board[8] == Tile::E)
    {
        return (8, true);
    }
    return (0, false);
}

fn go_rand_norm_edge(board: &mut [Tile; 9]) -> (usize, bool) {
    let rand = thread_rng().gen_range(1, 6);
    match rand {
        1 => {
            if board[1] == Tile::E {
                return (1, true);
            }
        }
        2 => {
            if board[3] == Tile::E {
                return (3, true);
            }
        }
        3 => {
            if board[4] == Tile::E {
                return (4, true);
            }
        }
        4 => {
            if board[5] == Tile::E {
                return (5, true);
            }
        }
        5 => {
            if board[7] == Tile::E {
                return (7, true);
            }
        }
        _ => unreachable!(),
    }
    return (0, false);
}

fn go_two_xs_nonconsecutive(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        if (board[0] == Tile::X && board[8] == Tile::X && board[4] == Tile::O)
            || (board[2] == Tile::X && board[6] == Tile::X && board[4] == Tile::O)
            || (board[0] == Tile::X && board[6] == Tile::X && board[4] == Tile::O)
            || (board[8] == Tile::X && board[7] == Tile::X && board[4] == Tile::O)
            || (board[2] == Tile::X && board[0] == Tile::X && board[4] == Tile::O)
            || (board[2] == Tile::X && board[8] == Tile::X && board[4] == Tile::O)
        {
            let norm_edge = go_rand_norm_edge(board);
            if norm_edge.1 == true {
                return (norm_edge.0, true);
            }
        }
    }
    return (0, false);
}

fn go_middle(board: &mut [Tile; 9]) -> (usize, bool) {
    if board[4] == Tile::E {
        return (4, true);
    }
    return (0, false);
}

fn go_randcorner_w_bordering_edge(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        let rand = thread_rng().gen_range(1, 5);
        match rand {
            1 => {
                if board[0] == Tile::E && board[3] == Tile::X && board[1] == Tile::X {
                    return (0, true);
                }
            }
            2 => {
                if board[2] == Tile::E && board[1] == Tile::X && board[5] == Tile::X {
                    return (2, true);
                }
            }
            3 => {
                if board[6] == Tile::E && board[3] == Tile::X && board[7] == Tile::X {
                    return (6, true);
                }
            }
            4 => {
                if board[8] == Tile::E && board[5] == Tile::X && board[7] == Tile::X {
                    return (9, true);
                }
            }
            _ => unreachable!(),
        }
    }
    return (0, false);
}

fn go_rand_norm_corner(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        let rand = thread_rng().gen_range(1, 5);
        match rand {
            1 => {
                if board[0] == Tile::E {
                    return (0, true);
                }
            }
            2 => {
                if board[2] == Tile::E {
                    return (2, true);
                }
            }
            3 => {
                if board[6] == Tile::E {
                    return (6, true);
                }
            }
            4 => {
                if board[8] == Tile::E {
                    return (8, true);
                }
            }
            _ => unreachable!(),
        }
    }
    return (0, false);
}

fn go_rand_corner_w_bordering_corner(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        let rand = thread_rng().gen_range(1, 5);
        match rand {
            1 => {
                if (board[2] == Tile::X && board[0] == Tile::E)
                    || (board[6] == Tile::X && board[0] == Tile::E)
                {
                    return (0, true);
                }
            }
            2 => {
                if (board[0] == Tile::X && board[2] == Tile::E)
                    || (board[8] == Tile::X && board[2] == Tile::E)
                {
                    return (2, true);
                }
            }
            3 => {
                if (board[0] == Tile::X && board[6] == Tile::E)
                    || (board[8] == Tile::X && board[6] == Tile::E)
                {
                    return (6, true);
                }
            }
            4 => {
                if (board[2] == Tile::X && board[8] == Tile::E)
                    || (board[6] == Tile::X && board[8] == Tile::E)
                {
                    return (8, true);
                }
            }
            _ => println!("uh oh, numbers broke."),
        }
    }
    return (0, false);
}

fn go_complete_random(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        let rand = thread_rng().gen_range(0, 9);
        match rand {
            0 => {
                if board[0] == Tile::E {
                    return (0, true);
                }
            }
            1 => {
                if board[1] == Tile::E {
                    return (1, true);
                }
            }
            2 => {
                if board[2] == Tile::E {
                    return (2, true);
                }
            }
            3 => {
                if board[3] == Tile::E {
                    return (3, true);
                }
            }
            4 => {
                if board[4] == Tile::E {
                    return (4, true);
                }
            }
            5 => {
                if board[5] == Tile::E {
                    return (5, true);
                }
            }
            6 => {
                if board[6] == Tile::E {
                    return (6, true);
                }
            }
            7 => {
                if board[7] == Tile::E {
                    return (7, true);
                }
            }
            8 => {
                if board[8] == Tile::E {
                    return (8, true);
                }
            }
            _ => unreachable!(),
        }
    }
    return (0, false);
}

fn computer_diff_gen(diffgen: i32) -> bool {
    if diffgen == 100 {
        return true;
    } else {
        let rand = thread_rng().gen_range(0, 101);
        if rand >= diffgen {
            return false;
        } else {
            return true;
        }
    }
}

fn computer_turn(board: &mut [Tile; 9], diffcomp: i32) -> (usize, bool) {
    // make it actually do like go_complete_random or the following using the pattern etc
    let compdiffgen = computer_diff_gen(diffcomp);
    if compdiffgen == true {
        let two_os = go_two_os(board);
        if two_os.1 == true {
            return (two_os.0, true);
        }
        let two_xs = go_two_xs(board);
        if two_xs.1 == true {
            return (two_xs.0, true);
        }
        let two_xs_nonconsecutive = go_two_xs_nonconsecutive(board);
        if two_xs_nonconsecutive.1 == true {
            return (two_xs_nonconsecutive.0, true);
        }
        let middle = go_middle(board);
        if middle.1 == true {
            return (middle.0, true);
        }
        let randcorner_w_bordering_edge = go_randcorner_w_bordering_edge(board);
        if randcorner_w_bordering_edge.1 == true {
            return (randcorner_w_bordering_edge.0, true);
        }
        let rand_norm_corner = go_rand_norm_corner(board);
        if rand_norm_corner.1 == true {
            return (rand_norm_corner.0, true);
        }
        let rand_norm_edge = go_rand_norm_edge(board);
        if rand_norm_edge.1 == true {
            return (rand_norm_edge.0, true);
        }
        let rand_corner_w_bordering_corner = go_rand_corner_w_bordering_corner(board);
        if rand_corner_w_bordering_corner.1 == true {
            return (rand_corner_w_bordering_corner.0, true);
        }
        let complete_random = go_complete_random(board);
        if complete_random.1 == true {
            return (complete_random.0, true);
        }
    } else {
        let complete_random = go_complete_random(board);
        if complete_random.1 == true {
            return (complete_random.0, true);
        }
    }
    return (0, false);
}
fn computer() {
    let now = time::Instant::now();
    // gen board
    let mut board: [Tile; 9] = [Tile::E; 9];
    clear();
    // difficulty
    println!(
        "what do you want the difficulty to be? 0-100, where 0 is easiest and 100 is impossible"
    );
    let mut difficulty = String::new();
    std::io::stdin().read_line(&mut difficulty).unwrap();
    // remove newline character and turn into an integer from string
    let difficulty_int: usize = difficulty.trim().parse().unwrap();
    clear();
    println!("1 | 2 | 3");
    println!("----------");
    println!("4 | 5 | 6");
    println!("----------");
    println!("7 | 8 | 9");
    loop {
        // Human's turn
        board[turn(true, &mut board)] = Tile::X;
        clear();
        print_board(&mut board);
        let win = win_check(&mut board);
        if win == 1 {
            println!("X Wins!");
            println!("Difficulty was: {}", difficulty_int);
            println!("Time: {} seconds", now.elapsed().as_secs());
            break;
        } else if win == 2 {
            println!("Tie!");
            println!("Difficulty was: {}", difficulty_int);
            println!("Time: {} seconds", now.elapsed().as_secs());
            break;
        }
        // computer turn
        println!("thinking...");
        // wait one second
        thread::sleep(time::Duration::from_secs(1));
        clear();
        let comp_turn = computer_turn(&mut board, difficulty_int.try_into().unwrap());
        if comp_turn.1 == true {
            board[comp_turn.0] = Tile::O;
        }
        print_board(&mut board);
        let win = win_check(&mut board);
        if win == 1 {
            println!("O wins!");
            println!("Difficulty was: {}", difficulty_int);
            println!("Time: {} seconds", now.elapsed().as_secs());
            break;
        } else if win == 2 {
            println!("Tie!");
            println!("Difficulty was: {}", difficulty_int);
            println!("Time: {} seconds", now.elapsed().as_secs());
            break;
        }
    }
}

fn main() {
    clear();
    println!("would you like to play two player or against the computer?");
    println!("'one' for single player, 'two' for two player");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    choice = choice.trim().to_string();
    if choice == "one" {
        computer();
    } else if choice == "two" {
        twoplayer()
    } else {
        println!("Please enter 'one' or 'two'.");
    }
}
