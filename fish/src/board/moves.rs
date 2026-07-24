use crate::board::board::*;
use crate::board::attacks::*;
use crate::board::bitboard::*;

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

pub const FLAG_NONE: u8 = 0;
pub const FLAG_PROMOTION: u8 = 1;
pub const FLAG_EN_PASSANT: u8 = 2;
pub const FLAG_CASTLING: u8 = 3;


impl Move {
    pub fn to_sq(self) -> u8 {
        // self.0 >> 6 makes it start from 'to square'
        // then isolate 6 bits with mask
        ((self.0 >> 6) & (0x3F)) as u8
    }

    pub fn from_sq(self) -> u8 {
        (self.0 & 0x3F) as u8
    }

    pub fn promotion_bits(self) -> u8 {
        ((self.0 >> 12) & (0x03)) as u8
    }

    pub fn flag_bits(self) -> u8 {
        ((self.0 >> 14) & (0x03)) as u8
    }
}

// ------------------------------------

pub fn generate_pseudo_legal_moves(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for i in 0..=1 {
        let color = if i == 0 { Color::White } else { Color::Black };
        let occupied = board.occupancy(color);

        for j in 0..6 {
            let mut bb = board.pieces[i][j];
            while bb.0 != 0 {
                // It would been better to make pop_lsb not an impl of bitboard..
                
                let sq = bb.pop_lsb();

                let attacks: Bitboard = match j {
                    0 => Bitboard::EMPTY, // I need to think about how to handle this
                    1 => knight_attacks(sq),
                    2 => bishop_attacks(sq, occupied),
                    3 => rook_attacks(sq, occupied),
                    4 => queen_attacks(sq, occupied),
                    5 => king_attacks(sq),
                    _ => unreachable!("generate_pseudo_legal_moves | piece type index bound must be between 0-5, got {}", j),
                    // unreachable <-> panic
                    //
                }; 

                // now attacks contain all the possible moves that piece can move 
                // This needs to be converted into individual "move"

                let mut target = Bitboard(attacks.0 & !(occupied.0));
                while target.0 != 0 {
                    let index = target.pop_lsb();
                    let packed: u16 = (sq as u16) | ((index as u16) << 6) | ((FLAG_NONE as u16) << 14);
                    moves.push(Move(packed));
                }
            }
        }
    }
    
    moves
}
