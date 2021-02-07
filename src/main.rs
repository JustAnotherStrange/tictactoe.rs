// TODO
// center on terminal using https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
// add stats after winning

// for random
use rand::Rng;
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
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
fn print_tile(t: Tile) -> char {
    return match t {
        Tile::X => 'X',
        Tile::O => 'O',
        _ => '-',
    };
}

fn print_board(board_nice: &mut [Tile; 9]) {
    println!(
        "{} | {} | {}                    1 | 2 | 3",
        print_tile(board_nice[0]),
        print_tile(board_nice[1]),
        print_tile(board_nice[2])
    );
    println!("----------                   ----------");
    println!(
        "{} | {} | {}              Key:  4 | 5 | 6",
        print_tile(board_nice[3]),
        print_tile(board_nice[4]),
        print_tile(board_nice[5])
    );
    println!("----------                   ----------");
    println!(
        "{} | {} | {}                    7 | 8 | 9",
        print_tile(board_nice[6]),
        print_tile(board_nice[7]),
        print_tile(board_nice[8])
    );
}

fn win_check(board_win: &mut [Tile; 9]) -> i32 {
    if (board_win[0] == Tile::X && board_win[1] == Tile::X && board_win[2] == Tile::X)
        || (board_win[3] == Tile::X && board_win[4] == Tile::X && board_win[5] == Tile::X)
        || (board_win[6] == Tile::X && board_win[7] == Tile::X && board_win[8] == Tile::X)
        || (board_win[0] == Tile::X && board_win[3] == Tile::X && board_win[6] == Tile::X)
        || (board_win[1] == Tile::X && board_win[4] == Tile::X && board_win[7] == Tile::X)
        || (board_win[2] == Tile::X && board_win[5] == Tile::X && board_win[8] == Tile::X)
        || (board_win[0] == Tile::X && board_win[4] == Tile::X && board_win[8] == Tile::X)
        || (board_win[2] == Tile::X && board_win[4] == Tile::X && board_win[6] == Tile::X)
    {
        return 1;
    }

    if board_win[0] == Tile::O && board_win[1] == Tile::O && board_win[2] == Tile::O
        || (board_win[3] == Tile::O && board_win[4] == Tile::O && board_win[5] == Tile::O)
        || (board_win[6] == Tile::O && board_win[7] == Tile::O && board_win[8] == Tile::O)
        || (board_win[0] == Tile::O && board_win[3] == Tile::O && board_win[6] == Tile::O)
        || (board_win[1] == Tile::O && board_win[4] == Tile::O && board_win[7] == Tile::O)
        || (board_win[2] == Tile::O && board_win[5] == Tile::O && board_win[8] == Tile::O)
        || (board_win[0] == Tile::O && board_win[4] == Tile::O && board_win[8] == Tile::O)
        || (board_win[2] == Tile::O && board_win[4] == Tile::O && board_win[6] == Tile::O)
    {
        return 1;
    }

    if board_win[0] != Tile::E
        && board_win[1] != Tile::E
        && board_win[3] != Tile::E
        && board_win[4] != Tile::E
        && board_win[5] != Tile::E
        && board_win[6] != Tile::E
        && board_win[6] != Tile::E
        && board_win[7] != Tile::E
        && board_win[8] != Tile::E
    {
        return 2;
    }
    return 0;
}

fn print_stats(diffstat: usize) {
    println!("Difficulty was: {}", diffstat);
}

fn turn(isx: bool, board_turn: &mut [Tile; 9]) -> usize {
    loop {
        let mut input = String::new();
        if isx == true {
            println!("player 1: what is your move?");
        } else {
            println!("player 2: what is your move?");
        };
        std::io::stdin().read_line(&mut input).unwrap();
        // remove newline character and turn into an integer from string
        let len = input.len();
        input.truncate(len - 1);
        let mut input_int: usize = input.parse().unwrap();
        // off by one error prevention
        input_int = input_int - 1;
        if board_turn[input_int] == Tile::E {
            return input_int;
        } else {
            println!("someone has already gone there!");
            thread::sleep(time::Duration::from_secs(1));
            clear();
            print_board(board_turn);
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

fn go_two_os(board_two_os: &mut [Tile; 9]) -> (usize, bool) {
    if (board_two_os[1] == Tile::O && board_two_os[2] == Tile::O && board_two_os[0] == Tile::E)
        || (board_two_os[3] == Tile::O && board_two_os[6] == Tile::O && board_two_os[0] == Tile::E)
        || (board_two_os[4] == Tile::O && board_two_os[8] == Tile::O && board_two_os[0] == Tile::E)
    {
        return (0, true);
    }
    if (board_two_os[0] == Tile::O && board_two_os[2] == Tile::O && board_two_os[1] == Tile::E)
        || (board_two_os[7] == Tile::O && board_two_os[4] == Tile::O && board_two_os[1] == Tile::E)
    {
        return (1, true);
    }
    if (board_two_os[0] == Tile::O && board_two_os[1] == Tile::O && board_two_os[2] == Tile::E)
        || (board_two_os[8] == Tile::O && board_two_os[5] == Tile::O && board_two_os[2] == Tile::E)
        || (board_two_os[4] == Tile::O && board_two_os[6] == Tile::O && board_two_os[2] == Tile::E)
    {
        return (2, true);
    }
    if (board_two_os[0] == Tile::O && board_two_os[6] == Tile::O && board_two_os[3] == Tile::E)
        || (board_two_os[4] == Tile::O && board_two_os[5] == Tile::O && board_two_os[3] == Tile::E)
    {
        return (3, true);
    }
    if (board_two_os[0] == Tile::O && board_two_os[8] == Tile::O && board_two_os[4] == Tile::E)
        || (board_two_os[1] == Tile::O && board_two_os[7] == Tile::O && board_two_os[4] == Tile::E)
        || (board_two_os[2] == Tile::O && board_two_os[6] == Tile::O && board_two_os[4] == Tile::E)
        || (board_two_os[3] == Tile::O && board_two_os[5] == Tile::O && board_two_os[4] == Tile::E)
    {
        return (4, true);
    }
    if (board_two_os[3] == Tile::O && board_two_os[4] == Tile::O && board_two_os[5] == Tile::E)
        || (board_two_os[2] == Tile::O && board_two_os[8] == Tile::O && board_two_os[5] == Tile::E)
    {
        return (5, true);
    }
    if (board_two_os[7] == Tile::O && board_two_os[8] == Tile::O && board_two_os[6] == Tile::E)
        || (board_two_os[0] == Tile::O && board_two_os[3] == Tile::O && board_two_os[6] == Tile::E)
        || (board_two_os[2] == Tile::O && board_two_os[4] == Tile::O && board_two_os[6] == Tile::E)
    {
        return (6, true);
    }
    if (board_two_os[6] == Tile::O && board_two_os[8] == Tile::O && board_two_os[7] == Tile::E)
        || (board_two_os[1] == Tile::O && board_two_os[4] == Tile::O && board_two_os[7] == Tile::E)
    {
        return (7, true);
    }
    if (board_two_os[6] == Tile::O && board_two_os[7] == Tile::O && board_two_os[8] == Tile::E)
        || (board_two_os[2] == Tile::O && board_two_os[5] == Tile::O && board_two_os[8] == Tile::E)
        || (board_two_os[0] == Tile::O && board_two_os[4] == Tile::O && board_two_os[8] == Tile::E)
    {
        return (8, true);
    }
    return (0, false);
}

fn go_two_xs(board_two_xs: &mut [Tile; 9]) -> (usize, bool) {
    if (board_two_xs[1] == Tile::X && board_two_xs[2] == Tile::X && board_two_xs[0] == Tile::E)
        || (board_two_xs[4] == Tile::X && board_two_xs[8] == Tile::X && board_two_xs[0] == Tile::E)
        || (board_two_xs[3] == Tile::X && board_two_xs[6] == Tile::X && board_two_xs[0] == Tile::E)
    {
        return (0, true);
    }
    if (board_two_xs[0] == Tile::X && board_two_xs[2] == Tile::X && board_two_xs[1] == Tile::E)
        || (board_two_xs[7] == Tile::X && board_two_xs[4] == Tile::X && board_two_xs[1] == Tile::E)
    {
        return (1, true);
    }
    if (board_two_xs[0] == Tile::X && board_two_xs[1] == Tile::X && board_two_xs[2] == Tile::E)
        || (board_two_xs[8] == Tile::X && board_two_xs[5] == Tile::X && board_two_xs[2] == Tile::E)
        || (board_two_xs[4] == Tile::X && board_two_xs[6] == Tile::X && board_two_xs[2] == Tile::E)
    {
        return (2, true);
    }
    if (board_two_xs[4] == Tile::X && board_two_xs[5] == Tile::X && board_two_xs[3] == Tile::E)
        || (board_two_xs[0] == Tile::X && board_two_xs[6] == Tile::X && board_two_xs[3] == Tile::E)
    {
        return (3, true);
    }
    if (board_two_xs[3] == Tile::X && board_two_xs[5] == Tile::X && board_two_xs[4] == Tile::E)
        || (board_two_xs[2] == Tile::X && board_two_xs[6] == Tile::X && board_two_xs[4] == Tile::E)
        || (board_two_xs[0] == Tile::X && board_two_xs[8] == Tile::X && board_two_xs[4] == Tile::E)
        || (board_two_xs[1] == Tile::X && board_two_xs[7] == Tile::X && board_two_xs[4] == Tile::E)
    {
        return (4, true);
    }
    if (board_two_xs[2] == Tile::X && board_two_xs[8] == Tile::X && board_two_xs[5] == Tile::E)
        || (board_two_xs[3] == Tile::X && board_two_xs[4] == Tile::X && board_two_xs[5] == Tile::E)
    {
        return (5, true);
    }
    if (board_two_xs[7] == Tile::X && board_two_xs[8] == Tile::X && board_two_xs[6] == Tile::E)
        || (board_two_xs[2] == Tile::X && board_two_xs[4] == Tile::X && board_two_xs[6] == Tile::E)
        || (board_two_xs[0] == Tile::X && board_two_xs[3] == Tile::X && board_two_xs[6] == Tile::E)
    {
        return (6, true);
    }
    if (board_two_xs[6] == Tile::X && board_two_xs[8] == Tile::X && board_two_xs[7] == Tile::E)
        || (board_two_xs[1] == Tile::X && board_two_xs[4] == Tile::X && board_two_xs[7] == Tile::E)
    {
        return (7, true);
    }
    if (board_two_xs[6] == Tile::X && board_two_xs[7] == Tile::X && board_two_xs[8] == Tile::E)
        || (board_two_xs[2] == Tile::X && board_two_xs[5] == Tile::X && board_two_xs[8] == Tile::E)
        || (board_two_xs[0] == Tile::X && board_two_xs[4] == Tile::X && board_two_xs[8] == Tile::E)
    {
        return (8, true);
    }
    return (0, false);
}

fn go_rand_norm_edge(board_rand_norm_edge: &mut [Tile; 9]) -> (usize, bool) {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(1, 6);
    match rand {
        1 => {
            if board_rand_norm_edge[1] == Tile::E {
                return (1, true);
            }
        }
        2 => {
            if board_rand_norm_edge[3] == Tile::E {
                return (3, true);
            }
        }
        3 => {
            if board_rand_norm_edge[4] == Tile::E {
                return (4, true);
            }
        }
        4 => {
            if board_rand_norm_edge[5] == Tile::E {
                return (5, true);
            }
        }
        5 => {
            if board_rand_norm_edge[7] == Tile::E {
                return (7, true);
            }
        }
        _ => println!("uh, numbers have broken."),
    }
    return (0, false);
}

fn go_two_xs_nonconsecutive(board_two_xs_nonconsecutive: &mut [Tile; 9]) -> (usize, bool) {
    for _i in 1..101 {
        if (board_two_xs_nonconsecutive[0] == Tile::X
            && board_two_xs_nonconsecutive[8] == Tile::X
            && board_two_xs_nonconsecutive[4] == Tile::O)
            || (board_two_xs_nonconsecutive[2] == Tile::X
                && board_two_xs_nonconsecutive[6] == Tile::X
                && board_two_xs_nonconsecutive[4] == Tile::O)
            || (board_two_xs_nonconsecutive[0] == Tile::X
                && board_two_xs_nonconsecutive[6] == Tile::X
                && board_two_xs_nonconsecutive[4] == Tile::O)
            || (board_two_xs_nonconsecutive[8] == Tile::X
                && board_two_xs_nonconsecutive[7] == Tile::X
                && board_two_xs_nonconsecutive[4] == Tile::O)
            || (board_two_xs_nonconsecutive[2] == Tile::X
                && board_two_xs_nonconsecutive[0] == Tile::X
                && board_two_xs_nonconsecutive[4] == Tile::O)
            || (board_two_xs_nonconsecutive[2] == Tile::X
                && board_two_xs_nonconsecutive[8] == Tile::X
                && board_two_xs_nonconsecutive[4] == Tile::O)
        {
            let norm_edge = go_rand_norm_edge(board_two_xs_nonconsecutive);
            if norm_edge.1 == true {
                return (norm_edge.0, true);
            }
        }
    }
    return (0, false);
}

fn go_middle(board_middle: &mut [Tile; 9]) -> (usize, bool) {
    if board_middle[4] == Tile::E {
        return (4, true);
    }
    return (0, false);
}

fn go_randcorner_w_bordering_edge(
    board_randcorner_w_bordering_edge: &mut [Tile; 9],
) -> (usize, bool) {
    let mut rng = rand::thread_rng();
    for _i in 1..101 {
        let rand = rng.gen_range(1, 5);
        match rand {
            1 => {
                if board_randcorner_w_bordering_edge[0] == Tile::E
                    && board_randcorner_w_bordering_edge[3] == Tile::X
                    && board_randcorner_w_bordering_edge[1] == Tile::X
                {
                    return (0, true);
                }
            }
            2 => {
                if board_randcorner_w_bordering_edge[2] == Tile::E
                    && board_randcorner_w_bordering_edge[1] == Tile::X
                    && board_randcorner_w_bordering_edge[5] == Tile::X
                {
                    return (2, true);
                }
            }
            3 => {
                if board_randcorner_w_bordering_edge[6] == Tile::E
                    && board_randcorner_w_bordering_edge[3] == Tile::X
                    && board_randcorner_w_bordering_edge[7] == Tile::X
                {
                    return (6, true);
                }
            }
            4 => {
                if board_randcorner_w_bordering_edge[8] == Tile::E
                    && board_randcorner_w_bordering_edge[5] == Tile::X
                    && board_randcorner_w_bordering_edge[7] == Tile::X
                {
                    return (9, true);
                }
            }
            _ => println!("uh oh, numbers broke"),
        }
    }
    return (0, false);
}

fn go_rand_norm_corner(board_rand_norm_corner: &mut [Tile; 9]) -> (usize, bool) {
    let mut rng = rand::thread_rng();
    for _i in 1..101 {
        let rand = rng.gen_range(1, 5);
        match rand {
            1 => {
                if board_rand_norm_corner[0] == Tile::E {
                    return (0, true);
                }
            }
            2 => {
                if board_rand_norm_corner[2] == Tile::E {
                    return (2, true);
                }
            }
            3 => {
                if board_rand_norm_corner[6] == Tile::E {
                    return (6, true);
                }
            }
            4 => {
                if board_rand_norm_corner[8] == Tile::E {
                    return (8, true);
                }
            }
            _ => println!("uh oh, numbers broke."),
        }
    }
    return (0, false);
}

fn go_rand_corner_w_bordering_corner(
    board_rand_corner_w_bordering_corner: &mut [Tile; 9],
) -> (usize, bool) {
    let mut rng = rand::thread_rng();
    for _i in 1..101 {
        let rand = rng.gen_range(1, 5);
        match rand {
            1 => {
                if (board_rand_corner_w_bordering_corner[2] == Tile::X
                    && board_rand_corner_w_bordering_corner[0] == Tile::E)
                    || (board_rand_corner_w_bordering_corner[6] == Tile::X
                        && board_rand_corner_w_bordering_corner[0] == Tile::E)
                {
                    return (0, true);
                }
            }
            2 => {
                if (board_rand_corner_w_bordering_corner[0] == Tile::X
                    && board_rand_corner_w_bordering_corner[2] == Tile::E)
                    || (board_rand_corner_w_bordering_corner[8] == Tile::X
                        && board_rand_corner_w_bordering_corner[2] == Tile::E)
                {
                    return (2, true);
                }
            }
            3 => {
                if (board_rand_corner_w_bordering_corner[0] == Tile::X
                    && board_rand_corner_w_bordering_corner[6] == Tile::E)
                    || (board_rand_corner_w_bordering_corner[8] == Tile::X
                        && board_rand_corner_w_bordering_corner[6] == Tile::E)
                {
                    return (6, true);
                }
            }
            4 => {
                if (board_rand_corner_w_bordering_corner[2] == Tile::X
                    && board_rand_corner_w_bordering_corner[8] == Tile::E)
                    || (board_rand_corner_w_bordering_corner[6] == Tile::X
                        && board_rand_corner_w_bordering_corner[8] == Tile::E)
                {
                    return (8, true);
                }
            }
            _ => println!("uh oh, numbers broke."),
        }
    }
    return (0, false);
}

fn go_complete_random(board_complete_random: &mut [Tile; 9]) -> (usize, bool) {
    let mut rng = rand::thread_rng();
    for _i in 1..101 {
        let rand = rng.gen_range(0, 9);
        match rand {
            0 => {
                if board_complete_random[0] == Tile::E {
                    return (0, true);
                }
            }
            1 => {
                if board_complete_random[1] == Tile::E {
                    return (1, true);
                }
            }
            2 => {
                if board_complete_random[2] == Tile::E {
                    return (2, true);
                }
            }
            3 => {
                if board_complete_random[3] == Tile::E {
                    return (3, true);
                }
            }
            4 => {
                if board_complete_random[4] == Tile::E {
                    return (4, true);
                }
            }
            5 => {
                if board_complete_random[5] == Tile::E {
                    return (5, true);
                }
            }
            6 => {
                if board_complete_random[6] == Tile::E {
                    return (6, true);
                }
            }
            7 => {
                if board_complete_random[7] == Tile::E {
                    return (7, true);
                }
            }
            8 => {
                if board_complete_random[8] == Tile::E {
                    return (8, true);
                }
            }
            _ => println!("uh oh, numbers broke."),
        }
    }
    return (0, false);
}

fn computer_diff_gen(diffgen: i32) -> bool {
    if diffgen == 100 {
        return true;
    } else {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0, 101);
        if rand >= diffgen {
            return false;
        } else {
            return true;
        }
    }
}

fn computer_turn(board_turn: &mut [Tile; 9], diffcomp: i32) -> (usize, bool) {
    // make it actually do like go_complete_random or the following using the pattern etc
    let compdiffgen = computer_diff_gen(diffcomp);
    if compdiffgen == true {
        let two_os = go_two_os(board_turn);
        if two_os.1 == true {
            return (two_os.0, true);
        }
        let two_xs = go_two_xs(board_turn);
        if two_xs.1 == true {
            return (two_xs.0, true);
        }
        let two_xs_nonconsecutive = go_two_xs_nonconsecutive(board_turn);
        if two_xs_nonconsecutive.1 == true {
            return (two_xs_nonconsecutive.0, true);
        }
        let middle = go_middle(board_turn);
        if middle.1 == true {
            return (middle.0, true);
        }
        let randcorner_w_bordering_edge = go_randcorner_w_bordering_edge(board_turn);
        if randcorner_w_bordering_edge.1 == true {
            return (randcorner_w_bordering_edge.0, true);
        }
        let rand_norm_corner = go_rand_norm_corner(board_turn);
        if rand_norm_corner.1 == true {
            return (rand_norm_corner.0, true);
        }
        let rand_norm_edge = go_rand_norm_edge(board_turn);
        if rand_norm_edge.1 == true {
            return (rand_norm_edge.0, true);
        }
        let rand_corner_w_bordering_corner = go_rand_corner_w_bordering_corner(board_turn);
        if rand_corner_w_bordering_corner.1 == true {
            return (rand_corner_w_bordering_corner.0, true);
        }
        let complete_random = go_complete_random(board_turn);
        if complete_random.1 == true {
            return (complete_random.0, true);
        }
    } else {
        let complete_random = go_complete_random(board_turn);
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
    let len = difficulty.len();
    difficulty.truncate(len - 1);
    let difficulty_int: usize = difficulty.parse().unwrap();
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
            print_stats(difficulty_int);
            println!("Time: {} seconds", now.elapsed().as_secs());
            break;
        } else if win == 2 {
            println!("Tie!");
            print_stats(difficulty_int);
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
            print_stats(difficulty_int);
            println!("Time: {} seconds", now.elapsed().as_secs());
            break;
        } else if win == 2 {
            println!("Tie!");
            print_stats(difficulty_int);
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
    let len = choice.len();
    choice.truncate(len - 1);
    if choice == "one" {
        computer();
    } else if choice == "two" {
        twoplayer()
    }
}
