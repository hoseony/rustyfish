use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not, BitAndAssign, BitOrAssign, BitXorAssign};

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

// ---------------- CONST ---------------
pub const FILE_A: u64 = 72340172838076673;
pub const FILE_B: u64 = 144680345676153346;
pub const FILE_C: u64 = 289360691352306692;
pub const FILE_D: u64 = 578721382704613384;
pub const FILE_E: u64 = 1157442765409226768;
pub const FILE_F: u64 = 2314885530818453536;
pub const FILE_G: u64 = 4629771061636907072;
pub const FILE_H: u64 = 9259542123273814144;

pub const RANK_1: u64 = 255;
pub const RANK_2: u64 = 65280;
pub const RANK_3: u64 = 16711680;
pub const RANK_4: u64 = 4278190080;
pub const RANK_5: u64 = 1095216660480;
pub const RANK_6: u64 = 280375465082880;
pub const RANK_7: u64 = 71776119061217280;
pub const RANK_8: u64 = 18374686479671623680;

// ---------------- IMPL ---------------

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    /// set bit of sqidx to 1
    pub fn set_bit(&mut self, sqidx: u8) {
        self.0 |= 1u64 << sqidx;
    }

    /// set bit of sqidx to 0
    pub fn remove_bit(&mut self, sqidx: u8) {
        self.0 &= !(1u64 << sqidx)
    }

    pub fn move_bit(&mut self, from_idx: u8, to_idx: u8) {
        // you can only move it when there is a bit that is 1
        if (self.0 & (1u64 << from_idx)) != 0 { 
            self.0 &= !(1u64 << from_idx);
            self.set_bit(to_idx);
            // huh, you do not need to put self in arg, funny
        }
    }

    pub fn pop_lsb(&mut self) -> u8 {
        let sq = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        sq
    }

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

        writeln!(f)?; // vertical padding
        write!(f, "    ")?; // horizontal padding

        for j in b'A'..=b'H' {
            write!(f, "{} ", j as char)?;
        }

        writeln!(f)?; // padding at the end

        Ok(())
    }
}


// ---------------- STD::OPS ---------------

// Bitwise operations for Bitboard
impl BitAnd for Bitboard {
    type Output = Self;

    // rhs: right hand side
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self; fn bitor(self, rhs: Self) -> Self::Output { Bitboard(self.0 | rhs.0) }
}

impl BitXor for Bitboard {
    type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Bitboard(self.0 ^ rhs.0) }
}

impl Not for Bitboard {
    type Output = Self; fn not(self) -> Self::Output { Bitboard(!self.0) }
}

// Bitwise assignment
impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}
