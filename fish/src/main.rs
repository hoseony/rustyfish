use fish::board::bitboard::*; // import the FISH!!!
use fish::board::board::*;
use fish::board::attacks::*;


fn main() {
    let mut bb = Bitboard::EMPTY;
    let bbb = Bitboard::EMPTY;
/*
    // testing basic bitboard
    bb.set_bit(28);
    println!("{}", bb);

    bb.remove_bit(28);
    println!("{}", bb);
*/
    // testing basic board 
    let board = Board::initialize_board();

//    println!("{}", board.pieces[0][0]);
//    println!("{}", board);
    
    for sq in 0..64u8 {
        println!("Square {}: ", sq);
        println!("{}", king_attacks(sq));
        println!();
    }
}
