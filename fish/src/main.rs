// use fish::board::bitboard::*; // import the FISH!!!
use fish::board::board::*;
// use fish::board::attacks::*;
// use fish::board::moves::*;
/*
fn main() {
    let mut board = Board::initialize_board();
    let moves = generate_pseudo_legal_moves(&board);


    for i in 0..moves.len() {
        let mut test_board = Board::initialize_board();
        Board::make_move(&mut test_board, moves[i]);
        println!("{}", test_board);
    }

    board.side_to_move = board.side_to_move.opposite();
    let moves_2 = generate_pseudo_legal_moves(&board);

    for i in 0..moves_2.len() {
        let mut test_board = board;
        test_board.make_move(moves_2[i]);
        println!("{}", test_board);
    }

    println!("WHITE: Total pseudo-legal moves: {}", moves.len());
    println!("BLACK: Total pseudo-legal moves: {}", moves_2.len());
}
*/

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let board = Board::from_fen(fen);

    println!("{}", board);
}
