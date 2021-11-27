use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct Bitboard {
    val: u64,
}

fn get_ones(mut x: u64) -> Vec<usize> {
    let mut res = Vec::new();
    while x != 0 {
        let ind = x.trailing_zeros() as usize;
        res.push(ind);
        x -= 1 << ind;
    }
    res
}

impl Bitboard {
    pub fn new(val: u64) -> Self {
        Self { val }
    }
    // is 1?
    pub fn is_set(&self, i: usize, j: usize) -> bool {
        self.val & (1 << (i * 8 + j)) != 0
    }
    // set to 1
    pub fn set(&mut self, i: usize, j: usize) {
        self.val |= 1 << (i * 8 + j);
    }
    // set to 0
    pub fn unset(&mut self, i: usize, j: usize) {
        self.val &= !(1 << (i * 8 + j));
    }
    // switch 0->1 and 1->0
    pub fn toogle(&mut self, i: usize, j: usize) {
        self.val ^= 1 << (i * 8 + j);
    }
    // focus on set positions, create all combinations of them, including all being unset
    pub fn generate_subsets(&self) -> Vec<Bitboard> {
        let set_bits = self.val.count_ones() as usize;
        let ones_indexes = get_ones(self.val);
        assert_eq!(set_bits, ones_indexes.len());
        let mut res = Vec::new();
        for i in 0..((1 as usize) << set_bits) {
            let mut new_val: u64 = 0;
            for (j, one_ind) in ones_indexes.iter().enumerate() {
                if (i & (1 << j)) != 0 {
                    new_val |= 1 << one_ind;
                }
            }
            res.push(Bitboard::new(new_val));
        }
        res
    }
}

// creates empty bitboard
impl Default for Bitboard {
    fn default() -> Self {
        Self { val: 0 }
    }
}

impl_op_ex!(+ |a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val + b.val) });
impl_op_ex!(-|a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val - b.val) });
impl_op_ex!(*|a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val * b.val) });
impl_op_ex!(/ |a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val / b.val) });

impl_op_ex!(&|a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val & b.val) });
impl_op_ex!(| |a: Bitboard, b: Bitboard| -> Bitboard { Self::new(a.val | b.val) });
impl_op_ex!(!|a: Bitboard| -> Bitboard { Self::new(!a.val) });

impl_op_ex!(<< |x: Bitboard, shift: usize| -> Bitboard { Self::new(x.val << shift) });
impl_op_ex!(>> |x: Bitboard, shift: usize| -> Bitboard { Self::new(x.val >> shift) });

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bitboard_as_bytes: [u8; 8] = self.val.to_be_bytes();
        for b in bitboard_as_bytes.iter() {
            write!(f, "{:#010b}\n", b.reverse_bits()).unwrap();
        }
        write!(f, "")
    }
}
