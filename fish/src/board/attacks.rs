// this is how some good chess engines store moves (e.g. stockfish)
// 
// This is how I will be doing here:
//
// bit 0 - 5: from square
// bit 6- 11: to square
// bit 12-13: promotion piece type - 2 (KNIGHT-2 to QUEEN-2)
// bit 14-15: special move flag: promotion(1), enpassant(2), castling(3)
//             * enpassant bit is set only when pawn can be captured

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Move(pub u16);

impl Move {
}
