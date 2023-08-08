// use crate::numbers::num_traits::zero_one::ZeroOne;
// use crate::numbers::unsigned_big_int::UBigInt;
// use crate::string::string::Str;
// use std::ops::AddAssign;
//
// pub struct BigInt {
//     value: UBigInt,
//     sign: i8,
// }
//
// impl From<Str<'_>> for BigInt {
//     fn from(value: Str) -> Self {
//         if value[0] == b'-' {
//             Self {
//                 value: Str::from(&value[1..]).into(),
//                 sign: -1,
//             }
//         } else {
//             Self {
//                 value: value.into(),
//                 sign: if value.len() == 1 && value[0] == b'0' {
//                     0
//                 } else {
//                     1
//                 },
//             }
//         }
//     }
// }
//
// impl From<i32> for BigInt {
//     fn from(mut v: i32) -> Self {
//         if v >= 0 {
//             Self {
//                 value: (v as u32).into(),
//                 sign: if v == 0 { 0 } else { 1 },
//             }
//         } else {
//             Self {
//                 value: (-v as u32).into(),
//                 sign: -1,
//             }
//         }
//     }
// }
//
// impl ZeroOne for UBigInt {
//     fn zero() -> Self {
//         Self::from(0)
//     }
//
//     fn one() -> Self {
//         Self::from(1)
//     }
// }
//
// impl AddAssign for BigInt {
//     fn add_assign(&mut self, rhs: Self) {
//         self.add_assign(&rhs);
//     }
// }
//
// impl<'a> AddAssign<&'a Self> for BigInt {
//     fn add_assign(&mut self, rhs: &'a Self) {
//         match self.sign {}
//     }
// }
//
// impl Add for UBigInt {
//     type Output = Self;
//
//     fn add(mut self, rhs: Self) -> Self::Output {
//         self += rhs;
//         self
//     }
// }
//
// impl<'a> Add<&'a Self> for UBigInt {
//     type Output = Self;
//
//     fn add(mut self, rhs: &'a Self) -> Self::Output {
//         self += rhs;
//         self
//     }
// }
//
// impl<'a> SubAssign<&'a Self> for UBigInt {
//     fn sub_assign(&mut self, rhs: &'a Self) {
//         assert!(self.z.len() >= rhs.z.len());
//         let mut carry = 0;
//         for (i, j) in self.z.iter_mut().zip(rhs.z.iter()) {
//             if *i >= j + carry {
//                 *i -= j + carry;
//                 carry = 0;
//             } else {
//                 *i += BASE - (j + carry);
//                 carry = 1;
//             }
//         }
//         if carry == 1 {
//             let mut at = rhs.z.len();
//             loop {
//                 if self.z[at] == 0 {
//                     self.z[at] = BASE - 1;
//                 } else {
//                     self.z[at] -= 1;
//                     break;
//                 }
//                 at += 1;
//             }
//         }
//         while !self.z.is_empty() && *self.z.last().unwrap() == 0 {
//             self.z.pop();
//         }
//     }
// }
//
// impl SubAssign for UBigInt {
//     fn sub_assign(&mut self, rhs: Self) {
//         self.sub_assign(&rhs);
//     }
// }
//
// impl<'a> Sub<&'a Self> for UBigInt {
//     type Output = Self;
//
//     fn sub(mut self, rhs: &'a Self) -> Self::Output {
//         self -= rhs;
//         self
//     }
// }
//
// impl Sub for UBigInt {
//     type Output = Self;
//
//     fn sub(mut self, rhs: Self) -> Self::Output {
//         self -= rhs;
//         self
//     }
// }
//
// impl MulAssign<u32> for UBigInt {
//     fn mul_assign(&mut self, rhs: u32) {
//         if rhs == 0 {
//             *self = Self::zero();
//             return;
//         }
//         let rhs = rhs.into_u64();
//         let mut carry = 0;
//         for i in self.z.iter_mut() {
//             let val = i.into_u64() * rhs + carry;
//             *i = (val % BASE.into_u64()).into_u32();
//             carry = val / BASE.into_u64();
//         }
//         while carry > 0 {
//             self.z.push((carry % BASE.into_u64()).into_u32());
//             carry /= BASE.into_u64();
//         }
//     }
// }
//
// impl DivAssign<u32> for UBigInt {
//     fn div_assign(&mut self, rhs: u32) {
//         let mut carry = 0;
//         for i in self.z.iter_mut().rev() {
//             let val = carry + i.into_u64();
//             *i = (val / rhs.into_u64()).into_u32();
//             carry = (val % rhs.into_u64()) * BASE.into_u64();
//         }
//         while let Some(d) = self.z.last() {
//             if *d == 0 {
//                 self.z.pop();
//             } else {
//                 break;
//             }
//         }
//     }
// }
//
// impl Writable for UBigInt {
//     fn write(&self, output: &mut Output) {
//         if let Some(tail) = self.z.last() {
//             tail.write(output);
//             for &i in self.z.iter().rev().skip(1) {
//                 format!("{:09}", i).write(output);
//             }
//         } else {
//             0u32.write(output);
//         }
//     }
// }
//
// impl ToString for UBigInt {
//     fn to_string(&self) -> String {
//         if let Some(tail) = self.z.last() {
//             let mut ans = tail.to_string();
//             for &i in self.z.iter().rev().skip(1) {
//                 ans += format!("{:09}", i).as_str();
//             }
//             ans
//         } else {
//             "0".to_string()
//         }
//     }
// }
//
// impl PartialOrd<Self> for UBigInt {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl Ord for UBigInt {
//     fn cmp(&self, other: &Self) -> Ordering {
//         if self.z.len() != other.z.len() {
//             return self.z.len().cmp(&other.z.len());
//         }
//         for (i, j) in self.z.iter().rev().zip(other.z.iter().rev()) {
//             if i != j {
//                 return i.cmp(j);
//             }
//         }
//         Ordering::Equal
//     }
// }
//
// impl Readable for UBigInt {
//     fn read(input: &mut Input) -> Self {
//         input.read_str().into()
//     }
// }
