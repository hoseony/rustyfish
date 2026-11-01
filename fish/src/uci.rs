use crate::board::board::*;
use crate::board::parse::*;
use crate::board::moves::*;
use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run() {
    let mut board = Board::initialize_board();
    let mut rng = seed();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line");
        let token: Vec<&str> = line.split_whitespace().collect();

        match token[0] {
            "quit" => break,
            "uci"  => {
                println!("engine: rusty fish");
                println!("uciok")
            }
            "ucinewgame" => board = Board::initialize_board(),
            "isready" => println!("readyok"),
            "position" => board = parse_position(&token),
            "go" => {
                let mv = random_move(&board, &mut rng);
                println!("bestmove {}", mv.to_uci());
            }
            _ => eprintln!("unknown command: {line}"),
        }

        io::stdout().flush().unwrap();
    }
}

fn seed() -> u64 {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    nanos | 1
}

fn next_random(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

fn random_move(board: &Board, rng: &mut u64) -> Move {
    let moves = generate_legal_moves(board);
    let i = (next_random(rng) % moves.len() as u64) as usize;
    moves[i]
}

/// parse token in to correct board
fn parse_position(token: &[&str]) -> Board {
    /* The input format of the "token" is as such:
     *  - position startpos 
     *  - position startpos moves e2e4 e7e5
     *  - position fen ....... 
     *  - position fen ....... moves ...   
     */

    let index = token.iter().position(|&r| r == "moves");

    // it is either startpos or fen
    let mut board = match token.get(1) {
        Some(&"startpos") => Board::initialize_board(),
        Some(&"fen") => {
            let end = index.unwrap_or_else(|| token.len());
            let fen = token[2..end].join("");
            Board::from_fen(&fen)
        }
        _ => {
            eprintln!("bad position command");
            Board::initialize_board()
        }
    };

    // actually I will just make it "play" the move
    if let Some(i) = index {
        for mv_str in &token[i + 1..] {
            let legal = generate_legal_moves(&board);
            match parse_move_input(mv_str, &legal) {
                Some(mv) => board.make_move(mv),
                None => {
                    eprintln!("illegal move in position: {mv_str}");
                    break;
                }
            }
        }
    }

    board
}
