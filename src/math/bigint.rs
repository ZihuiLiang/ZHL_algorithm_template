// use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Neg, RangeBounds, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
// pub trait Integer: Sized + Add{
// }


// pub struct BinaryInt {
//     neg: bool,
//     v: Vec<u64>,
// }

// impl BinaryInt {
//     pub fn zero() -> Self {
//         BinaryInt { neg: false, v: vec![0] }
//     }
//     pub fn from_i64(v: i64) -> Self {
//         if v >= 0 {
//             return BinaryInt { neg: false, v: vec![v.try_into().unwrap()] };
//         }
//         BinaryInt { neg: true, v: vec![(-v).try_into().unwrap()] }
//     }
//     pub fn from_i128(v: u64) -> Self {
//         todo!();
//     }
//     pub fn clean(&mut self) {
//         while self.v.len() > 1 && self.v[self.v.len() - 1] == 0 {
//             self.v.pop();
//         }
//         if self.neg && self.is_zero() {
//             self.neg = false;
//         }
//     }
//     pub fn is_zero(&self) -> bool {
//         self.v.len() == 1 && self.v[0] == 0
//     }
//     pub fn bits(&self) -> usize {
//         todo!();
//     }
// }

// impl Integer for BinaryInt {

// }

// impl Add for BinaryInt {
//     type Output = Self;

//     fn add(self,  other: Self) -> Self {

//     }
// }