pub type Bitboard = u64;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Square {
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A0, B0, C0, D0, E0, F0, G0, H0,
}

pub const EMPTY: Bitboard = 0;


// minimal bb_print with rank and file label
pub fn bb_print(bb: Bitboard) {
    for rank in (0..8).rev() {
        print!("{} \t", rank + 1);
        for file in 0..8 {
            let i = rank * 8 + file;
            let bit = (bb >> i) & 1;
            print!("{} ", bit);
        }
        println!();
    }

    // adding vertical padding
    println!();
    print!("    ");

    for f in b'A'..=b'H' {
        print!("{} ", f as char);
    }
}
