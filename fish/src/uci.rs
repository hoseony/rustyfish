use crate::board::board::*;
use crate::board::parse::*;
use crate::board::moves::*;
use crate::engine::search::*;
use crate::engine::tt::*;

use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH, Instant};

pub fn run() {
    let mut board = Board::initialize_board();
	let mut history: Vec<u64> = Vec::new();
	let mut tt = TT::new(1 << 20);

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line");
        let token: Vec<&str> = line.split_whitespace().collect();

        match token[0] {
            "quit" => break,
            "uci"  => {
                println!("id name rusty fish");
                println!("id author hoseon");
                println!("uciok")
            }
            "ucinewgame" => {
                board = Board::initialize_board();
                tt.clear();
            }
            "isready" => println!("readyok"),
            "position" => (board, history) = parse_position(&token),
            "go" => {
                // time left
				let my_time = get_value(&token, if board.side_to_move == Color::White { "wtime" } else { "btime" });
				let my_inc = get_value(&token, if board.side_to_move == Color::White { "winc" } else { "binc" });
				let budget_ms = (my_time / 30 + my_inc / 2).min(my_time / 2);

				// create time/search info
				let start = Instant::now();

				let mut info = SearchInfo {
    				deadline: start + std::time::Duration::from_millis(budget_ms),
    				stopped: false,
    				nodes: 0,
    				history: history.clone(),
				};

				let mut best: Option<Move> = None;

				// iterative deepening
				for depth in 1..=64 {
    				let (mv, score) = search_root(&board, depth, &mut info, &mut tt);
					if info.stopped {
    					if best.is_none() {
        					best = Some(mv);
    					}
    					break;
					}

    				let ms = start.elapsed().as_millis() as u64;
                    println!("info depth {depth} score cp {score} nodes {} time {ms} pv {}", info.nodes, mv.to_uci());
                    best = Some(mv);

					// next depth usually takes longer, so stop early
                    if start.elapsed().as_millis() as u64 > budget_ms / 2 {
                        break;
                    }
				}

                println!("bestmove {}", best.unwrap().to_uci());
            }

            _ => eprintln!("unknown command: {line}"),
        }

        io::stdout().flush().unwrap();
    }
}

fn get_value(token: &[&str], key: &str) -> u64 {
    token.iter()
        .position(|&t| t == key) // find the index of the key
        .and_then(|i| token.get(i + 1)) // get the next index
        .and_then(|v| v.parse().ok()) // turn it in to number
        .unwrap_or(0) // take the number or use 0
}


fn u64_random() -> u64 {
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
fn parse_position(token: &[&str]) -> (Board, Vec<u64>) {
    /* The input format of the "token" is as such:
     *  - position startpos
     *  - position startpos moves e2e4 e7e5
     *  - position fen ...
     *  - position fen ... moves ...
     */

    let index = token.iter().position(|&r| r == "moves");
    let mut history = Vec::new();

    // it is either startpos or fen
    let mut board = match token.get(1) {
        Some(&"startpos") => Board::initialize_board(),
        Some(&"fen") => {
            let end = index.unwrap_or(token.len());
            let fen = token[2..end].join(" ");
            Board::from_fen(&fen)
        }
        _ => {
            eprintln!("bad position command");
            Board::initialize_board()
        }
    };

    // actually "play" the move
    if let Some(i) = index {
        for mv_str in &token[i + 1..] {
            let legal = generate_legal_moves(&board);
            match parse_move_input(mv_str, &legal) {
                Some(mv) => {
                    history.push(board.hash);
                    board.make_move(mv);
                },
                None => {
                    eprintln!("illegal move in position: {mv_str}");
                    break;
                }
            }
        }
    }

    (board, history)
}
