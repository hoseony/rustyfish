use crate::board::bitboard::*;

// basically translating and optimizing what I wrote 
// previously on my cfish

// cute little chess board!
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 1 0 1 0 0 0 0 0
// 0 0 0 1 0 0 0 0
// 0 1 0 0 0 0 0 0
// 0 0 0 1 0 0 0 0

pub fn knight_attacks(sq: u8) -> Bitboard {
    let mut attacks: u64 = 0;
    let bb: u64 = 1u64 << sq;

    attacks |= (bb << 17) & !FILE_A;
    attacks |= (bb << 15) & !FILE_H;
    attacks |= (bb << 10) & !(FILE_A | FILE_B);
    attacks |= (bb << 6)  & !(FILE_G | FILE_H);
    
    attacks |= (bb >> 17) & !FILE_H;
    attacks |= (bb >> 15) & !FILE_A;
    attacks |= (bb >> 10) & !(FILE_G | FILE_H);
    attacks |= (bb >> 6)  & !(FILE_A | FILE_B);
    
    Bitboard(attacks)
}

// hopefully I will implement magic for these sliding pieces
pub fn bishop_attacks(sq: u8, occupied: Bitboard) -> Bitboard {
    let mut attacks: u64 = 0;

    let file: i32 = (sq % 8) as i32;
    let rank: i32 = (sq / 8) as i32;

    // right up
    let (mut r, mut f) = (rank + 1, file + 1);
    while f < 8 && r < 8 {
        attacks |= (1u64) << (r * 8 + f);

        if (occupied.0 & (1u64 << (r * 8 + f))) != 0 {
            break;
        }

        r += 1;
        f += 1;
    }

    // left up
    let (mut r, mut f) = (rank + 1, file - 1);
    while f >= 0 && r < 8 {
        attacks |= (1u64) << (r * 8 + f);

        if (occupied.0 & (1u64 << (r * 8 + f))) != 0 {
            break;
        }

        r += 1;
        f -= 1;
    }

    // right down
    let (mut r, mut f) = (rank - 1, file + 1);
    while f < 8 && r >= 0 {
        attacks |= (1u64) << (r * 8 + f);

        if (occupied.0 & (1u64 << (r * 8 + f))) != 0 {
            break;
        }

        r -= 1;
        f += 1;
    }

    // left down
    let (mut r, mut f) = (rank - 1, file - 1);
    while f >= 0 && r >= 0 {
        attacks |= (1u64) << (r * 8 + f);

        if (occupied.0 & (1u64 << (r * 8 + f))) != 0 {
            break;
        }

        r -= 1;
        f -= 1;
    }

    Bitboard(attacks)
}

pub fn rook_attacks(sq: u8, occupied: Bitboard) -> Bitboard {
    let mut attacks: u64 = 0;

    let file: i32 = (sq % 8) as i32;
    let rank: i32 = (sq / 8) as i32;

    // up
    let mut r = rank + 1;
    while r < 8 {
        attacks |= 1u64 << (r * 8 + file);

        if occupied.0 & (1u64 << (r * 8 + file)) != 0 {
            break;
        }
        r += 1;
    }


    // down
    let mut r = rank - 1;
    while r >= 0 {
        attacks |= 1u64 << (r * 8 + file);

        if occupied.0 & (1u64 << (r * 8 + file)) != 0 {
            break;
        }
        r -= 1;
    }

    // right
    let mut f = file + 1;
    while f < 8 {
        attacks |= 1u64 << (rank * 8 + f);

        if occupied.0 & (1u64 << (rank * 8 + f)) != 0 {
            break;
        }
        f += 1;
    }

    // left
    let mut f = file - 1;
    while f >= 0 {
        attacks |= 1u64 << (rank * 8 + f);

        if occupied.0 & (1u64 << (rank * 8 + f)) != 0 {
            break;
        }
        f -= 1;
    }


    Bitboard(attacks)
}


pub fn queen_attacks(sq: u8, occupied: Bitboard) -> Bitboard {
    bishop_attacks(sq, occupied) | rook_attacks(sq, occupied)
}


pub fn king_attacks(sq: u8) -> Bitboard {
    let mut attacks: u64 = 0;
    let bb: u64 = 1u64 << sq;

    attacks |= bb << 8; // up
    attacks |= bb >> 8; // down

    attacks |= (bb << 1) & !(FILE_A); // left
    attacks |= (bb >> 1) & !(FILE_H); // left
 
    attacks |= (bb << 9) & !(FILE_A); // up right
    attacks |= (bb << 7) & !(FILE_H); // up left
    
    attacks |= (bb >> 9) & !(FILE_H); // down right
    attacks |= (bb >> 7) & !(FILE_A); // down left

    Bitboard(attacks)
}

// wow I actually found ways that are so much better
// LOL


// --------------------

// I should probably move these into move.rs

// this is how some good chess engines store moves (e.g. stockfish)
// 
// This is how I will be doing here:
//
// bit 0 - 5: from square
// bit 6- 11: to square
// bit 12-13: promotion piece type - 2 (KNIGHT-2 to QUEEN-2)
// bit 14-15: special move flag: promotion(1), enpassant(2), castling(3)
//             * enpassant bit is set only when pawn can be captured

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Move(pub u16);

impl Move {

}
