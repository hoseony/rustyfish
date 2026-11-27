use crate::board::board::*;
use crate::engine::eval::*;
use crate::board::moves::*;

use std::time::Instant;

// these numbers should be good enough
pub const INF:i32  = 1_000_000;
pub const MATE: i32 = 100_000;

pub struct SearchInfo {
    pub deadline: Instant, 	// when to stop
    pub stopped: bool, 		// did any search figured time is up?
    pub nodes: u64, 		// positions searches
    pub history: Vec<u64> 	// history of moves (hashed!)
}

/// negamax: returns evaluated number
fn negamax(board: &Board, depth: u32, ply: i32, mut alpha: i32, beta: i32, info : &mut SearchInfo) -> i32 {
	// alpha: the best score secured based on moves searched
	// beta: the best score that opponent secured

	if time_up(info) {
    	return 0;
	}

    // base case:
	if depth == 0 { return qsearch(board, alpha, beta, info); }

	// special draw condition
	if is_repetition(board, &info.history) || board.halfmove_clock >= 100 {
    	return 0;
	}

	// generate & check moves
	let mut moves = generate_legal_moves(board);
	order_moves(board, &mut moves);

	if moves.is_empty() {
    	// mate or stalemate
    	return if board.is_in_check() { -MATE + ply } else { 0 }
	}

	// recursively search
	for mv in moves {
    	// copy the board & make the move
    	let mut child = *board;
    	child.make_move(mv);

		info.history.push(board.hash);
    	let score = -negamax(&child, depth - 1, ply + 1, -beta, -alpha, info);
		info.history.pop();

		// stop searching this node when
		// 1. Time is up (in this case, don't ues the score)
		if info.stopped {
    		return 0;
		}

		// if the score is higher than what opponent secured,
		// There's no way your opponent will let that happen.
    	if score >= beta {
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

fn qsearch(board: &Board, mut alpha: i32, beta: i32, info: &mut SearchInfo) -> i32 {
	if time_up(info) {
    	return 0;
	}

	let static_eval = eval(board);

	if static_eval >= beta {
    	return beta;
	}

	if static_eval > alpha {
    	alpha = static_eval;
	}

	let them = board.side_to_move.opposite();
	let enemy_pieces = board.occupancy(them);

	let mut moves = generate_legal_moves(board);
	order_moves(board, &mut moves);

	for mv in moves {
		// check if it is a capture
		let is_capture = enemy_pieces.is_set(mv.to_sq());
		let is_en_passant = mv.flag_bits() == FLAG_EN_PASSANT;

		if !is_capture && !is_en_passant {
    		continue;
		}

    	let mut child = *board;
    	child.make_move(mv);
    	let score = -qsearch(&child, -beta, -alpha, info);

		if info.stopped {
    		return 0;
		}

    	if score >= beta {
        	return beta;
    	}

    	if score > alpha {
        	alpha = score;
    	}
	}

	alpha
}

/// start of the search, negamax() only returns the number (evaluation)
/// you need to know what move is the best
pub fn search_root(board: &Board, depth: u32, info: &mut SearchInfo) -> (Move, i32) {
	let mut moves = generate_legal_moves(board);
	order_moves(board, &mut moves);
	let mut best_move = moves[0];

	let mut alpha = -INF;
	let beta = INF;

	for mv in moves {
    	let mut child = *board;
    	child.make_move(mv);

    	info.history.push(board.hash);
    	let score = -negamax(&child, depth - 1, 1, -beta, -alpha, info);
    	info.history.pop();

		if info.stopped {
    		break;
		}

		// when you see a better move, update
    	if score > alpha {
        	alpha = score;
        	best_move = mv;
    	}
	}

	(best_move, alpha)
}


/// reorder move based on mvv_lva
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

/// check if the time is up every 2048 nodes searched
/// returns info.stopped (default: false, if timeout: true)
fn time_up(info: &mut SearchInfo) -> bool {
    info.nodes += 1;
    if info.nodes.is_multiple_of(2048) && Instant::now() >= info.deadline {
        info.stopped = true;
    }
    info.stopped
}

/// is this position repeated? (yes->true)
fn is_repetition(board: &Board, history: &[u64]) -> bool {
    history.iter().rev()
        .take(board.halfmove_clock as usize)
        .skip(1).step_by(2).any(|&h| h == board.hash)
    // sorry for the funny function :(
}
