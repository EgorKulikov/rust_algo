// use crate::{add_assign, mul_assign, sub_assign};
// use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Range, Sub, SubAssign};
// use std::slice::SliceIndex;
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct ISize(pub isize);
//
// impl Add for ISize {
//     type Output = ISize;
//
//     fn add(self, other: ISize) -> ISize {
//         ISize(self.0 + other.0)
//     }
// }
//
// impl Sub for ISize {
//     type Output = ISize;
//
//     fn sub(self, other: ISize) -> ISize {
//         ISize(self.0 - other.0)
//     }
// }
//
// impl Mul for ISize {
//     type Output = ISize;
//
//     fn mul(self, other: ISize) -> ISize {
//         ISize(self.0 * other.0)
//     }
// }
//
// add_assign!(ISize, ISize);
// sub_assign!(ISize, ISize);
// mul_assign!(ISize, ISize);
//
// macro_rules! isize_ops {
//     ($($t:ident),*) => {
//         $(
//             impl Add<$t> for ISize {
//                 type Output = ISize;
//
//                 fn add(self, rhs: $t) -> Self::Output {
//                     ISize(self.0 + rhs as isize)
//                 }
//             }
//
//             impl Sub<$t> for ISize {
//                 type Output = ISize;
//
//                 fn sub(self, rhs: $t) -> Self::Output {
//                     ISize(self.0 - rhs as isize)
//                 }
//             }
//
//             impl Mul<$t> for ISize {
//                 type Output = ISize;
//
//                 fn mul(self, rhs: $t) -> Self::Output {
//                     ISize(self.0 * rhs as isize)
//                 }
//             }
//
//             $crate::
//                 add_assign!(ISize, $t);
//             $crate::
//                 sub_assign!(ISize, $t);
//             $crate::
//                 mul_assign!(ISize, $t);
//         )*
//     };
// }
//
// isize_ops!(isize, usize);
//
// pub trait IsizeOps<R: Iterator<Item = ISize>> {
//     fn ilen(&self) -> ISize;
//     fn iincicies(&self) -> R;
//     fn fits(&self, index: ISize) -> bool;
// }
//
// impl<T> IsizeOps for [T] {
//     fn ilen(&self) -> ISize {
//         ISize(self.len() as isize)
//     }
//
//     fn iincicies(&self) -> Range<ISize> {
//         ISize(0)..self.ilen()
//     }
//
//     fn fits(&self, index: ISize) -> bool {
//         ISize(0) <= index && index < self.ilen()
//     }
// }
//
// impl<T, V> Index<ISize> for [T] {
//     type Output = T;
//
//     fn index(&self, index: ISize) -> &Self::Output {
//         self.index(index.0 as usize)
//     }
// }
//
// impl<T> IndexMut<ISize> for [T] {
//     fn index_mut(&mut self, index: ISize) -> &mut Self::Output {
//         self.index_mut(index.0 as usize)
//     }
// }
//
// impl<T> Index<ISize> for Vec<T> {
//     type Output = T;
//
//     fn index(&self, index: ISize) -> &Self::Output {
//         self.index(index.0 as usize)
//     }
// }
//
// impl<T> IndexMut<ISize> for Vec<T> {
//     fn index_mut(&mut self, index: ISize) -> &mut Self::Output {
//         self.index_mut(index.0 as usize)
//     }
// }
