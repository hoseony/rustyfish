// use fish::board::bitboard::*; // import the FISH!!!
use fish::board::board::*;
// use fish::board::attacks::*;
// use fish::board::moves::*;

fn main() {
    /*
    let mut board = Board::initialize_board();
    let moves = generate_legal_moves(&board);

    println!("{}", board);

    Board::make_move(&mut board, moves[0]);
    println!("{}", board);
    */

    let board = Board::initialize_board();

    for depth in 1..=4 {
        let nodes = perft(&board, depth);
        println!("Depth {} : {}", depth, nodes);
    }
}
