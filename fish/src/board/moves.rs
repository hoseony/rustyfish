use crate::board::board::Piece::Pawn;
use crate::board::board::*;
use crate::board::attacks::*;
use crate::board::bitboard::*;

// I should probably move these into move.rs
//
// this is how some good chess engines store moves (e.g. stockfish)
// 
// This is how I will be doing here:
//
// bit 0 - 5: from square
// bit 6 -11: to square
// bit 12-13: promotion piece type
//               0 - Knight | 1 - Bishop | 2 - Rook | 3 - Queen
// bit 14-15: special move flag: promotion(1), enpassant(2), castling(3)
//             * enpassant bit is set only when pawn can be captured

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Move(pub u16);

pub const FLAG_NONE: u8 = 0;
pub const FLAG_PROMOTION: u8 = 1;
pub const FLAG_EN_PASSANT: u8 = 2;
pub const FLAG_CASTLING: u8 = 3;


impl Move {

    /// pack all the information into Move type
    pub fn new(from: u8, to: u8, promotion: u8, flag: u8) -> Move {
        let packed: u16 = (from as u16) | ((to as u16) << 6) | ((promotion as u16) << 12) | ((flag as u16) << 14);

        Move(packed)
    }

    /// unpack 'to' from Move
    pub fn to_sq(self) -> u8 {
        // self.0 >> 6 makes it start from 'to square'
        // then isolate 6 bits with mask
        ((self.0 >> 6) & (0x3F)) as u8
    }

    /// unpack 'from' from Move
    pub fn from_sq(self) -> u8 {
        (self.0 & 0x3F) as u8
    }

    /// unpack 'promotion piece'from Move
    pub fn promotion_bits(self) -> u8 {
        ((self.0 >> 12) & (0x03)) as u8
    }

    /// unpack 'flag' from Move
    pub fn flag_bits(self) -> u8 {
        ((self.0 >> 14) & (0x03)) as u8
    }
}

// ------------------------------------

pub fn generate_pseudo_legal_moves(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let friendly = board.side_to_move;

    let friendly_occupied = board.occupancy(friendly);
    let occupied = board.all_occupancy();

    generate_pawn_moves(board, friendly, &mut moves);
    generate_castling_moves(board, friendly, &mut moves);

    for j in 0..6 {
        let mut bb = board.pieces[friendly as usize][j];
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
            }; 

            // now attacks contain all the possible moves that piece can move 
            // This needs to be converted into individual "move"

            // targets must exclude your own pieces
            let mut target = Bitboard(attacks.0 & !(friendly_occupied.0));

            // iterate over and add it to the Moves
            while target.0 != 0 {
                let index = target.pop_lsb();
                let packed: u16 = (sq as u16) | ((index as u16) << 6) | ((FLAG_NONE as u16) << 14);
                moves.push(Move(packed));
            }
        }
    }
    
    moves
}

pub fn generate_attack_bitboard(board: &Board, color: Color) -> Bitboard {
    let mut attacks: u64 = 0; 
    let occupied = board.all_occupancy();

    // send help... I just gotta do this :(
    let mut knights = board.pieces[color as usize][Piece::Knight as usize];
    while knights.0 != 0 {
        let sq = knights.pop_lsb();
        attacks |= knight_attacks(sq).0;
    }

    let mut rooks = board.pieces[color as usize][Piece::Rook as usize];
    while rooks.0 != 0 {
        let sq = rooks.pop_lsb();
        attacks |= rook_attacks(sq, occupied).0;
    }

    let mut bishops = board.pieces[color as usize][Piece::Bishop as usize];
    while bishops.0 != 0 {
        let sq = bishops.pop_lsb();
        attacks |= bishop_attacks(sq, occupied).0;
    }

    let mut queens = board.pieces[color as usize][Piece::Queen as usize];
    while queens.0 != 0 {
        let sq = queens.pop_lsb();
        attacks |= queen_attacks(sq, occupied).0;
    }

    let mut king = board.pieces[color as usize][Piece::King as usize];
    while king.0 != 0 {
        let sq = king.pop_lsb();
        attacks |= king_attacks(sq).0;
    }

    let mut pawns = board.pieces[color as usize][Piece::Pawn as usize];
    while pawns.0 != 0 {
        let sq = pawns.pop_lsb();
        attacks |= if color == Color::White {
            white_pawn_attacks(sq).0
        } else {
            black_pawn_attacks(sq).0
        }
    }

    return Bitboard(attacks);
}

pub fn generate_castling_moves(board: &Board, color: Color, moves: &mut Vec<Move>) {
    let (kingside, queenside): (u8, u8) = 
                    if color == Color::White { 
                        (0b00000001, 0b00000010)
                    } else {
                        (0b00000100, 0b00001000)
                    };
   
    // kingside_right and queenside_right = 1 if it has the right
    let kingside_right: bool = (kingside & board.castling_rights) != 0;
    let queenside_right: bool = (queenside & board.castling_rights) != 0;
   
    // if no right, nothing to do
    if (kingside_right || queenside_right) == false {
        return;
    }

    // check occupancy, check attacks -> add to move
    // (reference the board for constants)
        //     1 0 * 0 x 0 * 1
        //     0 0 0 0 0 0 0 0
        //     0 0 0 0 0 0 0 0
        //     0 0 0 0 0 0 0 0
        //     0 0 0 0 0 0 0 0
        //     0 0 0 0 0 0 0 0
        //     0 0 0 0 0 0 0 0
        //     1 0 * 0 x 0 * 1

    let occupied = board.all_occupancy();

    let enemy = color.opposite();
    let attack_bb = generate_attack_bitboard(board, enemy);

    // The rook moving part will be implemented on the `make_move`
    // White
    if color == Color::White {
        // kingside
        if !occupied.is_set(5) && !occupied.is_set(6) { // occupancy check
            if kingside_right {
                // attack square check (king also should not be in check!)
                if !attack_bb.is_set(4) && !attack_bb.is_set(5) && !attack_bb.is_set(6) {
                    let packed: u16 = (4u16) | ((6u16) << 6) | ((FLAG_CASTLING as u16) << 14);
                    moves.push(Move(packed));
                }
            } 
        }

        // queenside
        if !occupied.is_set(1) && !occupied.is_set(2) && !occupied.is_set(3) {
            if queenside_right {
                if !attack_bb.is_set(2) && !attack_bb.is_set(3) && !attack_bb.is_set(4) {
                    let packed: u16 = (4u16)| ((2u16) << 6) | ((FLAG_CASTLING as u16) << 14);
                    moves.push(Move(packed));
                }
            }
        }
    }

    // King
    if color == Color::Black{
        // kingside
        if !occupied.is_set(62) && !occupied.is_set(61) {
            if kingside_right {
                if !attack_bb.is_set(62) && !attack_bb.is_set(61) && !attack_bb.is_set(60){
                    let packed: u16 = (60u16) | ((62u16) << 6) | ((FLAG_CASTLING as u16) << 14);
                    moves.push(Move(packed));
                } 
            }
        }
       
        // queenside
        if !occupied.is_set(59) && !occupied.is_set(58) && !occupied.is_set(57) {
            if !attack_bb.is_set(60) && !attack_bb.is_set(59) && !attack_bb.is_set(58) {
                if queenside_right {
                    let packed: u16 = (60u16) | ((58u16) << 6) | ((FLAG_CASTLING as u16) << 14);
                    moves.push(Move(packed));
                }
            }
        }
    }
}


/// This function adds possible moves by the pawns to the Vec<Move>
pub fn generate_pawn_moves(board: &Board, color: Color, moves: &mut Vec<Move>) {
    let enemy = color.opposite();
    let _friendly_occupied = board.occupancy(color);

    let enemy_occupied = board.occupancy(enemy);
    let all_occupied = board.all_occupancy();


    let mut pawns: Bitboard = board.pieces[color as usize][Piece::Pawn as usize];
    while pawns.0 != 0 {
        let from = pawns.pop_lsb();
        // 1. push 
        //  - pushing
        //  - promotion
        
        let (pawn_pushes, promotion_rank): (Bitboard, u64) = if color == Color::White {
            (white_pawn_pushes(from, all_occupied), RANK_8)
        } else {
            (black_pawn_pushes(from, all_occupied), RANK_1)
        };

        // check if the pawn push leads to a promotion rank
        let mut pawn_push_rank_check: Bitboard = pawn_pushes;
        while pawn_push_rank_check.0 != 0 {
            let to = pawn_push_rank_check.pop_lsb();
            are_you_promoting(from, to, promotion_rank, moves)
        }

        // 2. capture 
        //  - check capture -> Promotion, too
        let pawn_attacks = if color == Color::White {
            white_pawn_attacks(from)
        } else {
            black_pawn_attacks(from)
        };

        let mut pawn_capture_rank_check = Bitboard(pawn_attacks.0 & enemy_occupied.0);
        while pawn_capture_rank_check.0 != 0 {
            let to = pawn_capture_rank_check.pop_lsb();
            are_you_promoting(from, to, promotion_rank, moves);
        }

        // 3. en passant
        if let Some(ep_sq) = board.en_passant {
            if pawn_attacks.is_set(ep_sq) { // if the the move is enpassant & on the pawn_attacks
                                            // it is a valid move (EnPassant!)
                                            // board.en_passant updates on make_move function
                moves.push(Move::new(from, ep_sq, 0, FLAG_EN_PASSANT));
            }

        }

    }
}

fn are_you_promoting(from: u8, to: u8, promotion_rank: u64, moves: &mut Vec<Move>) {
    if (1u64 << to) & promotion_rank != 0 {
        for promotion in 0..=3u8 {
            // add move + flag + 4 different pieces it can promote (kn, rk, bs, q)
            moves.push(Move::new(from, to, promotion, FLAG_PROMOTION));
        }
    } else {
        moves.push(Move::new(from, to, 0, FLAG_NONE));
    }
}

