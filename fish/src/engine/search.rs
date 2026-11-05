use crate::board::board::*;
use crate::engine::eval::*;
use crate::board::moves::*;

// these numbers should be good enough
pub const INF:i32  = 1_000_000;
pub const MATE: i32 = 100_000;


/// negamax: returns evaluated number
fn negamax(board: &Board, depth: u32, mut alpha: i32, beta: i32) -> i32 {
	// alpha: the best score secured based on moves searched
	// beta: the best score that opponent secured

    // base case:
	if depth == 0 { return eval(board); }

	// generate & check moves
	let moves = generate_legal_moves(board);
	if moves.is_empty() {
    	// mate or stalemate
    	return if board.is_in_check() { -MATE } else { 0 }
	}

	// recursively search
	for mv in moves {
    	// copy the board & make the move
    	let mut child = *board;
    	child.make_move(mv);
    	let score = -negamax(&child, depth - 1, -beta, -alpha);

		// stop searching this node
		// if the score is higher than what opponent secured,
		// There's no way your opponent will let that happen.
    	if score > beta {
        	return beta;
    	}

		// this branch improves score
		// (it is higher than what you previously had)
    	if score > alpha {
        	alpha = score;
    	}
	}

	alpha
}

/// start of the search, negamax() only returns the number (evaluation)
/// you need to know what move is the best
pub fn search_root(board: &Board, depth: u32) -> (Move, i32) {
	let mut moves = generate_legal_moves(board);
	order_moves(board, &mut moves);
	let mut best_move = moves[0];

	let mut alpha = -INF;
	let beta = INF;

	for mv in moves {
    	let mut child = *board;
    	child.make_move(mv);
    	let score = -negamax(&child, depth - 1, -beta, -alpha);

		// when you see a better move, update
    	if score > alpha {
        	alpha = score;
        	best_move = mv;
    	}
	}

	(best_move, alpha)
}

fn order_moves(board: &Board, moves: &mut [Move]) {
	moves.sort_by_key(|&mv| -mvv_lva_score(board, mv));
}


/// Most Valuable Victim - Least Valuable Agressor
fn mvv_lva_score(board: &Board, mv: Move) -> i32 {
    let us = board.side_to_move;
    let them = us.opposite();

    match board.piece_type_at(them, mv.to_sq()) {
        Some(victim) => {
            let attacker = board.piece_type_at(us, mv.from_sq()).unwrap();
            10 * PIECE_VALUES[victim as usize] - PIECE_VALUES[attacker as usize]
        }
        None => 0,
    }

}
