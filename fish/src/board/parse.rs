use crate::board::board::*;
use crate::board::bitboard::*;

/// parse fen -> Board
impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board {
            pieces: [[Bitboard::EMPTY ; 6] ; 2],
            side_to_move: Color::White,
            castling_rights: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        let mut parts = fen.split_whitespace();
        // 1. let's put down some pieces

        let placement = parts.next().expect("from_fen | missing piece placement");
        // fen starts at A8!
        let mut sq: i32 = 56;

        for c in placement.chars() {
            match c {
                // e.g. for the first loop, when it hits h8
                // you need to move to a7
                '/' => sq -= 16,
                
                // if it is a number, move the square accordingly
                '1'..='8' => sq += c as i32 - 0x30,
                
                // everything else: place the piece
                _ => {
                    let (color, piece) = match c {
                        'P' => (Color::White, Piece::Pawn),
                        'N' => (Color::White, Piece::Knight),
                        'B' => (Color::White, Piece::Bishop),
                        'R' => (Color::White, Piece::Rook),
                        'Q' => (Color::White, Piece::Queen),
                        'K' => (Color::White, Piece::King),
                        'p' => (Color::Black, Piece::Pawn),
                        'n' => (Color::Black, Piece::Knight),
                        'b' => (Color::Black, Piece::Bishop),
                        'r' => (Color::Black, Piece::Rook),
                        'q' => (Color::Black, Piece::Queen),
                        'k' => (Color::Black, Piece::King),
                        _ => panic!("frome_fen | unexpected character: {}", c),
                    };
                    // let's place the piece
                    board.pieces[color as usize][piece as usize].set_bit(sq as u8);
                    sq += 1;
                }
            }
        }

        return board;
    }
}
