use fish::board::bitboard::*; // import the FISH!!!

fn main() {
    let mut bb = Bitboard::EMPTY;

    bb.set_bit(28);
    println!("{}", bb);
}
