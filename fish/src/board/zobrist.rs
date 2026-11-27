use crate::board::board::*;

pub struct Zobrist {
    pub pieces: [[[u64; 64]; 6]; 2], 	// [color][pieces][square]
    pub side: u64, 						// XOR when black to move
    pub castling: [u64; 16], 			// key for castling
    pub ep_file: [u64; 8], 				// key for en-passant (since one ep per file)
}

/// xorshift
const fn next(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

impl Zobrist {
    /// compile time zobrist init
    const fn new() -> Zobrist {
		let mut state: u64 = 0x1234_5678_9ABC_DEF1;
		let mut pieces = [[[0u64; 64]; 6]; 2];

		let mut c = 0;
		while c < 2 {
    		let mut p = 0;
    		while p < 6 {
        		let mut sq = 0;
        		while sq < 64 {
            		pieces[c][p][sq] = next(&mut state);
            		sq += 1;
        		}
        		p += 1;
    		}
    		c += 1;
		}

		let side = next(&mut state);

		let mut castling = [0u64; 16];
		let mut i = 0;
    	while i < 16 {
        	castling[i] = next(&mut state);
        	i += 1;
    	}

    	let mut ep_file = [0u64; 8];
    	let mut f = 0;
        while f < 8 {
            ep_file[f] = next(&mut state);
            f += 1;
        }

		Zobrist { pieces, side, castling, ep_file }
    }
}

/// computed once at compile time!
pub const ZOBRIST: Zobrist = Zobrist::new();


impl Board {
    /// calculate hash for the position
    pub fn compute_hash(&self) -> u64 {
        let mut hash = 0u64;

		// iterate over all pieces
        for c in 0..2 {
            for p in 0..6 {
                let mut bb = self.pieces[c][p];
                while bb.0 != 0 {
                    let sq = bb.pop_lsb() as usize;
                    hash ^= ZOBRIST.pieces[c][p][sq];
                }
            }
        }

        // side to move (xor if black)
        if self.side_to_move == Color::Black {
            hash ^= ZOBRIST.side;
        }

        // castling rights
        hash ^= ZOBRIST.castling[self.castling_rights as usize];

		// en-passant
		if let Some(ep) = self.en_passant {
    		hash ^= ZOBRIST.ep_file[(ep % 8) as usize];
		}

		hash
    }

	/// toggle a piece on hash
    pub fn zobrist_toggle_piece(&mut self, color: Color, piece: Piece, sq: u8) {
        self.hash ^= ZOBRIST.pieces[color as usize][piece as usize][sq as usize];
    }
}
