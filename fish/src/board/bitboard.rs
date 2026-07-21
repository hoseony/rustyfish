use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Bitboard(pub u64);

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Square {
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A0, B0, C0, D0, E0, F0, G0, H0,
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    pub fn set_bit(&mut self, sqidx: u8) {
        self.0 |= 1u64 << sqidx;
    }
}

/* learning note:
 *  
 *  we do this (impl fmt::Display for Bitboard) because we can not just
 *  print regularly with the struct. Thus, we are making special implementation
 *  for printing it.
 *
 *
 * fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 *               ^ this is the output destination
 *                                          ^ fmt::Result is the same
 *                                            Result<(), fmt::Error>
 *                                            OK(()), or Err(...)
 */

impl fmt::Display for Bitboard {

    // minimal bb_print with rank and file label
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} \t", rank + 1)?;
            //     ^ as described, f is whatever the destination
            //     is currently formatting this
            //     it is like print! but with "f"
            //
            //     ? handles the error case.
            //     If it fails, stop at there.
            //
            //     Thus you can just return Ok(()) at the end.

            for file in 0..8 {
                let i = rank * 8 + file;
                let bit = (self.0 >> i) & 1;
                write!(f, "{} ", bit)?;
            }

            writeln!(f)?; // vertical padding
        }

        write!(f, "    ")?; // horizontal padding

        for j in b'A'..=b'H' {
            write!(f, "{} ", j as char)?;
        }

        Ok(())
    }
}
