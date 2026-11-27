use crate::board::moves::*;

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
	pub fn probe(&self, key: u64) -> Option<TTEntry> {
    	match self.entries[self.index(key)] {
        	Some(entry) if entry.key == key => Some (entry),
        	_ => None,
    	}
	}

	/// save an entry (overwriting)
	pub fn store(&mut self, entry: TTEntry) {
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
