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
        
        // 1. place pieces
        let placement: &str = parts.next().expect("from_fen | missing piece placement");
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
                        _ => panic!("frome_fen (1) | unexpected character: {}", c),
                    };
                    // let's place the piece
                    board.pieces[color as usize][piece as usize].set_bit(sq as u8);
                    sq += 1;
                }
            }
        }

        // 2. side to move
        let side: &str = parts.next().expect("from_fen (2) | missing side to move");
        board.side_to_move = if side == "b" {
            Color::Black
        } else {
            Color::White
        };

        // 3. castling
        let castling = parts.next().expect("from_fen (3) | missing castling rights");
        board.castling_rights = 0;

        for c in castling.chars() {
            match c {
                'K' => board.castling_rights |= 0b0001,
                'Q' => board.castling_rights |= 0b0010,
                'k' => board.castling_rights |= 0b0100,
                'q' => board.castling_rights |= 0b1000,
                '-' => {},
                _ => panic!("from_fen (3) | unexpected character: {}", c),
            }
        }

        // 4. en_passant
        let ep: &str = parts.next().expect("from fen (4) | missing en passant field");
        println!("{}", ep);

        board.en_passant = if ep == "-" {
            None
        } else {
            // else, marks it with algebraic notation
            let bytes: &[u8] = ep.as_bytes();
            Some((bytes[1] - b'1') * 8 + (bytes[0] - b'a'))
        };

        // 5. halfmove clock
        let halfmove: Option<&str> = parts.next();       
        match halfmove {
            Some(count) => board.halfmove_clock = count.parse().expect("from fen (5) | Not a valid number"),
            None => board.halfmove_clock = 0,
        }

        // 6. full move
        let fullmove: Option<&str> = parts.next();
        match fullmove {
            Some(count) => board.fullmove_number = count.parse().expect("from fen (6) | Not a valid number"),
            None => board.halfmove_clock = 0,
        }

        return board;
    }
}
