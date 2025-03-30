// C: Longest Increasing Subsequence
use crate::algo_lib::collections::bit_set::BitSet;
use crate::algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use crate::algo_lib::collections::min_max::MinimMaxim;
use crate::algo_lib::collections::slice_ext::backward::Back;
use crate::algo_lib::collections::slice_ext::bounds::Bounds;
use crate::algo_lib::collections::vec_ext::gen_vec::VecGen;
use crate::algo_lib::collections::vec_ext::inc_dec::IncDec;
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::{BoolOutput, Output};
use crate::algo_lib::misc::test_type::TaskType;
use crate::algo_lib::misc::test_type::TestType;
type PreCalc = ();
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n).dec();
    let mut in_a = BitSet::new(m);
    for i in a.copy_iter() {
        in_a.set(i);
    }
    let mut b = Vec::with_capacity(m - n);
    for i in 0..m {
        if !in_a[i] {
            b.push(i);
        }
    }
    b.reverse();
    let mut lis_f = Vec::new();
    for i in a.copy_iter() {
        let pos = lis_f.lower_bound(&i);
        if pos == lis_f.len() {
            lis_f.push(i);
        } else {
            lis_f[pos] = i;
        }
    }
    let mut lis_b = Vec::new();
    for i in a.copy_rev() {
        let pos = lis_b.lower_bound(&(-(i as i32)));
        if pos == lis_b.len() {
            lis_b.push(-(i as i32));
        } else {
            lis_b[pos] = -(i as i32);
        }
    }
    lis_b.reverse();
    let lis_b = Vec::with_gen(lis_b.len(), |i| (-lis_b[i]) as usize);
    let mut lis_ab = lis_f.len();
    if lis_f[Back(0)] < b[0] {
        lis_ab += 1;
    }
    let mut prefix = 0;
    let mut lis_ba = lis_f.len();
    if lis_b[0] > b[Back(0)] {
        lis_ba += 1;
    }
    for i in 0..=m - n {
        if lis_ab == lis_ba {
            out.print_line(true);
            b[m - n - i..].reverse();
            out.print_line(b.inc());
            return;
        }
        if i == m - n {
            break;
        }
        prefix += 1;
        prefix.maxim(lis_f.lower_bound(&b[Back(i)]) + 1);
        lis_ab.maxim(prefix);
        lis_ba.maxim(i + 1 + lis_b.len() - lis_b.lower_bound(&b[Back(i)]));
    }
    let mut lis_ab = lis_f.len();
    if lis_f[Back(0)] < b[0] {
        lis_ab += 1;
    }
    let mut prefix = 0;
    let mut lis_ba = lis_f.len();
    if lis_b[0] > b[Back(0)] {
        lis_ba += 1;
    }
    for i in 0..=m - n {
        if lis_ab == lis_ba {
            out.print_line(true);
            b[..i].reverse();
            out.print_line(b.inc());
            return;
        }
        if i == m - n {
            break;
        }
        prefix += 1;
        prefix.maxim(lis_b.len() - lis_b.lower_bound(&b[i]) + 1);
        lis_ba.maxim(prefix);
        lis_ab.maxim(i + 1 + lis_f.lower_bound(&b[i]));
    }
    out.print_line(false);
}
pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;
pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);
    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}


fn main() {
    let input = crate::algo_lib::io::input::Input::stdin();
    let output = crate::algo_lib::io::output::Output::stdout();
    run(input, output);
}
pub mod algo_lib {
pub mod collections {
pub mod bit_set {
use crate::algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, Index, ShlAssign, ShrAssign};
const TRUE: bool = true;
const FALSE: bool = false;
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct BitSet {
    data: Vec<u64>,
    len: usize,
}
impl BitSet {
    pub fn new(len: usize) -> Self {
        let data_len = if len == 0 { 0 } else { Self::index(len - 1) + 1 };
        Self {
            data: vec![0; data_len],
            len,
        }
    }
    pub fn from_slice(len: usize, set: &[usize]) -> Self {
        let mut res = Self::new(len);
        for &i in set {
            res.set(i);
        }
        res
    }
    pub fn set(&mut self, at: usize) {
        assert!(at < self.len);
        self.data[Self::index(at)].set_bit(at & 63);
    }
    pub fn unset(&mut self, at: usize) {
        assert!(at < self.len);
        self.data[Self::index(at)].unset_bit(at & 63);
    }
    pub fn change(&mut self, at: usize, value: bool) {
        if value {
            self.set(at);
        } else {
            self.unset(at);
        }
    }
    pub fn flip(&mut self, at: usize) {
        self.change(at, !self[at]);
    }
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn fill(&mut self, value: bool) {
        self.data.fill(if value { u64::MAX } else { 0 });
        if value {
            self.fix_last();
        }
    }
    pub fn is_superset(&self, other: &Self) -> bool {
        assert_eq!(self.len, other.len);
        for (we, them) in self.data.iter().copied().zip(other.data.iter().copied()) {
            if (we & them) != them {
                return false;
            }
        }
        true
    }
    pub fn is_subset(&self, other: &Self) -> bool {
        other.is_superset(self)
    }
    pub fn iter(&self) -> BitSetIter<'_> {
        self.into_iter()
    }
    fn index(at: usize) -> usize {
        at >> 6
    }
    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|x| x.count_ones() as usize).sum()
    }
    pub fn shift_or(&mut self, rhs: usize) {
        if rhs == 0 || rhs >= self.len {
            return;
        }
        let small_shift = rhs & 63;
        let big_shift = rhs >> 6;
        for i in (0..self.data.len() - big_shift).rev() {
            if small_shift != 0 && i + 1 + big_shift < self.data.len() {
                let big = self.data[i] >> (64 - small_shift);
                self.data[i + 1 + big_shift] |= big;
            }
            let small = self.data[i] << small_shift;
            self.data[i + big_shift] |= small;
        }
        self.fix_last();
    }
    pub fn not(&self) -> Self {
        let mut res = self.clone();
        for data in res.data.iter_mut() {
            *data = !*data;
        }
        res.fix_last();
        res
    }
    fn fix_last(&mut self) {
        if self.len & 63 != 0 {
            let mask = (1 << (self.len & 63)) - 1;
            *self.data.last_mut().unwrap() &= mask;
        }
    }
}
pub struct BitSetIter<'s> {
    at: usize,
    inside: usize,
    set: &'s BitSet,
}
impl Iterator for BitSetIter<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.at < self.set.data.len()
            && (self.inside == 64 || (self.set.data[self.at] >> self.inside) == 0)
        {
            self.at += 1;
            self.inside = 0;
        }
        if self.at == self.set.data.len() {
            None
        } else {
            self.inside
                += (self.set.data[self.at] >> self.inside).trailing_zeros() as usize;
            let res = self.at * 64 + self.inside;
            self.inside += 1;
            Some(res)
        }
    }
}
impl<'a> IntoIterator for &'a BitSet {
    type Item = usize;
    type IntoIter = BitSetIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        BitSetIter {
            at: 0,
            inside: 0,
            set: self,
        }
    }
}
impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &BitSet) {
        assert_eq!(self.len, rhs.len);
        for (i, &j) in self.data.iter_mut().zip(rhs.data.iter()) {
            *i |= j;
        }
    }
}
impl BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &BitSet) {
        assert_eq!(self.len, rhs.len);
        for (i, &j) in self.data.iter_mut().zip(rhs.data.iter()) {
            *i &= j;
        }
    }
}
impl BitXorAssign<&BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: &BitSet) {
        assert_eq!(self.len, rhs.len);
        for (i, &j) in self.data.iter_mut().zip(rhs.data.iter()) {
            *i ^= j;
        }
    }
}
impl ShlAssign<usize> for BitSet {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        if rhs >= self.len {
            self.fill(false);
            return;
        }
        let small_shift = rhs & 63;
        if small_shift != 0 {
            let mut carry = 0;
            for data in self.data.iter_mut() {
                let new_carry = (*data) >> (64 - small_shift);
                *data <<= small_shift;
                *data |= carry;
                carry = new_carry;
            }
        }
        let big_shift = rhs >> 6;
        if big_shift != 0 {
            self.data.rotate_right(big_shift);
            self.data[..big_shift].fill(0);
        }
        self.fix_last();
    }
}
impl ShrAssign<usize> for BitSet {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        if rhs >= self.len {
            self.fill(false);
            return;
        }
        let small_shift = rhs & 63;
        if small_shift != 0 {
            let mut carry = 0;
            for data in self.data.iter_mut().rev() {
                let new_carry = (*data) << (64 - small_shift);
                *data >>= small_shift;
                *data |= carry;
                carry = new_carry;
            }
        }
        let big_shift = rhs >> 6;
        if big_shift != 0 {
            self.data.rotate_left(big_shift);
            let from = self.data.len() - big_shift;
            self.data[from..].fill(0);
        }
    }
}
impl Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, at: usize) -> &Self::Output {
        assert!(at < self.len);
        if self.data[Self::index(at)].is_set(at & 63) { &TRUE } else { &FALSE }
    }
}
impl From<Vec<bool>> for BitSet {
    fn from(data: Vec<bool>) -> Self {
        let mut res = Self::new(data.len());
        for (i, &value) in data.iter().enumerate() {
            res.change(i, value);
        }
        res
    }
}
}
pub mod iter_ext {
pub mod iter_copied {
use std::iter::{
    Chain, Copied, Enumerate, Filter, Map, Rev, Skip, StepBy, Sum, Take, Zip,
};
pub trait ItersCopied<'a, T: 'a + Copy>: Sized + 'a
where
    &'a Self: IntoIterator<Item = &'a T>,
{
    fn copy_iter(&'a self) -> Copied<<&'a Self as IntoIterator>::IntoIter> {
        self.into_iter().copied()
    }
    fn copy_enumerate(
        &'a self,
    ) -> Enumerate<Copied<<&'a Self as IntoIterator>::IntoIter>> {
        self.copy_iter().enumerate()
    }
    fn copy_rev(&'a self) -> Rev<Copied<<&'a Self as IntoIterator>::IntoIter>>
    where
        Copied<<&'a Self as IntoIterator>::IntoIter>: DoubleEndedIterator,
    {
        self.copy_iter().rev()
    }
    fn copy_skip(
        &'a self,
        n: usize,
    ) -> Skip<Copied<<&'a Self as IntoIterator>::IntoIter>> {
        self.copy_iter().skip(n)
    }
    fn copy_take(
        &'a self,
        n: usize,
    ) -> Take<Copied<<&'a Self as IntoIterator>::IntoIter>> {
        self.copy_iter().take(n)
    }
    fn copy_chain<V>(
        &'a self,
        chained: &'a V,
    ) -> Chain<
        Copied<<&'a Self as IntoIterator>::IntoIter>,
        Copied<<&'a V as IntoIterator>::IntoIter>,
    >
    where
        &'a V: IntoIterator<Item = &'a T>,
    {
        self.copy_iter().chain(chained.into_iter().copied())
    }
    fn copy_zip<V>(
        &'a self,
        other: &'a V,
    ) -> Zip<
        Copied<<&'a Self as IntoIterator>::IntoIter>,
        Copied<<&'a V as IntoIterator>::IntoIter>,
    >
    where
        &'a V: IntoIterator<Item = &'a T>,
    {
        self.copy_iter().zip(other.into_iter().copied())
    }
    fn copy_max(&'a self) -> T
    where
        T: Ord,
    {
        self.copy_iter().max().unwrap()
    }
    fn copy_max_by_key<B, F>(&'a self, f: F) -> T
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.copy_iter().max_by_key(f).unwrap()
    }
    fn copy_min(&'a self) -> T
    where
        T: Ord,
    {
        self.copy_iter().min().unwrap()
    }
    fn copy_min_by_key<B, F>(&'a self, f: F) -> T
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.copy_iter().min_by_key(f).unwrap()
    }
    fn copy_sum(&'a self) -> T
    where
        T: Sum<T>,
    {
        self.copy_iter().sum()
    }
    fn copy_map<F, U>(
        &'a self,
        f: F,
    ) -> Map<Copied<<&'a Self as IntoIterator>::IntoIter>, F>
    where
        F: FnMut(T) -> U,
    {
        self.copy_iter().map(f)
    }
    fn copy_all(&'a self, f: impl FnMut(T) -> bool) -> bool {
        self.copy_iter().all(f)
    }
    fn copy_any(&'a self, f: impl FnMut(T) -> bool) -> bool {
        self.copy_iter().any(f)
    }
    fn copy_step_by(
        &'a self,
        step: usize,
    ) -> StepBy<Copied<<&'a Self as IntoIterator>::IntoIter>> {
        self.copy_iter().step_by(step)
    }
    fn copy_filter<F: FnMut(&T) -> bool>(
        &'a self,
        f: F,
    ) -> Filter<Copied<<&'a Self as IntoIterator>::IntoIter>, F> {
        self.copy_iter().filter(f)
    }
    fn copy_fold<Acc, F>(&'a self, init: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, T) -> Acc,
    {
        self.copy_iter().fold(init, f)
    }
    fn copy_reduce<F>(&'a self, f: F) -> Option<T>
    where
        F: FnMut(T, T) -> T,
    {
        self.copy_iter().reduce(f)
    }
    fn copy_position<P>(&'a self, predicate: P) -> Option<usize>
    where
        P: FnMut(T) -> bool,
    {
        self.copy_iter().position(predicate)
    }
    fn copy_find(&'a self, val: T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.copy_iter().position(|x| x == val)
    }
    fn copy_count(&'a self, val: T) -> usize
    where
        T: PartialEq,
    {
        self.copy_iter().filter(|&x| x == val).count()
    }
}
impl<'a, U: 'a, T: 'a + Copy> ItersCopied<'a, T> for U
where
    &'a U: IntoIterator<Item = &'a T>,
{}
}
}
pub mod min_max {
pub trait MinimMaxim<Rhs = Self>: PartialOrd + Sized {
    fn minim(&mut self, other: Rhs) -> bool;
    fn maxim(&mut self, other: Rhs) -> bool;
}
impl<T: PartialOrd> MinimMaxim for T {
    fn minim(&mut self, other: Self) -> bool {
        if other < *self {
            *self = other;
            true
        } else {
            false
        }
    }
    fn maxim(&mut self, other: Self) -> bool {
        if other > *self {
            *self = other;
            true
        } else {
            false
        }
    }
}
impl<T: PartialOrd> MinimMaxim<T> for Option<T> {
    fn minim(&mut self, other: T) -> bool {
        match self {
            None => {
                *self = Some(other);
                true
            }
            Some(v) => v.minim(other),
        }
    }
    fn maxim(&mut self, other: T) -> bool {
        match self {
            None => {
                *self = Some(other);
                true
            }
            Some(v) => v.maxim(other),
        }
    }
}
}
pub mod slice_ext {
pub mod backward {
use std::ops::{Index, IndexMut};
pub struct Back(pub usize);
impl<T> Index<Back> for [T] {
    type Output = T;
    fn index(&self, index: Back) -> &Self::Output {
        &self[self.len() - index.0 - 1]
    }
}
impl<T> IndexMut<Back> for [T] {
    fn index_mut(&mut self, index: Back) -> &mut Self::Output {
        &mut self[self.len() - index.0 - 1]
    }
}
impl<T> Index<Back> for Vec<T> {
    type Output = T;
    fn index(&self, index: Back) -> &Self::Output {
        self.as_slice().index(index)
    }
}
impl<T> IndexMut<Back> for Vec<T> {
    fn index_mut(&mut self, index: Back) -> &mut Self::Output {
        self.as_mut_slice().index_mut(index)
    }
}
}
pub mod bounds {
use std::ops::{Bound, RangeBounds};
pub trait Bounds<T: PartialOrd> {
    fn lower_bound(&self, el: &T) -> usize;
    fn upper_bound(&self, el: &T) -> usize;
    fn bin_search(&self, el: &T) -> Option<usize>;
    fn more(&self, el: &T) -> usize;
    fn more_or_eq(&self, el: &T) -> usize;
    fn less(&self, el: &T) -> usize {
        self.lower_bound(el)
    }
    fn less_or_eq(&self, el: &T) -> usize {
        self.upper_bound(el)
    }
    fn inside<'a>(&self, bounds: impl RangeBounds<&'a T>) -> usize
    where
        T: 'a;
}
impl<T: PartialOrd> Bounds<T> for [T] {
    fn lower_bound(&self, el: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if &self[mid] < el {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
    fn upper_bound(&self, el: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if &self[mid] <= el {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
    fn bin_search(&self, el: &T) -> Option<usize> {
        let at = self.lower_bound(el);
        if at == self.len() || &self[at] != el { None } else { Some(at) }
    }
    fn more(&self, el: &T) -> usize {
        self.len() - self.upper_bound(el)
    }
    fn more_or_eq(&self, el: &T) -> usize {
        self.len() - self.lower_bound(el)
    }
    fn inside<'a>(&self, bounds: impl RangeBounds<&'a T>) -> usize
    where
        T: 'a,
    {
        let to = match bounds.end_bound() {
            Bound::Included(el) => self.less_or_eq(el),
            Bound::Excluded(el) => self.less(el),
            Bound::Unbounded => self.len(),
        };
        let from = match bounds.start_bound() {
            Bound::Included(el) => self.less(el),
            Bound::Excluded(el) => self.less_or_eq(el),
            Bound::Unbounded => 0,
        };
        to - from
    }
}
}
}
pub mod vec_ext {
pub mod default {
pub fn default_vec<T: Default>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(T::default());
    }
    v
}
}
pub mod gen_vec {
use crate::algo_lib::collections::vec_ext::default::default_vec;
pub trait VecGen<T> {
    fn with_gen(n: usize, f: impl FnMut(usize) -> T) -> Vec<T>;
    fn with_gen_prefix(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec<T>;
    fn gen_append(&mut self, n: usize, f: impl FnMut(usize, &Self) -> T);
    fn with_gen_back(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec<T>
    where
        T: Default;
}
impl<T> VecGen<T> for Vec<T> {
    fn with_gen(n: usize, mut f: impl FnMut(usize) -> T) -> Vec<T> {
        Self::with_gen_prefix(n, |i, _| f(i))
    }
    fn with_gen_prefix(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec<T> {
        let mut vec = Vec::with_capacity(n);
        vec.gen_append(n, f);
        vec
    }
    fn gen_append(&mut self, n: usize, mut f: impl FnMut(usize, &Self) -> T) {
        self.reserve(n);
        let len = self.len();
        for i in 0..n {
            self.push(f(len + i, self));
        }
    }
    fn with_gen_back(n: usize, mut f: impl FnMut(usize, &Self) -> T) -> Vec<T>
    where
        T: Default,
    {
        let mut vec = default_vec(n);
        for i in (0..n).rev() {
            vec[i] = f(i, &vec);
        }
        vec
    }
}
}
pub mod inc_dec {
use crate::algo_lib::numbers::num_traits::algebra::{AdditionMonoidWithSub, One};
pub trait IncDec {
    #[must_use]
    fn inc(self) -> Self;
    #[must_use]
    fn dec(self) -> Self;
}
impl<T: AdditionMonoidWithSub + One> IncDec for T {
    fn inc(self) -> Self {
        self + T::one()
    }
    fn dec(self) -> Self {
        self - T::one()
    }
}
impl<T: AdditionMonoidWithSub + One> IncDec for Vec<T> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|i| *i += T::one());
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|i| *i -= T::one());
        self
    }
}
impl<T: AdditionMonoidWithSub + One> IncDec for Vec<Vec<T>> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|v| v.iter_mut().for_each(|i| *i += T::one()));
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|v| v.iter_mut().for_each(|i| *i -= T::one()));
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One> IncDec
for Vec<(T, U)> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j)| {
                *i += T::one();
                *j += U::one();
            });
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j)| {
                *i -= T::one();
                *j -= U::one();
            });
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V> IncDec
for Vec<(T, U, V)> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, _)| {
                *i += T::one();
                *j += U::one();
            });
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, _)| {
                *i -= T::one();
                *j -= U::one();
            });
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V, W> IncDec
for Vec<(T, U, V, W)> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, ..)| {
                *i += T::one();
                *j += U::one();
            });
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, ..)| {
                *i -= T::one();
                *j -= U::one();
            });
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V, W, X> IncDec
for Vec<(T, U, V, W, X)> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, ..)| {
                *i += T::one();
                *j += U::one();
            });
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, ..)| {
                *i -= T::one();
                *j -= U::one();
            });
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One> IncDec for (T, U) {
    fn inc(mut self) -> Self {
        self.0 += T::one();
        self.1 += U::one();
        self
    }
    fn dec(mut self) -> Self {
        self.0 -= T::one();
        self.1 -= U::one();
        self
    }
}
}
}
}
pub mod io {
pub mod input {
use std::fs::File;
use std::io::{Read, Stdin};
use std::mem::MaybeUninit;
enum InputSource {
    Stdin(Stdin),
    File(File),
    Slice,
    Delegate(Box<dyn Read + Send>),
}
pub struct Input {
    input: InputSource,
    buf: Vec<u8>,
    at: usize,
    buf_read: usize,
    eol: bool,
}
macro_rules! read_impl {
    ($t:ty, $read_name:ident, $read_vec_name:ident) => {
        pub fn $read_name (& mut self) -> $t { self.read() } pub fn $read_vec_name (& mut
        self, len : usize) -> Vec <$t > { self.read_vec(len) }
    };
    ($t:ty, $read_name:ident, $read_vec_name:ident, $read_pair_vec_name:ident) => {
        read_impl!($t, $read_name, $read_vec_name); pub fn $read_pair_vec_name (& mut
        self, len : usize) -> Vec < ($t, $t) > { self.read_vec(len) }
    };
}
impl Input {
    const DEFAULT_BUF_SIZE: usize = 4096;
    pub fn slice(input: &[u8]) -> Self {
        Self {
            input: InputSource::Slice,
            buf: input.to_vec(),
            at: 0,
            buf_read: input.len(),
            eol: true,
        }
    }
    pub fn stdin() -> Self {
        Self {
            input: InputSource::Stdin(std::io::stdin()),
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
            eol: true,
        }
    }
    pub fn file(file: File) -> Self {
        Self {
            input: InputSource::File(file),
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
            eol: true,
        }
    }
    pub fn delegate(reader: impl Read + Send + 'static) -> Self {
        Self {
            input: InputSource::Delegate(Box::new(reader)),
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
            eol: true,
        }
    }
    pub fn get(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            self.at += 1;
            if res == b'\r' {
                self.eol = true;
                if self.refill_buffer() && self.buf[self.at] == b'\n' {
                    self.at += 1;
                }
                return Some(b'\n');
            }
            self.eol = res == b'\n';
            Some(res)
        } else {
            None
        }
    }
    pub fn peek(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            Some(if res == b'\r' { b'\n' } else { res })
        } else {
            None
        }
    }
    pub fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if !b.is_ascii_whitespace() {
                return;
            }
            self.get();
        }
    }
    pub fn next_token(&mut self) -> Option<Vec<u8>> {
        self.skip_whitespace();
        let mut res = Vec::new();
        while let Some(c) = self.get() {
            if c.is_ascii_whitespace() {
                break;
            }
            res.push(c);
        }
        if res.is_empty() { None } else { Some(res) }
    }
    pub fn is_exhausted(&mut self) -> bool {
        self.peek().is_none()
    }
    pub fn is_empty(&mut self) -> bool {
        self.skip_whitespace();
        self.is_exhausted()
    }
    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }
    pub fn read_vec<T: Readable>(&mut self, size: usize) -> Vec<T> {
        let mut res = Vec::with_capacity(size);
        for _ in 0..size {
            res.push(self.read());
        }
        res
    }
    pub fn read_char(&mut self) -> u8 {
        self.skip_whitespace();
        self.get().unwrap()
    }
    read_impl!(u32, read_unsigned, read_unsigned_vec);
    read_impl!(u64, read_u64, read_u64_vec);
    read_impl!(usize, read_size, read_size_vec, read_size_pair_vec);
    read_impl!(i32, read_int, read_int_vec, read_int_pair_vec);
    read_impl!(i64, read_long, read_long_vec, read_long_pair_vec);
    read_impl!(i128, read_i128, read_i128_vec);
    fn refill_buffer(&mut self) -> bool {
        if self.at == self.buf_read {
            self.at = 0;
            self.buf_read = match &mut self.input {
                InputSource::Stdin(stdin) => stdin.read(&mut self.buf).unwrap(),
                InputSource::File(file) => file.read(&mut self.buf).unwrap(),
                InputSource::Delegate(reader) => reader.read(&mut self.buf).unwrap(),
                InputSource::Slice => 0,
            };
            self.buf_read != 0
        } else {
            true
        }
    }
    pub fn is_eol(&self) -> bool {
        self.eol
    }
}
pub trait Readable {
    fn read(input: &mut Input) -> Self;
}
impl Readable for u8 {
    fn read(input: &mut Input) -> Self {
        input.read_char()
    }
}
impl<T: Readable> Readable for Vec<T> {
    fn read(input: &mut Input) -> Self {
        let size = input.read();
        input.read_vec(size)
    }
}
impl<T: Readable, const SIZE: usize> Readable for [T; SIZE] {
    fn read(input: &mut Input) -> Self {
        unsafe {
            let mut res = MaybeUninit::<[T; SIZE]>::uninit();
            for i in 0..SIZE {
                let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add(i).write(input.read::<T>());
            }
            res.assume_init()
        }
    }
}
macro_rules! read_integer {
    ($($t:ident)+) => {
        $(impl Readable for $t { fn read(input : & mut Input) -> Self { input
        .skip_whitespace(); let mut c = input.get().unwrap(); let sgn = match c { b'-' =>
        { c = input.get().unwrap(); true } b'+' => { c = input.get().unwrap(); false } _
        => false, }; let mut res = 0; loop { assert!(c.is_ascii_digit()); res *= 10; let
        d = (c - b'0') as $t; if sgn { res -= d; } else { res += d; } match input.get() {
        None => break, Some(ch) => { if ch.is_ascii_whitespace() { break; } else { c =
        ch; } } } } res } })+
    };
}
read_integer!(i8 i16 i32 i64 i128 isize u16 u32 u64 u128 usize);
macro_rules! tuple_readable {
    ($($name:ident)+) => {
        impl <$($name : Readable),+> Readable for ($($name,)+) { fn read(input : & mut
        Input) -> Self { ($($name ::read(input),)+) } }
    };
}
tuple_readable! {
    T
}
tuple_readable! {
    T U
}
tuple_readable! {
    T U V
}
tuple_readable! {
    T U V X
}
tuple_readable! {
    T U V X Y
}
tuple_readable! {
    T U V X Y Z
}
tuple_readable! {
    T U V X Y Z A
}
tuple_readable! {
    T U V X Y Z A B
}
tuple_readable! {
    T U V X Y Z A B C
}
tuple_readable! {
    T U V X Y Z A B C D
}
tuple_readable! {
    T U V X Y Z A B C D E
}
tuple_readable! {
    T U V X Y Z A B C D E F
}
}
pub mod output {
use std::cmp::Reverse;
use std::fs::File;
use std::io::{Stdout, Write};
#[derive(Copy, Clone)]
pub enum BoolOutput {
    YesNo,
    YesNoCaps,
    PossibleImpossible,
    Custom(&'static str, &'static str),
}
impl BoolOutput {
    pub fn output(&self, output: &mut Output, val: bool) {
        (if val { self.yes() } else { self.no() }).write(output);
    }
    fn yes(&self) -> &str {
        match self {
            BoolOutput::YesNo => "Yes",
            BoolOutput::YesNoCaps => "YES",
            BoolOutput::PossibleImpossible => "Possible",
            BoolOutput::Custom(yes, _) => yes,
        }
    }
    fn no(&self) -> &str {
        match self {
            BoolOutput::YesNo => "No",
            BoolOutput::YesNoCaps => "NO",
            BoolOutput::PossibleImpossible => "Impossible",
            BoolOutput::Custom(_, no) => no,
        }
    }
}
enum OutputDest<'s> {
    Stdout(Stdout),
    File(File),
    Buf(&'s mut Vec<u8>),
    Delegate(Box<dyn Write + 's>),
}
pub struct Output<'s> {
    output: OutputDest<'s>,
    buf: Vec<u8>,
    at: usize,
    bool_output: BoolOutput,
    precision: Option<usize>,
    separator: u8,
}
impl<'s> Output<'s> {
    pub fn buf(buf: &'s mut Vec<u8>) -> Self {
        Self::new(OutputDest::Buf(buf))
    }
    pub fn delegate(delegate: impl Write + 'static) -> Self {
        Self::new(OutputDest::Delegate(Box::new(delegate)))
    }
    fn new(output: OutputDest<'s>) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            bool_output: BoolOutput::YesNoCaps,
            precision: None,
            separator: b' ',
        }
    }
}
impl Output<'static> {
    pub fn stdout() -> Self {
        Self::new(OutputDest::Stdout(std::io::stdout()))
    }
    pub fn file(file: File) -> Self {
        Self::new(OutputDest::File(file))
    }
}
impl Output<'_> {
    const DEFAULT_BUF_SIZE: usize = 4096;
    pub fn flush(&mut self) {
        if self.at != 0 {
            match &mut self.output {
                OutputDest::Stdout(stdout) => {
                    stdout.write_all(&self.buf[..self.at]).unwrap();
                    stdout.flush().unwrap();
                }
                OutputDest::File(file) => {
                    file.write_all(&self.buf[..self.at]).unwrap();
                    file.flush().unwrap();
                }
                OutputDest::Buf(buf) => buf.extend_from_slice(&self.buf[..self.at]),
                OutputDest::Delegate(delegate) => {
                    delegate.write_all(&self.buf[..self.at]).unwrap();
                    delegate.flush().unwrap();
                }
            }
            self.at = 0;
        }
    }
    pub fn print<T: Writable>(&mut self, s: T) {
        s.write(self);
    }
    pub fn print_line<T: Writable>(&mut self, s: T) {
        self.print(s);
        self.put(b'\n');
    }
    pub fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }
    pub fn print_per_line<T: Writable>(&mut self, arg: &[T]) {
        self.print_per_line_iter(arg.iter());
    }
    pub fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(self.separator);
            }
            e.write(self);
        }
    }
    pub fn print_line_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        self.print_iter(iter);
        self.put(b'\n');
    }
    pub fn print_per_line_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        for e in iter {
            e.write(self);
            self.put(b'\n');
        }
    }
    pub fn set_bool_output(&mut self, bool_output: BoolOutput) {
        self.bool_output = bool_output;
    }
    pub fn set_precision(&mut self, precision: usize) {
        self.precision = Some(precision);
    }
    pub fn reset_precision(&mut self) {
        self.precision = None;
    }
    pub fn get_precision(&self) -> Option<usize> {
        self.precision
    }
    pub fn separator(&self) -> u8 {
        self.separator
    }
    pub fn set_separator(&mut self, separator: u8) {
        self.separator = separator;
    }
}
impl Write for Output<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut start = 0usize;
        let mut rem = buf.len();
        while rem > 0 {
            let len = (self.buf.len() - self.at).min(rem);
            self.buf[self.at..self.at + len].copy_from_slice(&buf[start..start + len]);
            self.at += len;
            if self.at == self.buf.len() {
                self.flush();
            }
            start += len;
            rem -= len;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.flush();
        Ok(())
    }
}
pub trait Writable {
    fn write(&self, output: &mut Output);
}
impl Writable for &str {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}
impl Writable for String {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}
impl Writable for char {
    fn write(&self, output: &mut Output) {
        output.put(*self as u8);
    }
}
impl Writable for u8 {
    fn write(&self, output: &mut Output) {
        output.put(*self);
    }
}
impl<T: Writable> Writable for [T] {
    fn write(&self, output: &mut Output) {
        output.print_iter(self.iter());
    }
}
impl<T: Writable, const N: usize> Writable for [T; N] {
    fn write(&self, output: &mut Output) {
        output.print_iter(self.iter());
    }
}
impl<T: Writable + ?Sized> Writable for &T {
    fn write(&self, output: &mut Output) {
        T::write(self, output)
    }
}
impl<T: Writable> Writable for Vec<T> {
    fn write(&self, output: &mut Output) {
        self.as_slice().write(output);
    }
}
impl Writable for () {
    fn write(&self, _output: &mut Output) {}
}
macro_rules! write_to_string {
    ($($t:ident)+) => {
        $(impl Writable for $t { fn write(& self, output : & mut Output) { self
        .to_string().write(output); } })+
    };
}
write_to_string!(u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
macro_rules! tuple_writable {
    ($name0:ident $($name:ident : $id:tt)*) => {
        impl <$name0 : Writable, $($name : Writable,)*> Writable for ($name0, $($name,)*)
        { fn write(& self, out : & mut Output) { self.0.write(out); $(out.put(out
        .separator); self.$id .write(out);)* } }
    };
}
tuple_writable! {
    T
}
tuple_writable! {
    T U : 1
}
tuple_writable! {
    T U : 1 V : 2
}
tuple_writable! {
    T U : 1 V : 2 X : 3
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4 Z : 5
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6 B : 7
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6 B : 7 C : 8
}
impl<T: Writable> Writable for Option<T> {
    fn write(&self, output: &mut Output) {
        match self {
            None => (-1).write(output),
            Some(t) => t.write(output),
        }
    }
}
impl Writable for bool {
    fn write(&self, output: &mut Output) {
        let bool_output = output.bool_output;
        bool_output.output(output, *self)
    }
}
impl<T: Writable> Writable for Reverse<T> {
    fn write(&self, output: &mut Output) {
        self.0.write(output);
    }
}
}
}
pub mod misc {
pub mod test_type {
pub enum TestType {
    Single,
    MultiNumber,
    MultiEof,
}
pub enum TaskType {
    Classic,
    Interactive,
}
}
}
pub mod numbers {
pub mod num_traits {
pub mod algebra {
use crate::algo_lib::numbers::num_traits::invertible::Invertible;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
pub trait AdditionMonoid: Add<Output = Self> + AddAssign + Zero + Eq + Sized {}
impl<T: Add<Output = Self> + AddAssign + Zero + Eq> AdditionMonoid for T {}
pub trait AdditionMonoidWithSub: AdditionMonoid + Sub<Output = Self> + SubAssign {}
impl<T: AdditionMonoid + Sub<Output = Self> + SubAssign> AdditionMonoidWithSub for T {}
pub trait AdditionGroup: AdditionMonoidWithSub + Neg<Output = Self> {}
impl<T: AdditionMonoidWithSub + Neg<Output = Self>> AdditionGroup for T {}
pub trait MultiplicationMonoid: Mul<Output = Self> + MulAssign + One + Eq + Sized {}
impl<T: Mul<Output = Self> + MulAssign + One + Eq> MultiplicationMonoid for T {}
pub trait IntegerMultiplicationMonoid: MultiplicationMonoid + Div<
        Output = Self,
    > + Rem<Output = Self> + DivAssign + RemAssign {}
impl<
    T: MultiplicationMonoid + Div<Output = Self> + Rem<Output = Self> + DivAssign
        + RemAssign,
> IntegerMultiplicationMonoid for T {}
pub trait MultiplicationGroup: MultiplicationMonoid + Div<
        Output = Self,
    > + DivAssign + Invertible<Output = Self> {}
impl<
    T: MultiplicationMonoid + Div<Output = Self> + DivAssign + Invertible<Output = Self>,
> MultiplicationGroup for T {}
pub trait SemiRing: AdditionMonoid + MultiplicationMonoid {}
impl<T: AdditionMonoid + MultiplicationMonoid> SemiRing for T {}
pub trait SemiRingWithSub: AdditionMonoidWithSub + SemiRing {}
impl<T: AdditionMonoidWithSub + SemiRing> SemiRingWithSub for T {}
pub trait Ring: SemiRing + AdditionGroup {}
impl<T: SemiRing + AdditionGroup> Ring for T {}
pub trait IntegerSemiRing: SemiRing + IntegerMultiplicationMonoid {}
impl<T: SemiRing + IntegerMultiplicationMonoid> IntegerSemiRing for T {}
pub trait IntegerSemiRingWithSub: SemiRingWithSub + IntegerSemiRing {}
impl<T: SemiRingWithSub + IntegerSemiRing> IntegerSemiRingWithSub for T {}
pub trait IntegerRing: IntegerSemiRing + Ring {}
impl<T: IntegerSemiRing + Ring> IntegerRing for T {}
pub trait Field: Ring + MultiplicationGroup {}
impl<T: Ring + MultiplicationGroup> Field for T {}
macro_rules! zero_one_integer_impl {
    ($($t:ident)+) => {
        $(impl Zero for $t { fn zero() -> Self { 0 } } impl One for $t { fn one() -> Self
        { 1 } })+
    };
}
zero_one_integer_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
}
pub mod bit_ops {
use std::ops::RangeInclusive;
pub trait BitOps: Sized {
    const BITS: usize;
    fn bit(at: usize) -> Self;
    fn is_set(&self, at: usize) -> bool;
    fn set_bit(&mut self, at: usize);
    fn unset_bit(&mut self, at: usize);
    fn with_bit(self, at: usize) -> Self;
    fn without_bit(self, at: usize) -> Self;
    fn flip_bit(&mut self, at: usize);
    fn flipped_bit(self, at: usize) -> Self;
    fn all_bits(n: usize) -> Self;
    fn iter_all(n: usize) -> RangeInclusive<Self>;
    fn lowest_bit(&self) -> usize;
    fn highest_bit(&self) -> usize;
}
pub struct BitIter<T> {
    cur: T,
    all: T,
    ended: bool,
}
impl<T: Copy> BitIter<T> {
    pub fn new(all: T) -> Self {
        Self {
            cur: all,
            all,
            ended: false,
        }
    }
}
macro_rules! bit_ops {
    ($($t:ty : $b:literal)+) => {
        $(impl BitOps for $t { const BITS : usize = $b; fn bit(at : usize) -> $t { 1 <<
        at } fn is_set(& self, at : usize) -> bool { (* self >> at & 1) == 1 } fn
        set_bit(& mut self, at : usize) { * self |= 1 << at } fn unset_bit(& mut self, at
        : usize) { * self &= ! (1 << at) } fn with_bit(mut self, at : usize) -> $t { self
        .set_bit(at); self } fn without_bit(mut self, at : usize) -> $t { self
        .unset_bit(at); self } fn flip_bit(& mut self, at : usize) { * self ^= 1 << at }
        fn flipped_bit(mut self, at : usize) -> $t { self.flip_bit(at); self } fn
        all_bits(n : usize) -> $t { (1 as $t << n).wrapping_sub(1) } fn iter_all(n :
        usize) -> RangeInclusive <$t > { 0..= Self::all_bits(n) } fn lowest_bit(& self)
        -> usize { assert_ne!(* self, 0); self.trailing_zeros() as usize } fn
        highest_bit(& self) -> usize { assert_ne!(* self, 0); Self::BITS as usize - 1 -
        self.leading_zeros() as usize } } impl Iterator for BitIter <$t > { type Item =
        $t; fn next(& mut self) -> Option < Self::Item > { if self.ended { return None; }
        let res = self.cur; if self.cur == 0 { self.ended = true; } else { self.cur =
        (self.cur - 1) & self.all; } Some(res) } })+
    };
}
bit_ops!(
    u8 : 8 u16 : 16 u32 : 32 u64 : 64 u128 : 128 usize : 64 i8 : 8 i16 : 16 i32 : 32 i64
    : 64 i128 : 128 isize : 64
);
}
pub mod invertible {
pub trait Invertible {
    type Output;
    fn inv(&self) -> Option<Self::Output>;
}
}
}
}
}
