# rustyfish versions

Rating estimates come from matches against Stockfish with `UCI_LimitStrength = true`, using cutechess-cli. Treat this as rough estimates...

## Summary

| Version | Main change | Opponent | Score | Elo diff | Est. rating |
|---|---|---|---|---|---|
| v0.1-baseline | material + PST, alpha-beta, MVV-LVA, qsearch | sf1700 | 0.395 | −74 ± 53 | ~1625 |
| v0.2-noFlag | time deadline, phase king table, mate distance | sf1700 | 0.540 | +28 ± 66 | ~1730 |
| v0.3-tt | zobrist, repetition detection, transposition table | sf2000 | 0.465 | -24 ± 64 | ~1980 |

## v0.3-tt
**Changes**
- Zobrist hashing
- en passant square only set when a capture is possible
- repetition detection & 50 move rule
- transposition table (with exact/lower/upper, move ordering with TT)

**Tests:** tc=10+0.1, 8moves_v3 openings
```
Score of rustyfish vs sf2000: 40 - 47 - 13  [0.465] 100
Elo difference: -24.4 +/- 64.3, LOS: 22.6 %, DrawRatio: 13.0 %
```

## v0.2-noFlag
**Changes**
- search stops mid-depth with deadline
- time budget capped at `time/2`
- king score transition by phase
- mate distance on negamax

**Test:** 100 games vs sf1700, tc=10+0.1
```
Score of rustyfish vs sf1700: 49 - 41 - 10  [0.540] 100
Elo difference: 27.9 +/- 65.5, LOS: 80.0 %
```

**Things to fix…**
- repetition detection

## v0.1-baseline
- bitboard move gen (verified with perft)
- FEN parsing, UCI protocol
- Evalulation: material + piece square table
- negamax with alpha beta, MVV_LVV move ordering, quiescence search
- Iterative deepening

**Test:** 100 games vs sf1700, tc=10+0.1
```
Score of rustyfish vs sf1700: 19 - 40 - 41  [0.395] 100
Elo difference: -74.1 +/- 53.0, LOS: 0.3 %
```
