use crate::board::bitboard::Bitboard;
use std::fmt;

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

/// struct to represent the board status
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub side_to_move: Color,
    pub castling_rights: u8,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u32, // if hit 100, 50 move rule
    pub fullmove_number: u32, // just to add more information
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
