use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Bitboard {
    val: u64,
}

impl Bitboard {
    pub fn new(val: u64) -> Self {
        Self { val }
    }
    pub fn is_set(&self, i: usize, j: usize) -> bool {
        self.val & (1 << (i * 8 + j)) != 0
    }
}

impl_op_ex!(+ |a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val + b.val) });
impl_op_ex!(-|a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val - b.val) });
impl_op_ex!(*|a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val * b.val) });
impl_op_ex!(/ |a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val / b.val) });

impl_op_ex!(&|a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val & b.val) });
impl_op_ex!(| |a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val | b.val) });
impl_op_ex!(!|a: Bitboard| -> Bitboard { Self::new(!a.val) });
