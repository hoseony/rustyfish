use fish::board::bitboard::*; // import the FISH!!!
use fish::board::board::*;
// use fish::board::attacks::*;
use fish::board::moves::*;

fn main() {
/*
    let mut bb = Bitboard::EMPTY;
    let bbb = Bitboard::EMPTY;
    
    // testing basic bitboard
    bb.set_bit(28);
    println!("{}", bb);

    bb.remove_bit(28);
    println!("{}", bb);
*/
    // testing basic board 
    let mut board = Board::initialize_board();

//    println!("{}", board.pieces[0][0]);
//    println!("{}", board);
/*    
    for sq in 0..64u8 {
        println!("Square {}: ", sq);
        println!("{}", king_attacks(sq));
        println!();
    }
*/ 
    let moves = generate_pseudo_legal_moves(&board);
    let mut all_targets: u64 = 0;

    // moves are packed as a one u16 number
    // we need to unpack it

    for mv in &moves {
        all_targets |= 1u64 << (mv.to_sq());
    }

    println!("{}", Bitboard(all_targets));

    board.make_move(moves[0]);

    println!("{}", board);
}
