use fish::board::board::*;
use fish::board::parse::*;
use std::io::{self, Write};

fn main() {
    /*
    let mut board = Board::initialize_board();
    let moves = generate_legal_moves(&board);

    println!("{}", board);

    Board::make_move(&mut board, moves[0]);
    println!("{}", board);
    */

    let mut board = Board::initialize_board();

    loop {
        println!("{}", board);
        println!("--- Side to move: {:?} ---", board.side_to_move);

        let legal_moves = generate_legal_moves(&board);
        if legal_moves.is_empty() {
            println!("game over");
            break;
        }

        print!("Make your Move: ");
        let mut s: String = String::new(); // flush print!

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut s).expect("failed to read input");
        let input = s.trim();

        if input == "quit" {
            break;
        }

        if input == "divide" {
            perft_divide(&board, 3);
            continue;
        }

        match parse_move_input(input, &legal_moves) {
            Some(mv) => board.make_move(mv),
            None => println!("illegal or unknown move: {input}"),
        }
        println!();
    }
}
