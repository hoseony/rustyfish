use crate::board::board::*;

/* -------- Piece Square Tables -------- */
// took it from https://chessprogramming.org/Simplified_Evaluation_Function

pub const PST_PAWN: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
];

pub const PST_KNIGHTS: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub const PST_BISHOPS: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

pub const PST_ROOKS: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0,
];

pub const PST_QUEENS: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

/// king middle game
pub const PST_KING_MG: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20,
];

/// king end game
pub const PST_KING_EG: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50,
];

/// p, kn, b, r, q, k (middle), k (end)
pub const PST: [[i32; 64]; 7] = [
    PST_PAWN, PST_KNIGHTS, PST_BISHOPS, PST_ROOKS, PST_QUEENS, PST_KING_MG, PST_KING_EG,
];

pub const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 0];

/* ---------- evaluation functions ---------- */

/// board -> score
pub fn eval(board: &Board) -> i32 {
	let score = piece_sum(board) + piece_square_table(board) + king_score(board);

    if board.side_to_move == Color::White {score} else {-score}
}

/// positional score, white pov
fn piece_square_table(board: &Board) -> i32 {
    let mut score = 0;

	// iterate over entire boards, adds up the score
	// exclude king (calculated separately with king_score)
	for i in 0..=1 {
    	for j in 0..5 {
			let mut bb = board.pieces[i][j];

			while bb.0 != 0 {
    			let sq = bb.pop_lsb() as usize;
    			let index = if i == 0 {sq ^ 56} else {sq};

    			if i == 0 {
        			score += PST[j][index];
    			} else {
        			score -= PST[j][index];
    			}
			}
    	}
	}

	score
}

/// material balnace, white pov
/// sum the value of the pieces (soley based on their value from PIECE_VALUES)
fn piece_sum(board: &Board) -> i32 {
    let mut score = 0;

    for j in 0..6 {
        let white = board.pieces[0][j].count_ones() as i32;
        let black = board.pieces[1][j].count_ones() as i32;

        score += (white - black) * PIECE_VALUES[j];
    }

    score
}

/// 24 max, determine phase of t he game based on the pieces left
fn game_phase(board: &Board) -> i32 {
    const WEIGHTS: [i32; 6] = [0, 1, 1, 2, 4, 0]; // P, N, B, R, Q, K

	let mut phase = 0;
	for j in 0..6 {
    	let count = (board.pieces[0][j].count_ones() + board.pieces[1][j].count_ones()) as i32;
    	phase += count * WEIGHTS[j]
	}

    phase.min(24)
}

fn king_score(board: &Board) -> i32 {
    let phase = game_phase(board);
    let mut score = 0;

    for i in 0..2 {
        let mut bb = board.pieces[i][Piece::King as usize];
        let sq = bb.pop_lsb() as usize;
        let index = if i == 0 {sq ^ 56} else {sq};

        let mg = PST[5][index];
        let eg = PST[6][index];

        let blended = (mg * phase + eg * (24 - phase)) / 24;

        if i == 0 {score += blended;} else {score -= blended}
    }

    score
}




#[test]
fn startpos_pst() {
    assert_eq!(piece_square_table(&Board::initialize_board()), 0);
}

#[test]
fn eval_flips_for_black() {
    // white is up a queen
    let w = Board::from_fen("4k3/8/8/8/8/8/8/3QK3 w - - 0 1");
    let b = Board::from_fen("4k3/8/8/8/8/8/8/3QK3 b - - 0 1");
    assert!(eval(&w) > 0); // good for white when white to move
    assert!(eval(&b) < 0); // bad for black when black to move
    assert_eq!(eval(&w), -eval(&b)); // exactly opposite
}
