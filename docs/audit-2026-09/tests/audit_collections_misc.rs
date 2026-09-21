#![allow(clippy::all)]
#![allow(unused_imports)]
use algo_lib::collections::array_map::ArrayMap;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::bounds::clamp;
use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::default_map::{by_index, qty, DefaultHashMap, DefaultTreeMap};
use algo_lib::collections::divided_set::DividedSet;
use algo_lib::collections::fast_clear_arr::FastClearArr;
use algo_lib::collections::fx_hash_map::{FxHashMap, FxHashSet};
use algo_lib::collections::id::Id;
use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::collections::iter_ext::cur_next::{cur_next, prev_cur_next};
use algo_lib::collections::iter_ext::interleave::interleave;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::collections::md_arr::arr4d::Arr4d;
use algo_lib::collections::md_arr::arr5d::Arr5d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::multi_set::{MultiHashSet, MultiTreeSet};
use algo_lib::collections::order::Order;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::compress::compress;
use algo_lib::collections::slice_ext::consecutive_iter::{ConsecutiveIter, ConsecutiveIterCopy};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::next_permutation::NextPermutation;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::slice_ext::radix_sort::radix_sort_by_key;
use algo_lib::collections::slice_ext::splits::Split;
use algo_lib::collections::vec_ext::default::default_vec;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::collections::vec_ext::transpose::TransposePairVec;
use algo_lib::misc::bin_search::{bin_search_real, first_true, last_false};
use algo_lib::misc::bump_alloc::{bump_alloc, bump_new};
use algo_lib::misc::direction::Direction;
use algo_lib::misc::dirs::{border, D4, D8, DK};
use algo_lib::misc::expression_parser::ExpressionParser;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::extensions::option::OptionExt;
use algo_lib::misc::extensions::replace_with::ReplaceWith;
use algo_lib::misc::lazy_lock::LazyLock;
use algo_lib::misc::memo::memoization::{Memoization, Memoization2};
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::memo::memoization_4d::Memoization4d;
use algo_lib::misc::memo::memoization_5d::Memoization5d;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::mo::{mo, MoWorker};
use algo_lib::misc::owned_cell::OwnedCell;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::zip;
use algo_lib::misc::recursive_function::{
    Callable, Callable2, Callable3, Callable4, Callable5, RecursiveFunction, RecursiveFunction2,
};
use algo_lib::numbers::real::Real;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::{BTreeMap, BTreeSet};

fn rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}

// ---------------------------------------------------------------- BitSet

fn check_bs(bs: &BitSet, model: &[bool], ctx: &str) {
    assert_eq!(bs.len(), model.len());
    for i in 0..model.len() {
        assert_eq!(bs[i], model[i], "{ctx}: bit {i}");
    }
    let ones: Vec<usize> = (0..model.len()).filter(|&i| model[i]).collect();
    assert_eq!(bs.iter().collect::<Vec<_>>(), ones, "{ctx}: iter");
    assert_eq!(bs.count_ones(), ones.len(), "{ctx}: count");
    // canonical representation (no garbage beyond len)
    assert!(*bs == BitSet::from(model.to_vec()), "{ctx}: canonical");
}

fn pick_shift(r: &mut ChaCha8Rng, len: usize) -> usize {
    match r.gen_range(0..6) {
        0 => 0,
        1 => 64 * r.gen_range(0..=len / 64 + 1),
        2 => len,
        3 => len + r.gen_range(0..70),
        4 => len.saturating_sub(1),
        _ => r.gen_range(0..=len + 2),
    }
}

#[test]
fn bit_set_stress() {
    let mut r = rng(1);
    let lens = [1usize, 2, 63, 64, 65, 100, 127, 128, 129, 191, 192, 193, 256, 300];
    for &len in &lens {
        for _ in 0..60 {
            let dens = [0.05, 0.5, 0.95][r.gen_range(0..3)];
            let mut model: Vec<bool> = (0..len).map(|_| r.gen_bool(dens)).collect();
            let mut bs = BitSet::from(model.clone());
            check_bs(&bs, &model, "init");
            for _ in 0..60 {
                let op = r.gen_range(0..14);
                let mut ctx = format!("len={len} op={op}");
                match op {
                    0 => {
                        let i = r.gen_range(0..len);
                        bs.set(i);
                        model[i] = true;
                    }
                    1 => {
                        let i = r.gen_range(0..len);
                        bs.unset(i);
                        model[i] = false;
                    }
                    2 => {
                        let i = r.gen_range(0..len);
                        bs.flip(i);
                        model[i] = !model[i];
                    }
                    3 => {
                        let i = r.gen_range(0..len);
                        assert_eq!(bs.get_and_set(i), model[i]);
                        model[i] = true;
                        let j = r.gen_range(0..len);
                        assert_eq!(bs.get_and_unset(j), model[j]);
                        model[j] = false;
                    }
                    4 => {
                        let s = pick_shift(&mut r, len);
                        ctx += &format!(" s={s}");
                        bs <<= s;
                        let mut nm = vec![false; len];
                        for i in 0..len {
                            if model[i] && i + s < len {
                                nm[i + s] = true;
                            }
                        }
                        model = nm;
                    }
                    5 => {
                        let s = pick_shift(&mut r, len);
                        ctx += &format!(" s={s}");
                        bs >>= s;
                        let mut nm = vec![false; len];
                        for i in 0..len {
                            if model[i] && i >= s {
                                nm[i - s] = true;
                            }
                        }
                        model = nm;
                    }
                    6 => {
                        let s = pick_shift(&mut r, len);
                        ctx += &format!(" s={s}");
                        bs.shift_left_or(s);
                        let mut nm = model.clone();
                        for i in 0..len {
                            if model[i] && i + s < len {
                                nm[i + s] = true;
                            }
                        }
                        model = nm;
                    }
                    7 => {
                        let s = pick_shift(&mut r, len);
                        ctx += &format!(" s={s}");
                        bs.shift_right_or(s);
                        let mut nm = model.clone();
                        for i in 0..len {
                            if model[i] && i >= s {
                                nm[i - s] = true;
                            }
                        }
                        model = nm;
                    }
                    8 | 9 | 10 => {
                        let other: Vec<bool> = (0..len).map(|_| r.gen_bool(0.5)).collect();
                        let ob = BitSet::from(other.clone());
                        for i in 0..len {
                            model[i] = match op {
                                8 => model[i] | other[i],
                                9 => model[i] & other[i],
                                _ => model[i] ^ other[i],
                            };
                        }
                        match op {
                            8 => bs |= &ob,
                            9 => bs &= &ob,
                            _ => bs ^= &ob,
                        }
                    }
                    11 => {
                        bs = bs.not();
                        for x in model.iter_mut() {
                            *x = !*x;
                        }
                    }
                    12 => {
                        let v = r.gen_bool(0.5);
                        if r.gen_bool(0.2) {
                            bs.fill(v);
                            model.fill(v);
                        }
                    }
                    _ => {
                        let other: Vec<bool> = (0..len)
                            .map(|i| if r.gen_bool(0.7) { model[i] } else { r.gen_bool(0.1) })
                            .collect();
                        let ob = BitSet::from(other.clone());
                        let sub = (0..len).all(|i| !other[i] || model[i]);
                        let sup = (0..len).all(|i| !model[i] || other[i]);
                        assert_eq!(bs.is_superset(&ob), sub);
                        assert_eq!(ob.is_subset(&bs), sub);
                        assert_eq!(bs.is_subset(&ob), sup);
                        let mut c = BitSet::new(len);
                        ob.copy_to(&mut c);
                        check_bs(&c, &other, "copy_to");
                    }
                }
                check_bs(&bs, &model, &ctx);
            }
        }
    }
    // len 0
    let mut e = BitSet::new(0);
    e.fill(true);
    e <<= 3;
    e >>= 3;
    e.shift_left_or(1);
    e.shift_right_or(1);
    assert_eq!(e.iter().count(), 0);
    assert_eq!(e.not().count_ones(), 0);
    let fs = BitSet::from_slice(130, &[0, 64, 129]);
    assert_eq!(fs.iter().collect::<Vec<_>>(), vec![0, 64, 129]);
}

// ---------------------------------------------------------------- IndexedHeap

#[test]
fn indexed_heap_stress() {
    let mut r = rng(2);
    for it in 0..3000 {
        let n = r.gen_range(1..=12);
        let maxv = [1i64, 3, 1000][r.gen_range(0..3)];
        let mut heap: IndexedHeap<i64> = IndexedHeap::new(n);
        let mut vals: Vec<Option<i64>> = vec![None; n];
        let mut set: BTreeSet<(i64, usize)> = BTreeSet::new();
        for _ in 0..80 {
            match r.gen_range(0..7) {
                0 | 1 => {
                    let el = r.gen_range(0..n);
                    let v = r.gen_range(-maxv..=maxv);
                    heap.add_or_adjust(el, v);
                    if let Some(o) = vals[el] {
                        set.remove(&(o, el));
                    }
                    vals[el] = Some(v);
                    set.insert((v, el));
                }
                2 => {
                    let el = r.gen_range(0..n);
                    let v = r.gen_range(-maxv..=maxv);
                    heap.add_or_relax(el, v);
                    match vals[el] {
                        Some(o) if o <= v => {}
                        _ => {
                            if let Some(o) = vals[el] {
                                set.remove(&(o, el));
                            }
                            vals[el] = Some(v);
                            set.insert((v, el));
                        }
                    }
                }
                3 => {
                    let el = r.gen_range(0..n);
                    let got = heap.remove(el);
                    assert_eq!(got, vals[el]);
                    if let Some(o) = vals[el] {
                        set.remove(&(o, el));
                    }
                    vals[el] = None;
                }
                4 => {
                    let got = heap.pop();
                    match set.iter().next().copied() {
                        None => assert!(got.is_none()),
                        Some((v, _)) => {
                            let (el, gv) = got.unwrap();
                            assert_eq!(gv, v, "iter {it}");
                            assert_eq!(vals[el], Some(v));
                            set.remove(&(v, el));
                            vals[el] = None;
                        }
                    }
                }
                5 => {
                    if r.gen_bool(0.05) {
                        heap.clear();
                        set.clear();
                        vals.fill(None);
                    }
                }
                _ => {}
            }
            assert_eq!(heap.len(), set.len());
            assert_eq!(heap.is_empty(), set.is_empty());
            match heap.peek() {
                None => assert!(set.is_empty()),
                Some((el, v)) => {
                    assert_eq!(*v, set.iter().next().unwrap().0);
                    assert_eq!(vals[el], Some(*v));
                }
            }
            for el in 0..n {
                assert_eq!(heap.value(el).copied(), vals[el]);
            }
            let mut ids: Vec<usize> = heap.iter().collect();
            ids.sort();
            let mut exp: Vec<usize> = set.iter().map(|x| x.1).collect();
            exp.sort();
            assert_eq!(ids, exp);
        }
    }
    let mut h: IndexedHeap<String> = IndexedHeap::new(5);
    h.add_or_adjust(0, "b".to_string());
    h.add_or_adjust(1, "a".to_string());
    h.add_or_adjust(0, "0".to_string());
    assert_eq!(h.pop(), Some((0, "0".to_string())));
    h.add_or_adjust(3, "zz".to_string());
    h.clear();
    assert!(h.pop().is_none());
    h.add_or_adjust(4, "q".to_string());
}

// ---------------------------------------------------------------- MultiSet

fn dec(model: &mut BTreeMap<i32, usize>, e: i32) {
    let c = model.get_mut(&e).unwrap();
    *c -= 1;
    if *c == 0 {
        model.remove(&e);
    }
}

#[test]
fn multi_set_stress() {
    let mut r = rng(3);
    for _ in 0..500 {
        let mut ms: MultiTreeSet<i32> = MultiTreeSet::new();
        let mut hs: MultiHashSet<i32> = MultiHashSet::new();
        let mut model: BTreeMap<i32, usize> = BTreeMap::new();
        for _ in 0..100 {
            let v = r.gen_range(-5..=5);
            match r.gen_range(0..8) {
                0 | 1 => {
                    ms.insert(v);
                    hs.insert(v);
                    *model.entry(v).or_insert(0) += 1;
                }
                2 => {
                    let q = r.gen_range(1..4);
                    ms.insert_few(v, q);
                    for _ in 0..q {
                        hs.insert(v);
                    }
                    *model.entry(v).or_insert(0) += q;
                }
                3 => {
                    let had = model.contains_key(&v);
                    assert_eq!(ms.remove(&v), had);
                    assert_eq!(hs.remove(&v), had);
                    if had {
                        dec(&mut model, v);
                    }
                }
                4 => {
                    let had = model.remove(&v).is_some();
                    assert_eq!(ms.remove_all(&v), had);
                    assert_eq!(hs.remove_all(&v), had);
                }
                5 => {
                    let exp = model.keys().next().copied();
                    assert_eq!(ms.pop_first(), exp);
                    if let Some(e) = exp {
                        hs.remove(&e);
                        dec(&mut model, e);
                    }
                }
                6 => {
                    let exp = model.keys().next_back().copied();
                    assert_eq!(ms.pop_last(), exp);
                    if let Some(e) = exp {
                        hs.remove(&e);
                        dec(&mut model, e);
                    }
                }
                _ => {
                    let a = r.gen_range(-6..=6);
                    let b = r.gen_range(a..=7);
                    let exp: Vec<i32> = model
                        .range(a..b)
                        .flat_map(|(k, c)| std::iter::repeat(*k).take(*c))
                        .collect();
                    assert_eq!(ms.range(a..b).copied().collect::<Vec<_>>(), exp);
                    let mut rev = exp.clone();
                    rev.reverse();
                    assert_eq!(ms.range_rev(a..b).copied().collect::<Vec<_>>(), rev);
                }
            }
            let total: usize = model.values().sum();
            assert_eq!(ms.len(), total);
            assert_eq!(hs.len(), total);
            assert_eq!(ms.is_empty(), total == 0);
            assert_eq!(ms.distinct(), model.len());
            assert_eq!(hs.num_distinct(), model.len());
            assert_eq!(ms.first(), model.keys().next());
            assert_eq!(ms.last(), model.keys().next_back());
            assert_eq!(ms.get(&v), model.get(&v).copied().unwrap_or(0));
            assert_eq!(ms.contains(&v), model.contains_key(&v));
            let exp: Vec<i32> = model
                .iter()
                .flat_map(|(k, c)| std::iter::repeat(*k).take(*c))
                .collect();
            assert_eq!(ms.iter().copied().collect::<Vec<_>>(), exp);
            let mut hv: Vec<i32> = hs.iter().copied().collect();
            hv.sort();
            assert_eq!(hv, exp);
        }
    }
}

// Candidate: insert_few(x, 0) leaves a phantom key with count 0.
#[test]
fn cand_multi_tree_set_insert_few_zero() {
    let mut ms: MultiTreeSet<i32> = MultiTreeSet::new();
    ms.insert_few(5, 0);
    assert_eq!(ms.len(), 0);
    assert!(!ms.contains(&5), "phantom element after insert_few(_, 0)");
    assert_eq!(ms.first(), None);
}

// ---------------------------------------------------------------- DividedSet

#[test]
fn divided_set_median() {
    let mut r = rng(4);
    for _ in 0..500 {
        let mut ds = DividedSet::new(|l, rr| {
            if l > rr + 1 {
                Some(Direction::Right)
            } else if l < rr {
                Some(Direction::Left)
            } else {
                None
            }
        });
        let mut model: Vec<i32> = Vec::new();
        for _ in 0..60 {
            match r.gen_range(0..4) {
                0 | 1 => {
                    let v = r.gen_range(0..6);
                    ds.insert(v);
                    model.push(v);
                }
                2 => {
                    model.sort();
                    let ls = (model.len() + 1) / 2;
                    let exp = if ls > 0 { Some(model.remove(ls - 1)) } else { None };
                    assert_eq!(ds.pop_left_tail(), exp);
                }
                _ => {
                    model.sort();
                    let ls = (model.len() + 1) / 2;
                    let exp = if ls < model.len() { Some(model.remove(ls)) } else { None };
                    assert_eq!(ds.pop_right_head(), exp);
                }
            }
            model.sort();
            let ls = (model.len() + 1) / 2;
            assert_eq!(ds.left_size(), ls);
            assert_eq!(ds.right_size(), model.len() - ls);
            assert_eq!(ds.left_tail().copied(), if ls > 0 { Some(model[ls - 1]) } else { None });
            assert_eq!(ds.right_head().copied(), model.get(ls).copied());
        }
    }
}

// ---------------------------------------------------------------- maps & small things

#[test]
fn default_maps() {
    let mut r = rng(5);
    let mut hm: DefaultHashMap<i32, i64> = DefaultHashMap::new(7);
    let mut tm: DefaultTreeMap<i32, i64> = DefaultTreeMap::new(7);
    let mut model: BTreeMap<i32, i64> = BTreeMap::new();
    for _ in 0..5000 {
        let k = r.gen_range(-20..20);
        match r.gen_range(0..3) {
            0 => {
                let d = r.gen_range(-5..5);
                hm[k] += d;
                tm[k] += d;
                *model.entry(k).or_insert(7) += d;
            }
            1 => {
                assert_eq!(hm[k], model.get(&k).copied().unwrap_or(7));
                assert_eq!(tm[&k], model.get(&k).copied().unwrap_or(7));
                assert_eq!(*hm.get(&k), model.get(&k).copied().unwrap_or(7));
            }
            _ => {
                hm.remove(&k);
                tm.remove(&k);
                model.remove(&k);
            }
        }
        assert_eq!(hm.len(), model.len());
        assert_eq!(tm.len(), model.len());
    }
    assert_eq!(tm.into_iter().collect::<Vec<_>>(), model.into_iter().collect::<Vec<_>>());
    let arr = [3, 1, 3, 3, 2, 1];
    let q = qty(&arr);
    assert_eq!((q[3], q[1], q[2], q[9]), (3, 2, 1, 0));
    let bi = by_index(&arr);
    assert_eq!(bi[3], vec![0, 2, 3]);
    assert_eq!(bi[1], vec![1, 5]);
    assert!(bi[7].is_empty());
}

#[test]
fn array_map_test() {
    let mut a: ArrayMap<i32, i64> = ArrayMap::new(-5..=5);
    assert_eq!(a.len(), 11);
    for i in -5..=5 {
        a[i] = i as i64 * 10;
    }
    for i in -5..=5 {
        assert_eq!(a[i], i as i64 * 10);
    }
    let b: ArrayMap<i64, i64> = ArrayMap::with_gen(-3..4, |i| i * i);
    assert_eq!(b.len(), 7);
    assert_eq!(b[-3], 9);
    assert_eq!(b[3], 9);
    assert_eq!(b[0], 0);
    let c: ArrayMap<i32, i32> = ArrayMap::with_gen(7..7, |i| i);
    assert_eq!(c.len(), 0);
    let d: ArrayMap<usize, usize> = ArrayMap::with_gen(10..=12, |i| i);
    assert_eq!(&*d, &[10, 11, 12]);
    use std::ops::Bound;
    let e: ArrayMap<i32, i32> =
        ArrayMap::with_gen((Bound::Excluded(-2), Bound::Included(1)), |i| i);
    assert_eq!(&*e, &[-1, 0, 1]);
}

#[test]
fn fx_hash_smoke() {
    let mut r = rng(6);
    let mut m: FxHashMap<(u64, String, u8), usize> = FxHashMap::default();
    let mut model = BTreeMap::new();
    let mut s: FxHashSet<i128> = FxHashSet::default();
    let mut ms = BTreeSet::new();
    for i in 0..20000 {
        let k = (
            r.gen_range(0..50u64) << 40,
            format!("{}", r.gen_range(0..30)),
            r.gen_range(0..3u8),
        );
        m.insert(k.clone(), i);
        model.insert(k, i);
        let v = (r.gen_range(-100..100i128)) << 64;
        assert_eq!(s.insert(v), ms.insert(v));
    }
    assert_eq!(m.len(), model.len());
    for (k, v) in &model {
        assert_eq!(m[k], *v);
    }
}

#[test]
fn btree_ext_bounds_order_minmax_id() {
    let mut r = rng(7);
    for _ in 0..500 {
        let set: BTreeSet<i32> = (0..r.gen_range(0..10)).map(|_| r.gen_range(0..20)).collect();
        let map: BTreeMap<i32, i32> = set.iter().map(|&x| (x, x * 2)).collect();
        let v: Vec<i32> = set.iter().copied().collect();
        for x in -1..=21 {
            assert_eq!(set.next(&x).copied(), v.iter().copied().find(|&y| y > x));
            assert_eq!(set.ceil(&x).copied(), v.iter().copied().find(|&y| y >= x));
            assert_eq!(set.prev(&x).copied(), v.iter().rev().copied().find(|&y| y < x));
            assert_eq!(set.floor(&x).copied(), v.iter().rev().copied().find(|&y| y <= x));
            assert_eq!(map.next(&x).map(|p| *p.0), v.iter().copied().find(|&y| y > x));
            assert_eq!(map.ceil(&x).map(|p| *p.0), v.iter().copied().find(|&y| y >= x));
            assert_eq!(map.prev(&x).map(|p| *p.0), v.iter().rev().copied().find(|&y| y < x));
            assert_eq!(map.floor(&x).map(|p| *p.0), v.iter().rev().copied().find(|&y| y <= x));
        }
    }
    assert_eq!(clamp(&(2..5), 10), (2, 5));
    assert_eq!(clamp(&(2..=5), 10), (2, 6));
    assert_eq!(clamp(&(..), 10), (0, 10));
    assert_eq!(clamp(&(3..), 10), (3, 10));
    assert_eq!(clamp(&(3..50), 10), (3, 10));
    assert_eq!(clamp(&(0..0), 0), (0, 0));

    for _ in 0..500 {
        let v: Vec<i32> = (0..r.gen_range(0..12)).map(|_| r.gen_range(0..4)).collect();
        let o = v.order();
        let mut exp: Vec<usize> = (0..v.len()).collect();
        exp.sort_by_key(|&i| (v[i], i));
        assert_eq!(o, exp);
    }

    let mut x = 5;
    assert!(!x.minim(5));
    assert!(x.minim(4));
    assert!(!x.maxim(4));
    assert!(x.maxim(9));
    assert_eq!(x, 9);
    let mut o: Option<i32> = None;
    assert!(o.minim(3));
    assert!(!o.minim(3));
    assert!(o.maxim(4));
    assert_eq!(o, Some(4));

    let mut id = Id::new();
    assert_eq!(id.get("a"), 0);
    assert_eq!(id.get("b"), 1);
    assert_eq!(id.get("a"), 0);
    id.add(vec!["c", "a"]);
    id.add_pairs(vec![("d", "b")]);
    assert_eq!(id.len(), 4);
    assert_eq!(id.by_id(), vec!["a", "b", "c", "d"]);
}

// ---------------------------------------------------------------- FastClearArr

// Candidate: plain assignment + clear vs Vec model
#[test]
fn cand_fast_clear_arr_stress() {
    let mut r = rng(8);
    for it in 0..2000 {
        let n = r.gen_range(1..8);
        let mut a: FastClearArr<i32> = FastClearArr::with_default(-1);
        let mut model = vec![-1; n];
        for step in 0..40 {
            match r.gen_range(0..5) {
                0 | 1 => {
                    let i = r.gen_range(0..n);
                    let v = r.gen_range(0..100);
                    a[i] = v;
                    model[i] = v;
                }
                2 => {
                    if r.gen_bool(0.3) {
                        a.clear();
                        model.fill(-1);
                    }
                }
                _ => {}
            }
            for i in 0..n {
                assert_eq!(a[i], model[i], "iter {it} step {step} index {i}");
            }
        }
    }
}

// Candidate minimal 1: writing a stale low index truncates live higher indices
#[test]
fn cand_fast_clear_arr_truncates() {
    let mut a: FastClearArr<i32> = FastClearArr::with_default(0);
    a[0] = 1;
    a[1] = 2;
    a.clear();
    a[1] = 3;
    a[0] = 4;
    assert_eq!(a[1], 3);
}

// Candidate minimal 2: read-modify-write through IndexMut sees the stale value
#[test]
fn cand_fast_clear_arr_add_assign_after_clear() {
    let mut a: FastClearArr<i32> = FastClearArr::with_default(0);
    a[0] += 5;
    assert_eq!(a[0], 5);
    a.clear();
    assert_eq!(a[0], 0);
    a[0] += 1;
    assert_eq!(a[0], 1);
}

// Verification of the suggested fix on a copy.
mod fixed_fca {
    use std::ops::{Index, IndexMut};
    pub struct Fca<T> {
        pub arr: Vec<(T, u32)>,
        pub epoch: u32,
        pub default: T,
    }
    impl<T> Index<usize> for Fca<T> {
        type Output = T;
        fn index(&self, index: usize) -> &T {
            if index >= self.arr.len() || self.arr[index].1 != self.epoch {
                &self.default
            } else {
                &self.arr[index].0
            }
        }
    }
    impl<T: Clone> IndexMut<usize> for Fca<T> {
        fn index_mut(&mut self, index: usize) -> &mut T {
            if index >= self.arr.len() {
                let (d, e) = (self.default.clone(), self.epoch);
                self.arr.resize_with(index + 1, || (d.clone(), e));
            } else if self.arr[index].1 != self.epoch {
                self.arr[index] = (self.default.clone(), self.epoch);
            }
            &mut self.arr[index].0
        }
    }
}

#[test]
fn fixed_fast_clear_arr_stress() {
    let mut r = rng(8);
    for _ in 0..2000 {
        let n = r.gen_range(1..8);
        let mut a = fixed_fca::Fca { arr: Vec::new(), epoch: 0, default: -1 };
        let mut model = vec![-1; n];
        for _ in 0..40 {
            match r.gen_range(0..5) {
                0 => {
                    let i = r.gen_range(0..n);
                    let v = r.gen_range(0..100);
                    a[i] = v;
                    model[i] = v;
                }
                1 => {
                    let i = r.gen_range(0..n);
                    a[i] += 3;
                    model[i] += 3;
                }
                2 => {
                    if r.gen_bool(0.3) {
                        a.epoch += 1;
                        model.fill(-1);
                    }
                }
                _ => {}
            }
            for i in 0..n {
                assert_eq!(a[i], model[i]);
            }
        }
    }
}

// ---------------------------------------------------------------- md_arr

#[test]
fn arr2d_ops() {
    let mut r = rng(9);
    for _ in 0..500 {
        let n = r.gen_range(0..6);
        let m = r.gen_range(0..6);
        let a = Arr2d::with_gen(n, m, |i, j| i * 10 + j);
        assert_eq!((a.d1(), a.d2()), (n, m));
        for i in 0..n {
            let exp: Vec<usize> = (0..m).map(|j| i * 10 + j).collect();
            assert_eq!(a.row(i).copied().collect::<Vec<_>>(), exp);
            assert_eq!(a[i].to_vec(), exp);
        }
        for j in 0..m {
            let exp: Vec<usize> = (0..n).map(|i| i * 10 + j).collect();
            assert_eq!(a.col(j).copied().collect::<Vec<_>>(), exp);
        }
        let t = a.clone().transpose();
        assert_eq!((t.d1(), t.d2()), (m, n));
        let cw = a.clone().rotate_clockwise();
        let ccw = a.clone().rotate_counterclockwise();
        assert_eq!((cw.d1(), cw.d2()), (m, n));
        assert_eq!((ccw.d1(), ccw.d2()), (m, n));
        for i in 0..n {
            for j in 0..m {
                assert_eq!(t[(j, i)], a[(i, j)]);
                assert_eq!(cw[(j, n - 1 - i)], a[(i, j)]);
                assert_eq!(ccw[(m - 1 - j, i)], a[(i, j)]);
            }
        }
        assert_eq!(a.clone().rotate_clockwise().rotate_counterclockwise(), a);
        assert_eq!(
            a.indices().collect::<Vec<_>>(),
            (0..n).flat_map(|i| (0..m).map(move |j| (i, j))).collect::<Vec<_>>()
        );
        if n > 0 && m > 0 {
            let (i, j) = (r.gen_range(0..n), r.gen_range(0..m));
            assert_eq!(a.find(&(i * 10 + j)), Some((i, j)));
            assert_eq!(a.find(&999), None);
            let (r1, r2) = (r.gen_range(0..n), r.gen_range(0..n));
            let mut b = a.clone();
            b.swap_rows(r1, r2);
            for i in 0..n {
                let src = if i == r1 {
                    r2
                } else if i == r2 {
                    r1
                } else {
                    i
                };
                assert_eq!(b[i], a[src]);
            }
            let mut c = a.clone();
            let (i2, j2) = (r.gen_range(0..n), r.gen_range(0..m));
            c.swap(i, j, i2, j2);
            assert_eq!(c[(i, j)], a[(i2, j2)]);
            assert_eq!(c[(i2, j2)], a[(i, j)]);
            let mut d = a.clone();
            for x in d.col_mut(j) {
                *x = 0;
            }
            for x in d.row_mut(i) {
                *x = 1;
            }
            for ii in 0..n {
                for jj in 0..m {
                    let e = if ii == i {
                        1
                    } else if jj == j {
                        0
                    } else {
                        a[(ii, jj)]
                    };
                    assert_eq!(d[(ii, jj)], e);
                }
            }
        }
    }
}

#[test]
fn arr345d_indexing() {
    let a = Arr3d::with_gen(2, 3, 4, |i, j, k| (i, j, k));
    let b = Arr4d::with_gen(2, 3, 4, 2, |i, j, k, l| (i, j, k, l));
    let c = Arr5d::with_gen(2, 3, 1, 2, 3, |i, j, k, l, m| (i, j, k, l, m));
    let mut c2 = Arr5d::new(2, 3, 1, 2, 3, (0, 0, 0, 0, 0));
    let mut b2 = Arr4d::new(2, 3, 4, 2, (0, 0, 0, 0));
    let mut a2 = Arr3d::new(2, 3, 4, (0, 0, 0));
    for i in 0..2 {
        for j in 0..3 {
            for k in 0..4 {
                assert_eq!(a[(i, j, k)], (i, j, k));
                a2[(i, j, k)] = (i, j, k);
                for l in 0..2 {
                    assert_eq!(b[(i, j, k, l)], (i, j, k, l));
                    b2[(i, j, k, l)] = (i, j, k, l);
                }
            }
            for l in 0..2 {
                for m in 0..3 {
                    assert_eq!(c[(i, j, 0, l, m)], (i, j, 0, l, m));
                    c2[(i, j, 0, l, m)] = (i, j, 0, l, m);
                }
            }
        }
    }
    assert!(a == a2 && b == b2 && c == c2);
}

// ---------------------------------------------------------------- slice_ext

#[test]
fn slice_bounds() {
    let mut r = rng(10);
    for _ in 0..5000 {
        let n = r.gen_range(0..12);
        let hi = [1, 3, 8][r.gen_range(0..3)];
        let mut v: Vec<i32> = (0..n).map(|_| r.gen_range(0..hi)).collect();
        v.sort();
        for x in -1..=hi {
            let lb = v.iter().filter(|&&y| y < x).count();
            let ub = v.iter().filter(|&&y| y <= x).count();
            assert_eq!(v.lower_bound(&x), lb);
            assert_eq!(v.upper_bound(&x), ub);
            assert_eq!(v.less(&x), lb);
            assert_eq!(v.less_or_eq(&x), ub);
            assert_eq!(v.more(&x), n - ub);
            assert_eq!(v.more_or_eq(&x), n - lb);
            assert_eq!(v.bin_search(&x), v.iter().position(|&y| y == x));
            for y in x..=hi {
                assert_eq!(v.inside(&x..&y), v.iter().filter(|&&z| x <= z && z < y).count());
                assert_eq!(v.inside(&x..=&y), v.iter().filter(|&&z| x <= z && z <= y).count());
            }
            assert_eq!(v.inside(&x..), n - lb);
            assert_eq!(v.inside(..&x), lb);
            assert_eq!(v.inside(..), n);
        }
    }
}

fn all_perms(v: &mut Vec<i32>, k: usize, out: &mut BTreeSet<Vec<i32>>) {
    if k == v.len() {
        out.insert(v.clone());
        return;
    }
    for i in k..v.len() {
        v.swap(k, i);
        all_perms(v, k + 1, out);
        v.swap(k, i);
    }
}

#[test]
fn slice_permutations_compress_misc() {
    let mut r = rng(11);
    for _ in 0..300 {
        let n = r.gen_range(0..7);
        let mut v: Vec<i32> = (0..n).map(|_| r.gen_range(0..4)).collect();
        v.sort();
        let mut exp = BTreeSet::new();
        all_perms(&mut v.clone(), 0, &mut exp);
        let mut got = vec![v.clone()];
        while v.next_permutation() {
            got.push(v.clone());
        }
        assert_eq!(got, exp.into_iter().collect::<Vec<_>>());
    }
    for _ in 0..1000 {
        let n = r.gen_range(0..10);
        let mut p: Vec<usize> = (0..n).collect();
        p.shuffle(&mut r);
        let mut q: Vec<usize> = (0..n).collect();
        q.shuffle(&mut r);
        let inv = p.inv();
        assert_eq!(unsafe { p.unsafe_inv() }, inv);
        for i in 0..n {
            assert_eq!(inv[p[i]], i);
            assert_eq!(p.mul(&q)[i], p[q[i]]);
        }
        assert_eq!(p.mul(&inv), (0..n).collect::<Vec<_>>());

        let a: Vec<i64> = (0..r.gen_range(0..8)).map(|_| r.gen_range(-3..3) * 1_000_000_000_000).collect();
        let b: Vec<i64> = (0..r.gen_range(0..8)).map(|_| r.gen_range(-3..3) * 1_000_000_000_000).collect();
        let c = compress([&a, &b]);
        let mut all: Vec<i64> = a.iter().chain(b.iter()).copied().collect();
        all.sort();
        all.dedup();
        assert_eq!(c.order, all);
        assert_eq!(c.arrs[0].iter().map(|&i| all[i]).collect::<Vec<_>>(), a);
        assert_eq!(c.arrs[1].iter().map(|&i| all[i]).collect::<Vec<_>>(), b);

        let u: Vec<usize> = (0..r.gen_range(0..10)).map(|_| r.gen_range(0..5)).collect();
        let q = u.qty();
        assert_eq!(q.len(), u.iter().max().map_or(0, |m| m + 1));
        for (x, &c) in q.iter().enumerate() {
            assert_eq!(c, u.iter().filter(|&&y| y == x).count());
        }
        assert_eq!(u.qty_bound(7).len(), 7);
        assert_eq!(u.indices(), 0..u.len());
        assert_eq!(
            u.consecutive_iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>(),
            u.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>()
        );
        assert_eq!(
            u.consecutive_iter_copy().collect::<Vec<_>>(),
            u.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>()
        );
    }
    let mut v = vec![1, 2, 3, 4];
    assert_eq!(v[Back(0)], 4);
    assert_eq!(v[Back(3)], 1);
    v[Back(1)] = 9;
    assert_eq!(v, vec![1, 2, 9, 4]);
    for i in 0..4 {
        for j in 0..4 {
            if i != j {
                let (a, b) = v.two_mut(i, j);
                assert_eq!((*a, *b), ([1, 2, 9, 4][i], [1, 2, 9, 4][j]));
            }
            for k in 0..4 {
                if i != j && i != k {
                    let (a, b, c) = v.three_mut(i, j, k);
                    assert_eq!((*a, *b, *c), ([1, 2, 9, 4][i], [1, 2, 9, 4][j], [1, 2, 9, 4][k]));
                }
            }
        }
    }
}

#[test]
fn radix_sort_stress() {
    let mut r = rng(12);
    for it in 0..400 {
        let n = if it % 4 == 0 { r.gen_range(0..70) } else { r.gen_range(60..600) };
        let mode = r.gen_range(0..5);
        let mut data: Vec<(u64, usize)> = (0..n)
            .map(|i| {
                let k = match mode {
                    0 => r.gen::<u64>(),
                    1 => r.gen_range(0..4),
                    2 => u64::MAX - r.gen_range(0..3),
                    3 => r.gen_range(0..3) << 56,
                    _ => 0,
                };
                (k, i)
            })
            .collect();
        let mut exp = data.clone();
        exp.sort_by_key(|x| x.0);
        radix_sort_by_key(&mut data, |x| x.0);
        assert_eq!(data, exp);
    }
}

// ---------------------------------------------------------------- iter_ext / vec_ext

#[test]
fn iter_vec_ext() {
    for n in 0..6usize {
        assert_eq!(cur_next(n).collect::<Vec<_>>(), (0..n).map(|i| (i, (i + 1) % n)).collect::<Vec<_>>());
        if n > 0 {
            assert_eq!(
                prev_cur_next(n).collect::<Vec<_>>(),
                (0..n).map(|i| ((i + n - 1) % n, i, (i + 1) % n)).collect::<Vec<_>>()
            );
        }
    }
    assert_eq!(interleave(vec![1, 3, 5].into_iter(), vec![2, 4, 6].into_iter()).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(interleave(vec![1, 3, 5].into_iter(), vec![2, 4].into_iter()).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    let v = vec![3, 7, 1, 7, 1];
    assert_eq!(v.max_position(), 1);
    assert_eq!(v.min_position(), 2);
    assert_eq!(v.copy_max(), 7);
    assert_eq!(v.copy_min(), 1);
    assert_eq!(v.copy_sum(), 19);
    assert_eq!(v.copy_find(7), Some(1));
    assert_eq!(v.copy_count(1), 2);
    assert_eq!(v.copy_rev().collect::<Vec<_>>(), vec![1, 7, 1, 7, 3]);
    assert_eq!(v.iter().iter_count(&7), 2);
    assert_eq!(v.clone().iter_find(1), Some(2));
    assert_eq!(v.clone().iter_sum(), 19);
    let z: Vec<(i32, i32, i32)> = zip!(vec![1, 2], vec![3, 4], vec![5, 6]).collect();
    assert_eq!(z, vec![(1, 3, 5), (2, 4, 6)]);

    let pre = Vec::with_gen_prefix(6, |i, p: &Vec<usize>| if i == 0 { 0 } else { p[i - 1] + i });
    assert_eq!(pre, vec![0, 1, 3, 6, 10, 15]);
    let suf = Vec::with_gen_suffix(5, |i, s: &Vec<usize>| if i == 4 { 1 } else { s[i + 1] * 2 });
    assert_eq!(suf, vec![16, 8, 4, 2, 1]);
    let mut g = vec![5usize];
    g.gen_append(3, |i, s| i + s.len());
    assert_eq!(g, vec![5, 2, 4, 6]);
    assert_eq!(Vec::with_gen(0, |i| i), Vec::<usize>::new());
    assert_eq!(default_vec::<i32>(3), vec![0, 0, 0]);
    assert_eq!(vec![(1usize, 2usize, -5i64)].dec(), vec![(0, 1, -5)]);
    assert_eq!(vec![(1usize, 2usize)].inc(), vec![(2, 3)]);
    assert_eq!(vec![1i32, 2].dec(), vec![0, 1]);
    assert_eq!(vec![(1, 'a'), (2, 'b')].detuple(), (vec![1, 2], vec!['a', 'b']));
    assert_eq!(vec![(1, 'a', 2.0)].detuple(), (vec![1], vec!['a'], vec![2.0]));
    assert_eq!(vec![3, 1, 2].sorted(), vec![1, 2, 3]);
    assert_eq!(vec![3, 1, 2].reversed(), vec![2, 1, 3]);
    assert_eq!(vec![3, 1, 2].sorted_by_key(|x| -*x), vec![3, 2, 1]);
    assert_eq!(vec![(1, 'a')].transpose_pair_vec(), vec![('a', 1)]);
}

// ---------------------------------------------------------------- misc: bin_search, mo

#[test]
fn bin_search_int() {
    let mut r = rng(13);
    for _ in 0..20000 {
        let left: i64 = r.gen_range(-50..50);
        let right: i64 = r.gen_range(left..=left + 60);
        let th = r.gen_range(left..=right);
        assert_eq!(first_true(left, right, |x| x >= th), th);
        assert_eq!(last_false(left, right, |x| x > th), th);
        let (l, rr, t) = ((left + 50) as usize, (right + 50) as usize, (th + 50) as usize);
        assert_eq!(first_true(l, rr, |x| x >= t), t);
        assert_eq!(last_false(l, rr, |x| x > t), t);
    }
    assert_eq!(first_true(i64::MIN / 2, i64::MAX / 2, |x| x >= 12345), 12345);
    assert_eq!(last_false(i64::MIN / 2 + 1, i64::MAX / 2, |x| x > -12345), -12345);
    assert_eq!(first_true(0u64, u64::MAX, |x| x >= 1 << 63), 1 << 63);
    let res = bin_search_real(Real(0.0), Real(10.0), |x| x.0 * x.0 >= 2.0);
    assert!((res.0 - 2f64.sqrt()).abs() < 1e-12);
}

struct Window {
    window: std::collections::VecDeque<u32>,
}

impl MoWorker for Window {
    type T = u32;
    type R = Vec<u32>;
    fn empty() -> Self {
        Self { window: Default::default() }
    }
    fn add_left(&mut self, val: &u32) {
        self.window.push_front(*val);
    }
    fn add_right(&mut self, val: &u32) {
        self.window.push_back(*val);
    }
    fn remove_left(&mut self, val: &u32) {
        assert_eq!(self.window.pop_front(), Some(*val));
    }
    fn result(&self) -> Vec<u32> {
        self.window.iter().copied().collect()
    }
}

#[test]
fn mo_stress() {
    let mut r = rng(14);
    for _ in 0..3000 {
        let n = r.gen_range(1..30);
        let arr: Vec<u32> = (0..n).map(|_| r.gen()).collect();
        let q = [1, 2, 5, 40, 400][r.gen_range(0..5)];
        let q = r.gen_range(1..=q);
        let queries: Vec<(usize, usize)> = (0..q)
            .map(|_| {
                let a = r.gen_range(0..n);
                let b = r.gen_range(a..n);
                (a, b)
            })
            .collect();
        let ans = mo::<Window>(&arr, &queries);
        for (&(a, b), res) in queries.iter().zip(ans.iter()) {
            assert_eq!(res.as_slice(), &arr[a..=b]);
        }
    }
}

// ---------------------------------------------------------------- expression parser

struct Gen<'a> {
    r: &'a mut ChaCha8Rng,
    spaces: bool,
    out: Vec<u8>,
}

impl Gen<'_> {
    fn sp(&mut self) {
        if self.spaces && self.r.gen_bool(0.5) {
            self.out.push(b' ');
        }
    }
    // priority 0: + -, priority 1: *
    fn expr(&mut self, prio: usize, depth: usize) -> i64 {
        if prio == 2 {
            return self.unary(depth);
        }
        let mut res = self.expr(prio + 1, depth);
        while depth > 0 && self.r.gen_bool(0.35) {
            let op = if prio == 0 { [b'+', b'-'][self.r.gen_range(0..2)] } else { b'*' };
            self.sp();
            self.out.push(op);
            self.sp();
            let other = self.expr(prio + 1, depth - 1);
            res = match op {
                b'+' => res.wrapping_add(other),
                b'-' => res.wrapping_sub(other),
                _ => res.wrapping_mul(other),
            };
        }
        res
    }
    fn unary(&mut self, depth: usize) -> i64 {
        match if depth == 0 { 9 } else { self.r.gen_range(0..10) } {
            0 => {
                self.out.push(b'-');
                self.unary(depth - 1).wrapping_neg()
            }
            1 | 2 | 3 => {
                self.out.push(b'(');
                self.sp();
                let res = self.expr(0, depth - 1);
                self.sp();
                self.out.push(b')');
                res
            }
            _ => {
                let v: i64 = self.r.gen_range(0..20);
                self.out.extend_from_slice(v.to_string().as_bytes());
                v
            }
        }
    }
}

fn make_parser() -> ExpressionParser<i64, impl Fn(&[u8]) -> i64> {
    let mut p = ExpressionParser::new(|s: &[u8]| std::str::from_utf8(s).unwrap().parse::<i64>().unwrap());
    p.add_binary_op(0, b'+', i64::wrapping_add);
    p.add_binary_op(0, b'-', i64::wrapping_sub);
    p.add_binary_op(1, b'*', i64::wrapping_mul);
    p.add_unary_op(b'-', i64::wrapping_neg);
    p
}

#[test]
fn expression_parser_no_spaces() {
    let mut r = rng(15);
    let p = make_parser();
    for _ in 0..20000 {
        let mut g = Gen { r: &mut r, spaces: false, out: Vec::new() };
        let exp = g.expr(0, 4);
        let s = g.out;
        assert_eq!(p.parse(&s), exp, "{}", String::from_utf8_lossy(&s));
    }
}

// Candidate: whitespace after ')' silently truncates the expression
#[test]
fn cand_expression_parser_spaces() {
    let mut r = rng(16);
    let p = make_parser();
    for _ in 0..20000 {
        let mut g = Gen { r: &mut r, spaces: true, out: Vec::new() };
        let exp = g.expr(0, 3);
        let s = g.out;
        assert_eq!(p.parse(&s), exp, "{}", String::from_utf8_lossy(&s));
    }
}

#[test]
fn cand_expression_parser_space_after_paren_min() {
    let p = make_parser();
    assert_eq!(p.parse(b"1 + 2"), 3);
    assert_eq!(p.parse(b"(1)+2"), 3);
    assert_eq!(p.parse(b"(1) +2"), 3);
}

// ---------------------------------------------------------------- random

#[test]
fn random_ranges_ok() {
    let mut rnd = Random::new_with_seed(17);
    let mut seen = [0usize; 7];
    for _ in 0..70000 {
        let x: i32 = rnd.gen_range(-3..4);
        assert!((-3..4).contains(&x));
        seen[(x + 3) as usize] += 1;
        let y: i64 = rnd.gen_range(-10..=-5);
        assert!((-10..=-5).contains(&y));
        let z: usize = rnd.gen_range(5..6);
        assert_eq!(z, 5);
        let w: u8 = rnd.gen_range(250..=255);
        assert!(w >= 250);
        let _: u64 = rnd.gen_range(..);
        let _: i64 = rnd.gen_range(..);
        let a: u64 = rnd.gen_range(1..);
        assert!(a >= 1);
        let b: u32 = rnd.gen_range(..=7);
        assert!(b <= 7);
        let c: i64 = rnd.gen_range(-4_000_000_000_000_000_000..=4_000_000_000_000_000_000);
        assert!(c.abs() <= 4_000_000_000_000_000_000);
        let d: usize = rnd.gen_bound(3usize);
        assert!(d < 3);
    }
    for c in seen {
        assert!((9000..11000).contains(&c), "{seen:?}");
    }
    // both extremes reachable on inclusive range
    let mut lo = false;
    let mut hi = false;
    for _ in 0..1000 {
        let x: i8 = rnd.gen_range(-128..=127);
        lo |= x < -100;
        hi |= x > 100;
        let y: i32 = rnd.gen_range(0..=1);
        assert!(y == 0 || y == 1);
    }
    assert!(lo && hi);
    // shuffle uniformity, n = 3
    let mut cnt: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
    for _ in 0..60000 {
        let mut v = vec![0u8, 1, 2];
        algo_lib::misc::random::Shuffle::shuffle_with(v.as_mut_slice(), &mut rnd);
        *cnt.entry(v).or_insert(0) += 1;
    }
    assert_eq!(cnt.len(), 6);
    for (_, c) in cnt {
        assert!((9300..10700).contains(&c));
    }
    let mut e: Vec<u8> = vec![];
    algo_lib::misc::random::Shuffle::shuffle_with(e.as_mut_slice(), &mut rnd);
    assert_eq!(*algo_lib::misc::random::Shuffle::choice_with(&[5][..], &mut rnd), 5);
}

// Candidate: signed range spanning >= half of the type
#[test]
fn cand_gen_range_wide_i32() {
    let mut rnd = Random::new_with_seed(18);
    for _ in 0..1000 {
        let x: i32 = rnd.gen_range(-2_000_000_000..=2_000_000_000);
        assert!((-2_000_000_000..=2_000_000_000).contains(&x));
    }
}

#[test]
fn cand_gen_range_half_open_signed() {
    let mut rnd = Random::new_with_seed(19);
    for _ in 0..1000 {
        let x: i32 = rnd.gen_range(0..);
        assert!(x >= 0);
    }
}

#[test]
fn cand_gen_range_negative_half_signed() {
    let mut rnd = Random::new_with_seed(19);
    for _ in 0..1000 {
        let x: i64 = rnd.gen_range(..0);
        assert!(x < 0);
    }
}

// Candidate: 128-bit bounds are truncated to u64
#[test]
fn cand_gen_bound_u128() {
    let mut rnd = Random::new_with_seed(20);
    let n: u128 = (1u128 << 64) + 5;
    let mut big = false;
    for _ in 0..1000 {
        let x = rnd.gen_bound(n);
        assert!(x < n);
        big |= x >= 5;
    }
    assert!(big, "gen_bound((1<<64)+5) only ever returned values < 5");
}

#[test]
fn cand_gen_bound_u128_pow2() {
    let mut rnd = Random::new_with_seed(20);
    let x = rnd.gen_bound(1u128 << 64);
    assert!(x < 1u128 << 64);
}

// ---------------------------------------------------------------- dirs, memo, misc small

#[test]
fn dirs_test() {
    for n in 1..5usize {
        for m in 1..5usize {
            for i in 0..n {
                for j in 0..m {
                    for (deltas, got) in [
                        (vec![(0, 1), (1, 0), (0, -1), (-1, 0)], D4::iter(i, j, n, m).collect::<Vec<_>>()),
                        (
                            vec![(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)],
                            D8::iter(i, j, n, m).collect::<Vec<_>>(),
                        ),
                        (
                            vec![(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)],
                            DK::iter(i, j, n, m).collect::<Vec<_>>(),
                        ),
                    ] {
                        let exp: Vec<(usize, usize)> = deltas
                            .iter()
                            .map(|&(a, b): &(isize, isize)| (i as isize + a, j as isize + b))
                            .filter(|&(a, b)| a >= 0 && b >= 0 && (a as usize) < n && (b as usize) < m)
                            .map(|(a, b)| (a as usize, b as usize))
                            .collect();
                        assert_eq!(got, exp);
                    }
                    for d in 0..4 {
                        let (a, b) = D4::go(i, j, d, n, m);
                        assert!(a < n && b < m);
                    }
                }
            }
            let mut b = border(n, m);
            let len = b.len();
            b.sort();
            b.dedup();
            assert_eq!(b.len(), len, "duplicates in border {n} {m}");
            let exp: Vec<(usize, usize)> = (0..n)
                .flat_map(|i| (0..m).map(move |j| (i, j)))
                .filter(|&(i, j)| i == 0 || j == 0 || i == n - 1 || j == m - 1)
                .collect();
            assert_eq!(b, exp);
        }
    }
}

#[test]
fn memo_and_recursive() {
    let mut fib = Memoization::new(|f, n: u64| -> u64 { if n < 2 { n } else { f.call(n - 1).wrapping_add(f.call(n - 2)) } });
    assert_eq!(fib.call(90), 2880067194370816120);
    let mut binom = Memoization2::new(|f, n: usize, k: usize| -> u64 {
        if k == 0 || k == n {
            1
        } else {
            f.call(n - 1, k - 1) + f.call(n - 1, k)
        }
    });
    assert_eq!(binom.call(40, 20), 137846528820);
    let mut m1 = Memoization1d::new(91, |f, n| -> u64 { if n < 2 { n as u64 } else { f.call(n - 1) + f.call(n - 2) } });
    assert_eq!(m1.call(90), 2880067194370816120);
    let mut m2 = Memoization2d::new(41, 41, |f, n, k| -> u64 {
        if k == 0 || k == n {
            1
        } else {
            f.call(n - 1, k - 1) + f.call(n - 1, k)
        }
    });
    assert_eq!(m2.call(40, 20), 137846528820);
    // number of lattice paths in 3d/4d/5d boxes, non-cubic dims to catch index mixups
    let mut m3 = Memoization3d::new(3, 4, 5, |f, a, b, c| -> u64 {
        if a + b + c == 0 {
            return 1;
        }
        let mut s = 0;
        if a > 0 {
            s += f.call(a - 1, b, c);
        }
        if b > 0 {
            s += f.call(a, b - 1, c);
        }
        if c > 0 {
            s += f.call(a, b, c - 1);
        }
        s
    });
    assert_eq!(m3.call(2, 3, 4), 1260); // 9!/(2!3!4!)
    let mut m4 = Memoization4d::new(2, 3, 4, 3, |f, a, b, c, d| -> u64 {
        if a + b + c + d == 0 {
            return 1;
        }
        let mut s = 0;
        if a > 0 {
            s += f.call(a - 1, b, c, d);
        }
        if b > 0 {
            s += f.call(a, b - 1, c, d);
        }
        if c > 0 {
            s += f.call(a, b, c - 1, d);
        }
        if d > 0 {
            s += f.call(a, b, c, d - 1);
        }
        s
    });
    assert_eq!(m4.call(1, 2, 3, 2), 1680); // 8!/(1!2!3!2!)
    let mut m5 = Memoization5d::new(2, 3, 2, 3, 2, |f, a, b, c, d, e| -> u64 {
        if a + b + c + d + e == 0 {
            return 1;
        }
        let mut s = 0;
        if a > 0 {
            s += f.call(a - 1, b, c, d, e);
        }
        if b > 0 {
            s += f.call(a, b - 1, c, d, e);
        }
        if c > 0 {
            s += f.call(a, b, c - 1, d, e);
        }
        if d > 0 {
            s += f.call(a, b, c, d - 1, e);
        }
        if e > 0 {
            s += f.call(a, b, c, d, e - 1);
        }
        s
    });
    assert_eq!(m5.call(1, 2, 1, 2, 1), 1260); // 7!/(1!2!1!2!1!)

    let mut sum = 0u64;
    let mut rf = RecursiveFunction::new(|f, n: u64| {
        sum += n;
        if n > 0 {
            f.call(n - 1);
        }
    });
    rf.call(1000);
    assert_eq!(sum, 500500);
    let mut gcd = RecursiveFunction2::new(|f, a: u64, b: u64| if b == 0 { a } else { f.call(b, a % b) });
    assert_eq!(gcd.call(84, 36), 12);
}

static LAZY: LazyLock<Vec<u64>> = LazyLock::new(|| (0..10).collect());

#[test]
fn small_misc() {
    assert_eq!(LAZY.len(), 10);
    assert_eq!(LAZY[9], 9);
    let c = OwnedCell::new(5);
    unsafe {
        *c.as_mut() += 1;
        assert_eq!(*c.as_ref(), 6);
        assert_eq!(c.replace(9), 6);
        assert_eq!(*c.as_ref(), 9);
    }
    let mut v = vec![1, 2, 3];
    v.replace_with(|mut x| {
        x.push(4);
        x
    });
    assert_eq!(v, vec![1, 2, 3, 4]);
    assert_eq!(vec![1].do_with(|x| x.push(2)), vec![1, 2]);
    assert_eq!(algo_lib::misc::extensions::with::With::with(5, |x| x + 1), 6);
    assert_eq!(5.take_if(true), Some(5));
    assert_eq!(5.take_if(false), None);
    assert_eq!(algo_lib::misc::between::between(5, 2), 2..=5);
    let x = 7;
    let s = algo_lib::when! {
        x < 5 => "small",
        x < 10 => "mid",
        else => "big",
    };
    assert_eq!(s, "mid");

    // bump allocator: alignment + no overlap
    use std::alloc::Layout;
    let mut r = rng(21);
    let mut blocks: Vec<(usize, usize, u8)> = Vec::new();
    for i in 0..3000 {
        let align = 1usize << r.gen_range(0..7);
        let size = if i % 500 == 499 { (1 << 22) + r.gen_range(0..100) } else { r.gen_range(1..5000) };
        let p = bump_alloc(Layout::from_size_align(size, align).unwrap());
        let addr = p.as_ptr() as usize;
        assert_eq!(addr % align, 0);
        let tag = (i % 251) as u8;
        unsafe { std::ptr::write_bytes(p.as_ptr(), tag, size) };
        blocks.push((addr, size, tag));
    }
    for &(addr, size, tag) in &blocks {
        let s = unsafe { std::slice::from_raw_parts(addr as *const u8, size) };
        assert!(s.iter().all(|&b| b == tag));
    }
    let p = bump_new(12345u64);
    assert_eq!(unsafe { *p.as_ptr() }, 12345);

    if is_x86_feature_detected!("avx2") {
        let mut a = vec![1u32, 2, 3];
        algo_lib::misc::simd::fast_apply(&mut a, |x| x * 2);
        assert_eq!(a, vec![2, 4, 6]);
        assert_eq!(algo_lib::misc::simd::fast_fold(&a, 0u32, |s, x| s + x), 12);
    }
    let mut tt = algo_lib::misc::time_tracker::TimeTracker::new();
    tt.disable();
    tt.milestone("x");
    let _ = tt.silent_milestone();
}

algo_lib::value!(ValSeven: i32 = 7);
algo_lib::dynamic_value!(DynVal: i64);
algo_lib::value_ref!(RefVal: Vec<i32>);

#[test]
fn value_macros() {
    use algo_lib::misc::value::{DynamicValue, Value};
    use algo_lib::misc::value_ref::ValueRef;
    assert_eq!(ValSeven::val(), 7);
    DynVal::set(11);
    assert_eq!(DynVal::val(), 11);
    DynVal::set(12);
    assert_eq!(DynVal::val(), 12);
    assert!(!RefVal::is_init());
    RefVal::set(vec![1]);
    assert!(RefVal::is_init());
    RefVal::with_mut(|v| v.push(2));
    assert_eq!(RefVal::with(|v| v.clone()), vec![1, 2]);
}

// ---------------------------------------------------------------- fixed copies

mod fixed_parser {
    use algo_lib::string::trim::StrTrim;
    use std::cell::Cell;

    pub struct P {
        pub max_priority: usize,
        pub binary_ops: Vec<(usize, u8, Box<dyn Fn(i64, i64) -> i64>)>,
        pub unary_ops: Vec<(u8, Box<dyn Fn(i64) -> i64>)>,
        pub binary_id: Vec<Option<usize>>,
        pub unary_id: Vec<Option<usize>>,
        pub pos: Cell<usize>,
    }

    impl P {
        pub fn parse(&self, s: &[u8]) -> i64 {
            self.pos.set(0);
            self.parse_expr(0, s)
        }
        fn parse_expr(&self, priority: usize, s: &[u8]) -> i64 {
            if priority == self.max_priority {
                self.parse_unary(s)
            } else {
                let mut res = self.parse_expr(priority + 1, s);
                loop {
                    // FIX: skip blanks before looking for an operator / end / ')'
                    while self.pos.get() < s.len() && s[self.pos.get()] == b' ' {
                        self.pos.set(self.pos.get() + 1);
                    }
                    if self.pos.get() == s.len() {
                        return res;
                    }
                    let mut found = false;
                    if let Some(id) = self.binary_id[s[self.pos.get()] as usize] {
                        if self.binary_ops[id].0 == priority {
                            self.pos.set(self.pos.get() + 1);
                            let other = self.parse_expr(priority + 1, s);
                            res = (self.binary_ops[id].2)(res, other);
                            found = true;
                        }
                    }
                    if !found {
                        break;
                    }
                }
                res
            }
        }
        fn parse_unary(&self, s: &[u8]) -> i64 {
            while s[self.pos.get()] == b' ' {
                self.pos.set(self.pos.get() + 1);
            }
            if let Some(id) = self.unary_id[s[self.pos.get()] as usize] {
                self.pos.set(self.pos.get() + 1);
                let other = self.parse_unary(s);
                return (self.unary_ops[id].1)(other);
            }
            if s[self.pos.get()] == b'(' {
                self.pos.set(self.pos.get() + 1);
                let res = self.parse_expr(0, s);
                self.pos.set(self.pos.get() + 1);
                return res;
            }
            let start = self.pos.get();
            loop {
                if self.pos.get() == s.len()
                    || s[self.pos.get()] == b')'
                    || self.binary_id[s[self.pos.get()] as usize].is_some()
                {
                    break;
                }
                self.pos.set(self.pos.get() + 1);
            }
            let s = s[start..self.pos.get()].trim();
            std::str::from_utf8(s).unwrap().parse().unwrap()
        }
    }
}

#[test]
fn fixed_expression_parser_spaces() {
    let mut p = fixed_parser::P {
        max_priority: 2,
        binary_ops: vec![
            (0, b'+', Box::new(i64::wrapping_add)),
            (0, b'-', Box::new(i64::wrapping_sub)),
            (1, b'*', Box::new(i64::wrapping_mul)),
        ],
        unary_ops: vec![(b'-', Box::new(i64::wrapping_neg))],
        binary_id: vec![None; 256],
        unary_id: vec![None; 256],
        pos: std::cell::Cell::new(0),
    };
    p.binary_id[b'+' as usize] = Some(0);
    p.binary_id[b'-' as usize] = Some(1);
    p.binary_id[b'*' as usize] = Some(2);
    p.unary_id[b'-' as usize] = Some(0);
    let mut r = rng(16);
    for _ in 0..20000 {
        let mut g = Gen { r: &mut r, spaces: true, out: Vec::new() };
        let exp = g.expr(0, 3);
        let s = g.out;
        assert_eq!(p.parse(&s), exp, "{}", String::from_utf8_lossy(&s));
    }
}

fn gen_range_fixed<T: Copy + algo_lib::numbers::num_traits::primitive::Primitive<u64>>(
    rng: &mut impl RandomTrait,
    f: T,
    t: T,
) -> T
where
    u64: algo_lib::numbers::num_traits::primitive::Primitive<T>,
{
    use algo_lib::numbers::num_traits::primitive::Primitive;
    let fu: u64 = f.to();
    let tu: u64 = t.to();
    let span = tu.wrapping_sub(fu).wrapping_add(1);
    let off = if span == 0 { rng.gen_impl() } else { rng.gen_impl() % span };
    Primitive::<T>::to(fu.wrapping_add(off))
}

#[test]
fn fixed_gen_range() {
    let mut rnd = Random::new_with_seed(18);
    let (mut neg, mut pos) = (false, false);
    for _ in 0..20000 {
        let x: i32 = gen_range_fixed(&mut rnd, -2_000_000_000, 2_000_000_000);
        assert!((-2_000_000_000..=2_000_000_000).contains(&x));
        neg |= x < -1_000_000_000;
        pos |= x > 1_000_000_000;
        let y: i32 = gen_range_fixed(&mut rnd, 0, i32::MAX);
        assert!(y >= 0);
        let z: i64 = gen_range_fixed(&mut rnd, i64::MIN, -1);
        assert!(z < 0);
        let w: i8 = gen_range_fixed(&mut rnd, -3, 3);
        assert!((-3..=3).contains(&w));
        let u: u8 = gen_range_fixed(&mut rnd, 250, 255);
        assert!(u >= 250);
        let _: i64 = gen_range_fixed(&mut rnd, i64::MIN, i64::MAX);
        let v: i64 = gen_range_fixed(&mut rnd, i64::MIN + 1, i64::MAX);
        assert!(v != i64::MIN);
    }
    assert!(neg && pos);
}
