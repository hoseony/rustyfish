use fish::board::board::*;

fn next_random(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn same_position_same_hash() {
    let a = Board::initialize_board();
    let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(a.hash, b.hash);

    // only the side to move differs → different hash
    let c = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
    assert_ne!(a.hash, c.hash);
}

#[test]
fn incremental_hash_matches_full() {
    let fens = [
        // start position
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // kiwipete: castling, en passant, promotions show up quickly
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    ];
    let mut rng: u64 = 12345;

    for fen in fens {
        for _game in 0..50 {
            let mut board = Board::from_fen(fen);
            for _ply in 0..80 {
                let moves = generate_legal_moves(&board);
                if moves.is_empty() {
                    break;
                }
                let mv = moves[(next_random(&mut rng) % moves.len() as u64) as usize];
                board.make_move(mv);

                assert_eq!(
                    board.hash,
                    board.compute_hash(),
                    "hash wrong after {} in game from {fen}",
                    mv.to_uci()
                );
            }
        }
    }
}
