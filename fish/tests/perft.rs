use fish::board::board::*;

fn check(fen : &str, depth: u32, expected: u64) {
    let board = Board::from_fen(fen);
    let nodes = perft(&board, depth);
    assert_eq!(nodes, expected, "perft failed: fen={fen} depth{depth}");
}

#[test]
fn perft_startpos() {
    check("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 4, 197_281);
}

#[test]
fn perft_kiwipete() {
    check("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 3, 97_862);
}

#[test]
#[ignore]
// too slow to run every time; only runs when you ask for it
// `cargo test -- --ignored` ensures this to run
fn perft_deep() {
    check("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 5, 4_865_609);
}
