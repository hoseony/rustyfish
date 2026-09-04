use crate::board::bitboard::Bitboard;
use crate::board::moves::*;

use std::fmt;

// ------------------- def  ------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const ALL_PIECES: [Piece; 6] = [
    Piece::Pawn, 
    Piece::Knight, 
    Piece::Bishop, 
    Piece::Rook, 
    Piece::Queen, 
    Piece::King, 
];

/// struct to represent the board status
#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub side_to_move: Color,
    pub castling_rights: u8, // going to use the 4 bits only
    // 0001: white King
    // 0010: whiet queen 
    // 0100: black king 
    // 1000: black queen

    pub en_passant: Option<u8>,
    pub halfmove_clock: u32, // if hit 100, 50 move rule
    pub fullmove_number: u32, // just to add more information
}

// -------------------- impl --------------------

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Board {

    /// initialize Board wth correct state; pieces, side, ...
    pub fn initialize_board() -> Board {
        Board {
            pieces: [
                [
                    Bitboard(0x000000000000FF00), // wp
                    Bitboard(0x0000000000000042), // wn
                    Bitboard(0x0000000000000024), // wb
                    Bitboard(0x0000000000000081), // wr
                    Bitboard(0x0000000000000008), // wq
                    Bitboard(0x0000000000000010), // wk
                ],
                [
                    Bitboard(0x00FF000000000000), // bp
                    Bitboard(0x4200000000000000), // bn
                    Bitboard(0x2400000000000000), // bb
                    Bitboard(0x8100000000000000), // br
                    Bitboard(0x0800000000000000), // bq
                    Bitboard(0x1000000000000000), // bk
                ],
            ],
            side_to_move: Color::White,
            castling_rights: 0b1111,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// returns symbol of a piece at the target square from the board
    pub fn piece_at(&self, sq: u8) -> char {
        let target_sq = 1u64 << sq;

        // it made more sense to use filled in pieces as white as 
        // filled in texts are rendered in white on my terminal.

        if self.pieces[0][0].0 & target_sq != 0 { return '♟'; }
        if self.pieces[0][1].0 & target_sq != 0 { return '♞'; }
        if self.pieces[0][2].0 & target_sq != 0 { return '♝'; }
        if self.pieces[0][3].0 & target_sq != 0 { return '♜'; }
        if self.pieces[0][4].0 & target_sq != 0 { return '♛'; }
        if self.pieces[0][5].0 & target_sq != 0 { return '♚'; }

        if self.pieces[1][0].0 & target_sq != 0 { return '♙'; }
        if self.pieces[1][1].0 & target_sq != 0 { return '♘'; }
        if self.pieces[1][2].0 & target_sq != 0 { return '♗'; }
        if self.pieces[1][3].0 & target_sq != 0 { return '♖'; }
        if self.pieces[1][4].0 & target_sq != 0 { return '♕'; }
        if self.pieces[1][5].0 & target_sq != 0 { return '♔'; }

        return '.';
    }

    /// returns type of Piece at the square given color
    pub fn piece_type_at(&self, color: Color, sq: u8) -> Option<Piece> {

        // for piece in &ALL_PIECES 
        //  - this will make piece type of &Piece
        //  - by doing &piece, we are matching &piece = &ALL_PIECES

        for &piece in &ALL_PIECES {
            if self.pieces[color as usize][piece as usize].is_set(sq) {
                return Some(piece);
            }
        }

        None
    }


    pub fn occupancy(&self, color: Color) -> Bitboard {
        let mut occupied: u64 = 0;
        
        for piece in self.pieces[color as usize].iter() {
            occupied |= piece.0
        }

        Bitboard(occupied)
    }

    pub fn all_occupancy(&self) -> Bitboard {
        self.occupancy(Color::White) | self.occupancy(Color::Black)
    }

    pub fn make_move(&mut self, mv: Move) {
        // unpacking move
        let from: u8 = mv.from_sq();
        let to: u8   = mv.to_sq();
        let flag: u8 = mv.flag_bits();
        
        let friendly: Color = self.side_to_move;
        let enemy: Color = friendly.opposite();

        /* 1. who is moving? */
        let moving_piece = self.piece_type_at(friendly, from).expect("make_move | where is the move?");

        /* 2. remove captured piece if any */
        let mut was_capture = false; // this will be used at 7.
                                      
        // if there a enemy piece, remove bit, update was_capture
        if let Some(capture) = self.piece_type_at(enemy, to) {
            self.pieces[enemy as usize][capture as usize].remove_bit(to);
            was_capture = true;

            // if rook capture, revoke opponent's castle right accordingly
            if capture == Piece::Rook {
                match (enemy, to) {
                    (Color::White, 7)  => self.castling_rights &= !0b0001u8,
                    (Color::White, 0)  => self.castling_rights &= !0b0010u8,
                    (Color::Black, 63) => self.castling_rights &= !0b0100u8,
                    (Color::Black, 56) => self.castling_rights &= !0b1000u8,
                    _ => {},

                }

            }
        }

        /* 3. move the piece */
        self.pieces[friendly as usize][moving_piece as usize].move_bit(from, to);

        /* 4. handle special rule */
        match flag {
            FLAG_NONE => {}, // there nothing to do here

            FLAG_PROMOTION => {
                // remove pawn 
                // add some piece
                self.pieces[friendly as usize][0].remove_bit(to);

                let promotion_piece: u8 = mv.promotion_bits();
                self.pieces[friendly as usize][promotion_piece as usize + 1].set_bit(to);
                // 0 - Knight 
                // 1 - Bishop
                // 2 - Rook 
                // 3 - Queen
            },

            FLAG_EN_PASSANT => {
                // remove pawn
                let target: u8 = if friendly == Color::White { to - 8 } else { to + 8 };
                self.pieces[enemy as usize][0].remove_bit(target);
            },

            FLAG_CASTLING => { 
                let (rook_from, rook_to) = match to {
                    6  => (7, 5),
                    2  => (0, 3),
                    62 => (63, 61),
                    58 => (56, 59),
                    _ => unreachable!("make_move | invalid castling destination: {}", to),

                };
                self.pieces[friendly as usize][Piece::Rook as usize].move_bit(rook_from, rook_to);

            },

            _ => unreachable!("make_move | flag should be 0-3, got {}", flag),
        }

        // 5. update en passant 

        self.en_passant = None;
        if moving_piece == Piece::Pawn {
            let diff = (to as i16 - from as i16).abs();
            // if the pawn moved two square forward, 
            // add the midpoint between from and to (which is the ep square)
            // to the en_passant value.

            if diff == 16 {
                self.en_passant = Some(((from + to) as u16 / 2) as u8);
            }
        }

        // 6. update castling 
        //     0001: white King
        //     0010: whiet queen 
        //     0100: black king 
        //     1000: black queen
        
        if moving_piece == Piece::King {
            if friendly == Color::White {
                self.castling_rights &= !0b0011u8;
            } else {
                self.castling_rights &= !0b1100u8;
            }
        }

        if moving_piece == Piece::Rook {
            match (friendly, from) {
                (Color::White, 7)  => self.castling_rights &= !0b0001u8,
                (Color::White, 0)  => self.castling_rights &= !0b0010u8,
                (Color::Black, 63) => self.castling_rights &= !0b0100u8,
                (Color::Black, 56) => self.castling_rights &= !0b1000u8,
                _ => {}, // oops, this should not panic
            }
        }

        // 7. update halfmove clock 
        
        // if capture, reset the clock
        // if pawn moves, reset the clock, otherwise increment
        if (moving_piece == Piece::Pawn) || (was_capture) || flag == FLAG_EN_PASSANT {
            self.halfmove_clock = 0; 
        } else {
            self.halfmove_clock += 1;
        }

        // 8. update full move 
        if friendly == Color::Black {
            self.fullmove_number += 1;
        }

        // 9. switch side, turn ends
        self.side_to_move = enemy;
    }
}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for rank in (0..8).rev() {
            write!(f, "{} \t", rank + 1)?;

            for file in 0..8 {
                let sq: u8 = (rank * 8 + file) as u8;
                let symbol: char = self.piece_at(sq);
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }

        writeln!(f)?; // vertical padding
        write!(f, "    ")?; // horizontal padding

        for j in b'A'..=b'H' {
            write!(f, "{} ", j as char)?;
        }


        writeln!(f)?; // padding at the end

        Ok(())
    }
}

/// generate_pseudo_legal_moves -> generate_legal_moves
/// legal_moeves checks king safety after the move.
/// I will still do the brute-force method of checking.
/// I might come back to change it to more clever ways (only checking pinned pieces, king moves, ... )
pub fn generate_legal_moves(board: &Board) -> Vec<Move> {
    let pseudo_moves: Vec<Move> = generate_pseudo_legal_moves(board);
    let mut legal_moves = Vec::new();
    let friendly = board.side_to_move;
    let enemy = friendly.opposite();

    // let enemy_occupied = board.occupancy(enemy);
    for i in 0..pseudo_moves.len() {
        let mut test_board: Board = *board;
        test_board.make_move(pseudo_moves[i]);

        let attack = generate_attack_bitboard(&test_board, enemy);
        if (attack.0 & test_board.pieces[friendly as usize][Piece::King as usize].0) == 0 {
            legal_moves.push(pseudo_moves[i]);
        }
    }
   
    legal_moves
}

pub fn perft(board: &Board , depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_legal_moves(board);
    let mut nodes: u64 = 0;

    for mv in moves {
        let mut new_board = *board;
        new_board.make_move(mv);
        nodes += perft(&new_board, depth - 1);
    }
    nodes
}
