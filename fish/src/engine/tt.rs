use crate::board::moves::*;
use crate::engine::search::MATE;

#[derive(Clone, Copy)]
/// Transposition table node bound
pub enum Bound {
    Exact, // real score
    Lower, // real score >= this (beta)
    Upper, // real score <= this (alpha)
}

#[derive(Clone, Copy)]
/// transposition table entry
pub struct TTEntry {
    pub key: u64,
    pub depth: u32,
    pub score: i32,
    pub bound: Bound,
    pub best_move: Move,
}

/// transposition table
pub struct TT {
    entries: Vec<Option<TTEntry>>,
}

impl TT {
    /// create a new TT
	pub fn new(size: usize) -> TT {
    	TT {
        	entries: vec![None; size]
    	}
	}

	/// calculate index
	fn index(&self, key: u64) -> usize {
    	(key % self.entries.len() as u64) as usize
	}

	/// look up a position
	pub fn probe(&self, key: u64, ply: i32) -> Option<TTEntry> {
    	match self.entries[self.index(key)] {
        	Some(mut entry) if entry.key == key => {
                entry.score = score_from_tt(entry.score, ply);
            	Some (entry)
        	}
        	_ => None,
    	}
	}

	/// save an entry (overwriting)
	pub fn store(&mut self, mut entry: TTEntry, ply: i32) {
    	entry.score = score_to_tt(entry.score, ply);
    	let i = self.index(entry.key);
    	self.entries[i] = Some(entry);
	}

	/// clear entire table
	pub fn clear(&mut self) {
    	for slot in self.entries.iter_mut() {
        	*slot = None;
    	}
	}
}

/// root relative -> position relative
fn score_to_tt(score: i32, ply: i32) -> i32 {
    if score > (MATE - 1000) { 		 // winning
        score + ply
    } else if score < -MATE + 1000 { // getting mated
        score - ply
    } else {
        score
    }
}

/// position relative -> root relative
fn score_from_tt(score: i32, ply: i32) -> i32 {
    if score > (MATE - 1000) {
        score - ply
    } else if score < -MATE + 1000 {
        score + ply
    } else {
        score
    }
}
