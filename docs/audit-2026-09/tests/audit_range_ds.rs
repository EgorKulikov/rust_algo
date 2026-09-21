#![allow(clippy::all)]
use algo_lib::collections::payload::ValueDeltaPayload;
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::value_delta::ValueDeltaTrait;

const P: i64 = 998_244_353;

// ---------- treap payload: (sum, cnt) with affine (non-commutative) delta ----------
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Aff(i64, i64);
impl Default for Aff {
    fn default() -> Self {
        Aff(1, 0)
    }
}
#[derive(Clone, Copy)]
struct SumAff;
impl ValueDeltaTrait for SumAff {
    type V = (i64, i64);
    type D = Aff;
    fn join(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
        ((a.0 + b.0) % P, a.1 + b.1)
    }
    // d1 existing, d2 applied afterwards
    fn accumulate(d1: Aff, d2: Aff) -> Aff {
        Aff(d1.0 * d2.0 % P, (d1.1 * d2.0 + d2.1) % P)
    }
    fn apply(v: (i64, i64), d: Aff) -> (i64, i64) {
        ((v.0 * d.0 + d.1 * v.1) % P, v.1)
    }
}
type Pl = ValueDeltaPayload<SumAff>;
fn delta(a: i64, b: i64) -> Pl {
    let mut p = Pl::new((0, 0));
    p.d = Aff(a, b);
    p
}


#[path = "audit_range_ds_fixed/treap_fixed.rs"]
mod treap_fixed;
#[path = "audit_range_ds_fixed/persistent_treap_fixed.rs"]
mod persistent_treap_fixed;

macro_rules! treap_tests {
    ($name:ident, $check_ids:expr, $($tree:ident)::+, $($ptree:ident)::+) => {
        mod $name {
            use super::*;
            use $($tree)::+ as Tree;
            use $($ptree)::+ as PersistentTree;
#[test]
fn treap_implicit_long_sequences() {
    let mut rng = Random::new_with_seed(1);
    for iter in 0..3000 {
        let n = if iter % 10 == 0 { rng.gen_range(0..=60usize) } else { rng.gen_range(0..=12usize) };
        let mut arr: Vec<(usize, i64)> = (0..n).map(|i| (i, rng.gen_range(0..10i64))).collect();
        let mut tree: Tree<Pl> = Tree::with_gen(n, |i| Pl::new((arr[i].1, 1)));
        let mut ids = tree.refs();
        for step in 0..200 {
            let n = arr.len();
            let l = rng.gen_range(0..=n);
            let r = rng.gen_range(l..=n);
            let mut op = rng.gen_range(0..15u32);
            if (op == 6 || op == 7) && !$check_ids {
                op = 5;
            }
            if (op == 8 || op == 9 || op == 14) && n > 80 {
                op = 10;
            }
            match op {
                8 | 9 => {
                    let v = rng.gen_range(0..10i64);
                    let id = if op == 8 { tree.add_back(Pl::new((v, 1))) } else { tree.add_front(Pl::new((v, 1))) };
                    ids.push(id);
                    if op == 8 {
                        arr.push((ids.len() - 1, v));
                    } else {
                        arr.insert(0, (ids.len() - 1, v));
                    }
                }
                10 => {
                    let mut d = tree.range_index(l..r).detach();
                    assert_eq!(d.size(), r - l);
                    let got: Vec<i64> = d.iter().map(|p| p.self_v.0).collect();
                    let exp: Vec<i64> = arr.drain(l..r).map(|x| x.1).collect();
                    assert_eq!(got, exp);
                    assert_eq!(tree.size(), arr.len());
                }
                11 => {
                    if n > 0 {
                        let i = rng.gen_range(0..n);
                        let v = rng.gen_range(0..10i64);
                        tree.get_at(i).replace(Pl::new((v, 1)));
                        arr[i].1 = v;
                    }
                }
                12 => {
                    if n > 0 {
                        let i = rng.gen_range(0..n);
                        let mut at = i;
                        let mut found = None;
                        tree.binary_search_with_size(|p, left, _, ls, _| {
                            assert_eq!(left.map_or(0, |l| l.v.1) as usize, ls);
                            if at < ls {
                                Some(Direction::Left)
                            } else if at == ls {
                                found = Some(p.self_v.0);
                                None
                            } else {
                                at -= ls + 1;
                                Some(Direction::Right)
                            }
                        });
                        assert_eq!(found, Some(arr[i].1));
                    }
                }
                13 => {
                    let t = std::mem::take(&mut tree);
                    let (a, rest) = t.split_at(l);
                    let (b, c) = rest.split_at(r - l);
                    tree = Tree::merge_three(c, b, a);
                    let mut na = arr[r..].to_vec();
                    na.extend_from_slice(&arr[l..r]);
                    na.extend_from_slice(&arr[..l]);
                    arr = na;
                }
                14 => {
                    let v = rng.gen_range(0..10i64);
                    if rng.gen_bool() {
                        tree.merge_back(Tree::single(Pl::new((v, 1))));
                        arr.push((usize::MAX, v));
                    } else {
                        tree.merge_front(Tree::single(Pl::new((v, 1))));
                        arr.insert(0, (usize::MAX, v));
                    }
                }
                0 => {
                    let (a, b) = (rng.gen_range(0..3i64), rng.gen_range(0..5i64));
                    tree.range_index(l..r).push(&delta(a, b));
                    for x in &mut arr[l..r] {
                        x.1 = (x.1 * a + b) % P;
                    }
                }
                1 => {
                    tree.range_index(l..r).reverse();
                    arr[l..r].reverse();
                }
                2 => {
                    let got = tree.range_index(l..r).payload().map(|p| p.v);
                    let exp = if l == r {
                        None
                    } else {
                        Some((arr[l..r].iter().fold(0, |a, x| (a + x.1) % P), (r - l) as i64))
                    };
                    assert_eq!(got, exp, "iter {iter} step {step} sum {l}..{r}");
                }
                3 => {
                    if n > 0 {
                        let i = rng.gen_range(0..n);
                        assert_eq!(tree.at(i).map(|p| p.self_v.0), Some(arr[i].1));
                    }
                    assert!(tree.at(n).is_none());
                }
                4 => {
                    let t = std::mem::take(&mut tree);
                    let (a, b) = t.split_at(l);
                    assert_eq!(a.size(), l);
                    tree = Tree::merge(b, a);
                    arr.rotate_left(l);
                }
                5 => {
                    let got: Vec<i64> = tree.iter().map(|p| p.self_v.0).collect();
                    let exp: Vec<i64> = arr.iter().map(|x| x.1).collect();
                    assert_eq!(got, exp);
                    assert_eq!(tree.size(), n);
                    assert_eq!(tree.first().map(|p| p.self_v.0), arr.first().map(|x| x.1));
                    assert_eq!(tree.last().map(|p| p.self_v.0), arr.last().map(|x| x.1));
                }
                6 => {
                    if n > 0 {
                        let i = rng.gen_range(0..n);
                        let id = arr[i].0;
                        if id != usize::MAX {
                            assert_eq!(tree.index_ref(&ids[id]), i, "iter {iter} step {step} index_ref");
                        }
                    }
                }
                _ => {
                    if n > 0 {
                        let i = rng.gen_range(0..n);
                        let id = arr[i].0;
                        if id != usize::MAX {
                            let got = unsafe { ids[id].with_payload(|p| p.self_v.0) };
                            assert_eq!(got, arr[i].1, "iter {iter} step {step} with_payload");
                        }
                    }
                }
            }
        }
    }
}

// minimal: index_ref after a reverse
#[test]
fn treap_index_ref_after_reverse() {
    for _ in 0..200 {
        let n = 3;
        let mut tree: Tree<Pl> = Tree::with_gen(n, |i| Pl::new((i as i64, 1)));
        let ids = tree.refs();
        tree.reverse();
        for i in 0..n {
            assert_eq!(tree.index_ref(&ids[i]), n - 1 - i, "id {i}");
        }
    }
}

// minimal: split_by sees stale child aggregates
#[test]
fn treap_split_by_prefix_sum_after_push() {
    let mut rng = Random::new_with_seed(5);
    for iter in 0..2000 {
        let n = rng.gen_range(1..=6usize);
        let mut arr: Vec<i64> = (0..n).map(|_| rng.gen_range(1..4i64)).collect();
        let mut tree: Tree<Pl> = Tree::with_gen(n, |i| Pl::new((arr[i], 1)));
        let l = rng.gen_range(0..n);
        let r = rng.gen_range(l + 1..=n);
        let b = rng.gen_range(1..5i64);
        tree.range_index(l..r).push(&delta(1, b));
        for x in &mut arr[l..r] {
            *x += b;
        }
        let x = rng.gen_range(0..30i64);
        let mut rem = x;
        let got = tree
            .split_by_head(|p, left, _| {
                let ls = left.map_or(0, |l| l.v.0);
                if ls + p.self_v.0 > rem {
                    Direction::Left
                } else {
                    rem -= ls + p.self_v.0;
                    Direction::Right
                }
            })
            .size();
        let mut exp = 0;
        let mut s = 0;
        while exp < n && s + arr[exp] <= x {
            s += arr[exp];
            exp += 1;
        }
        assert_eq!(got, exp, "iter {iter} arr {arr:?} x {x}");
    }
}

// ---------- persistent treap ----------
#[test]
fn persistent_treap_versions_affine() {
    let mut rng = Random::new_with_seed(7);
    for iter in 0..200 {
        let n = rng.gen_range(1..=8usize);
        let base: Vec<i64> = (0..n).map(|_| rng.gen_range(0..10i64)).collect();
        let mut versions: Vec<(PersistentTree<Pl>, Vec<i64>)> =
            vec![(PersistentTree::with_gen(n, |i| Pl::new((base[i], 1))), base)];
        for step in 0..80 {
            let idx = rng.gen_bound(versions.len());
            let (tree, reference) = versions[idx].clone();
            let len = reference.len();
            let l = rng.gen_range(0..=len);
            let r = rng.gen_range(l..=len);
            match rng.gen_range(0..7u32) {
                0 => {
                    let (a, b) = (rng.gen_range(0..3i64), rng.gen_range(0..5i64));
                    let nt = tree.push_range(l..r, &delta(a, b));
                    let mut nr = reference.clone();
                    for x in &mut nr[l..r] {
                        *x = (*x * a + b) % P;
                    }
                    versions.push((nt, nr));
                }
                1 => {
                    let (a, b, c) = tree.split_range_index(l..r);
                    let nt = PersistentTree::merge_three(a, b.reverse(), c);
                    let mut nr = reference.clone();
                    nr[l..r].reverse();
                    versions.push((nt, nr));
                }
                2 => {
                    // split/push/merge variant
                    let (a, bt, c) = tree.split_range_index(l..r);
                    let (x, y) = (rng.gen_range(0..3i64), rng.gen_range(0..5i64));
                    let nt = PersistentTree::merge_three(a, bt.push(&delta(x, y)), c);
                    let mut nr = reference.clone();
                    for v in &mut nr[l..r] {
                        *v = (*v * x + y) % P;
                    }
                    versions.push((nt, nr));
                }
                3 => {
                    let other = rng.gen_bound(versions.len());
                    let (ot, or) = versions[other].clone();
                    if len + or.len() <= 40 {
                        let mut nr = reference.clone();
                        nr.extend_from_slice(&or);
                        versions.push((PersistentTree::merge(tree, ot), nr));
                    }
                }
                4 => {
                    let got = tree.range_payload(l..r).map(|p| p.v);
                    let exp = if l == r {
                        None
                    } else {
                        Some((reference[l..r].iter().fold(0, |a, x| (a + x) % P), (r - l) as i64))
                    };
                    assert_eq!(got, exp, "iter {iter} step {step}");
                }
                5 => {
                    // binary search by index using sizes
                    if len > 0 {
                        let mut at = rng.gen_range(0..len);
                        let want = reference[at];
                        let mut found = None;
                        tree.binary_search(|p, left, _| {
                            let ls = left.map_or(0, |l| l.v.1) as usize;
                            if at < ls {
                                Some(Direction::Left)
                            } else if at == ls {
                                found = Some(p.self_v.0);
                                None
                            } else {
                                at -= ls + 1;
                                Some(Direction::Right)
                            }
                        });
                        assert_eq!(found, Some(want));
                    }
                }
                _ => {
                    let (a, b) = tree.split_at(l);
                    let mut nr = reference[l..].to_vec();
                    nr.extend_from_slice(&reference[..l]);
                    versions.push((PersistentTree::merge(b, a), nr));
                }
            }
        }
        for (tree, reference) in &versions {
            let values: Vec<i64> = tree.iter().map(|p| p.self_v.0).collect();
            assert_eq!(&values, reference);
            assert_eq!(tree.size(), reference.len());
            assert_eq!(
                tree.payload().map_or(0, |p| p.v.0),
                reference.iter().fold(0, |a, x| (a + x) % P)
            );
            assert_eq!(tree.first().map(|p| p.self_v.0), reference.first().copied());
            assert_eq!(tree.last().map(|p| p.self_v.0), reference.last().copied());
        }
    }
}

#[test]
fn persistent_treap_split_by_prefix_sum() {
    let mut rng = Random::new_with_seed(9);
    for iter in 0..2000 {
        let n = rng.gen_range(1..=6usize);
        let mut arr: Vec<i64> = (0..n).map(|_| rng.gen_range(1..4i64)).collect();
        let mut tree: PersistentTree<Pl> = PersistentTree::with_gen(n, |i| Pl::new((arr[i], 1)));
        let l = rng.gen_range(0..n);
        let r = rng.gen_range(l + 1..=n);
        let b = rng.gen_range(1..5i64);
        tree = tree.push_range(l..r, &delta(1, b));
        for x in &mut arr[l..r] {
            *x += b;
        }
        if rng.gen_bool() {
            tree = tree.reverse();
            arr.reverse();
        }
        let x = rng.gen_range(0..30i64);
        let mut rem = x;
        let got = tree
            .split_by(|p, left, _| {
                let ls = left.map_or(0, |l| l.v.0);
                if ls + p.self_v.0 > rem {
                    Direction::Left
                } else {
                    rem -= ls + p.self_v.0;
                    Direction::Right
                }
            })
            .0
            .size();
        let mut exp = 0;
        let mut s = 0;
        while exp < n && s + arr[exp] <= x {
            s += arr[exp];
            exp += 1;
        }
        assert_eq!(got, exp, "iter {iter} arr {arr:?} x {x}");
    }
}

        }
    };
}
treap_tests!(bug_treap_orig, true, algo_lib::collections::treap::treap::Tree, algo_lib::collections::treap::persistent_treap::PersistentTree);
treap_tests!(treap_fixed_copy, true, treap_fixed::Tree, persistent_treap_fixed::PersistentTree);
// original library with the known-broken NodeId operations left out, to cover everything else
treap_tests!(treap_orig_no_ids, false, algo_lib::collections::treap::treap::Tree, algo_lib::collections::treap::persistent_treap::PersistentTree);

#[test]
fn bug_multi_treap_set_iter() {
    let mut s = MultiTreapSet::new();
    for x in [1, 2, 2, 3, 3, 3, 0] {
        s.insert(x);
    }
    let v: Vec<i32> = s.iter().copied().collect();
    assert_eq!(v, vec![0, 1, 2, 2, 3, 3, 3]);
    let v: Vec<i32> = s.range(&1..&3).copied().collect();
    assert_eq!(v, vec![1, 2, 2]);
}


// ---------- keyed treaps ----------
mod keyed {
    use algo_lib::collections::payload::PurePayload;
    use algo_lib::collections::treap::multi_payload::MultiPayload;
    use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
    use algo_lib::collections::treap::persistent_treap::PersistentTree;
    use algo_lib::collections::treap::treap::Tree;
    use algo_lib::collections::treap::treap_map::TreapSet;
    use algo_lib::misc::random::{Random, RandomTrait};
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn multi_set_everything_but_iter() {
        let mut rng = Random::new_with_seed(11);
        for _ in 0..300 {
            let mut set = MultiTreapSet::new();
            let mut reference: BTreeMap<i32, usize> = BTreeMap::new();
            for _ in 0..200 {
                let key = rng.gen_range(-8..8i32);
                match rng.gen_range(0..8u32) {
                    0 => {
                        set.insert(key);
                        *reference.entry(key).or_default() += 1;
                    }
                    1 => {
                        let q = rng.gen_range(1..4usize);
                        set.insert_few(key, q);
                        *reference.entry(key).or_default() += q;
                    }
                    2 => {
                        assert_eq!(set.remove(&key), reference.contains_key(&key));
                        if let Some(q) = reference.get_mut(&key) {
                            *q -= 1;
                            if *q == 0 {
                                reference.remove(&key);
                            }
                        }
                    }
                    3 => {
                        assert_eq!(set.first(), reference.keys().next());
                        assert_eq!(set.last(), reference.keys().next_back());
                        assert_eq!(set.prev(&key), reference.range(..key).next_back().map(|x| x.0));
                        assert_eq!(set.floor(&key), reference.range(..=key).next_back().map(|x| x.0));
                        assert_eq!(set.ceil(&key), reference.range(key..).next().map(|x| x.0));
                        assert_eq!(set.next(&key), reference.range(key + 1..).next().map(|x| x.0));
                    }
                    4 => {
                        let keys: Vec<i32> = set.keys().copied().collect();
                        assert_eq!(keys, reference.keys().copied().collect::<Vec<_>>());
                        assert_eq!(set.distinct(), reference.len());
                        assert_eq!(set.len(), reference.values().sum::<usize>());
                        assert_eq!(set.is_empty(), reference.is_empty());
                    }
                    5 => {
                        let hi = rng.gen_range(-8..8i32);
                        let (lo, hi) = (key.min(hi), key.max(hi));
                        // distinct keys in range via dedup (iter multiplicity is a known bug)
                        let mut got: Vec<i32> = set.range(&lo..&hi).copied().collect();
                        got.dedup();
                        assert_eq!(got, reference.range(lo..hi).map(|x| *x.0).collect::<Vec<_>>());
                        // structure must still be intact after range()
                        assert_eq!(set.len(), reference.values().sum::<usize>());
                    }
                    6 => {
                        if rng.gen_range(0..20u32) == 0 {
                            set.clear();
                            reference.clear();
                        }
                    }
                    _ => {
                        let less: usize = reference.range(..key).map(|x| x.1).sum();
                        let leq: usize = reference.range(..=key).map(|x| x.1).sum();
                        assert_eq!(set.lower_bound(&key), less);
                        assert_eq!(set.upper_bound(&key), leq);
                        assert_eq!(set.get(&key), reference.get(&key).copied().unwrap_or(0));
                        assert_eq!(set.index(&key), reference.contains_key(&key).then_some(less));
                    }
                }
            }
        }
    }

    #[test]
    fn treap_set_vs_btree() {
        let mut rng = Random::new_with_seed(12);
        for _ in 0..300 {
            let mut set = TreapSet::new();
            let mut reference = BTreeSet::new();
            for _ in 0..200 {
                let key = rng.gen_range(-10..10i32);
                match rng.gen_range(0..5u32) {
                    0 | 1 => {
                        // NOTE: returns whether the key WAS present (opposite of BTreeSet)
                        assert_eq!(set.insert(key), !reference.insert(key));
                    }
                    2 => assert_eq!(set.remove(&key), reference.remove(&key)),
                    3 => {
                        let hi = rng.gen_range(-10..10i32);
                        let (lo, hi) = (key.min(hi), key.max(hi));
                        let got: Vec<i32> = set.range(&lo..=&hi).copied().collect();
                        assert_eq!(got, reference.range(lo..=hi).copied().collect::<Vec<_>>());
                        let got: Vec<i32> = set.range(&lo..&hi).copied().collect();
                        assert_eq!(got, reference.range(lo..hi).copied().collect::<Vec<_>>());
                        use std::ops::Bound::*;
                        let got: Vec<i32> = set.range((Excluded(&lo), Unbounded)).copied().collect();
                        assert_eq!(got, reference.range((Excluded(lo), Unbounded)).copied().collect::<Vec<_>>());
                    }
                    _ => {
                        assert_eq!(set.iter().copied().collect::<Vec<_>>(), reference.iter().copied().collect::<Vec<_>>());
                        assert_eq!(set.first(), reference.iter().next());
                        assert_eq!(set.last(), reference.iter().next_back());
                        assert_eq!(set.lower(&key), reference.range(..key).next_back());
                        assert_eq!(set.higher(&key), reference.range(key + 1..).next());
                        assert_eq!(set.len(), reference.len());
                        if !reference.is_empty() {
                            let at = rng.gen_range(0..reference.len());
                            assert_eq!(set.get_at(at), reference.iter().nth(at));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn tree_union_multi() {
        let mut rng = Random::new_with_seed(13);
        for _ in 0..2000 {
            let mut trees: Vec<Tree<MultiPayload<i32>>> = Vec::new();
            let mut reference: BTreeMap<i32, usize> = BTreeMap::new();
            for _ in 0..rng.gen_range(0..4usize) {
                let mut t = Tree::new();
                for _ in 0..rng.gen_range(0..8usize) {
                    let k = rng.gen_range(0..10i32);
                    t.insert_or_update(MultiPayload::new(k, ()));
                    *reference.entry(k).or_default() += 1;
                }
                trees.push(t);
            }
            let mut all = Tree::new();
            for t in trees {
                all = Tree::union(all, t);
            }
            let got: Vec<(i32, usize)> = all.iter().map(|p| (p.key, p.self_size)).collect();
            assert_eq!(got, reference.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>());
            assert_eq!(all.payload().map_or(0, |p| p.total_size), reference.values().sum::<usize>());
            // split + totals
            let k = rng.gen_range(0..10i32);
            let (mut a, mut b) = all.split(&k);
            assert_eq!(a.payload().map_or(0, |p| p.total_size), reference.range(..k).map(|x| x.1).sum::<usize>());
            assert_eq!(b.payload().map_or(0, |p| p.total_size), reference.range(k..).map(|x| x.1).sum::<usize>());
        }
    }

    #[test]
    fn persistent_keyed_union_and_lookups() {
        let mut rng = Random::new_with_seed(14);
        for _ in 0..300 {
            let mut versions: Vec<(PersistentTree<PurePayload<i32>>, BTreeSet<i32>)> =
                vec![(PersistentTree::new(), BTreeSet::new())];
            for _ in 0..80 {
                let idx = rng.gen_bound(versions.len());
                let (tree, reference) = versions[idx].clone();
                let key = rng.gen_range(0..20i32);
                match rng.gen_range(0..6u32) {
                    0 => {
                        let mut r = reference.clone();
                        r.insert(key);
                        versions.push((tree.insert_or_update(PurePayload(key)), r));
                    }
                    1 => {
                        let (t, old) = tree.remove(&key);
                        let mut r = reference.clone();
                        assert_eq!(old.is_some(), r.remove(&key));
                        versions.push((t, r));
                    }
                    2 => {
                        let o = rng.gen_bound(versions.len());
                        let (ot, or) = versions[o].clone();
                        let r: BTreeSet<i32> = reference.union(&or).copied().collect();
                        versions.push((PersistentTree::union(tree, ot), r));
                    }
                    3 => {
                        let hi = rng.gen_range(0..20i32);
                        let (lo, hi) = (key.min(hi), key.max(hi));
                        let r: BTreeSet<i32> = reference.range(lo..hi).copied().collect();
                        versions.push((tree.range(&lo..&hi), r));
                    }
                    _ => {
                        assert_eq!(tree.prev(&key).map(|p| p.0), reference.range(..key).next_back().copied());
                        assert_eq!(tree.next(&key).map(|p| p.0), reference.range(key + 1..).next().copied());
                        assert_eq!(tree.floor(&key).map(|p| p.0), reference.range(..=key).next_back().copied());
                        assert_eq!(tree.ceil(&key).map(|p| p.0), reference.range(key..).next().copied());
                        assert_eq!(tree.index(&key), reference.contains(&key).then(|| reference.range(..key).count()));
                    }
                }
            }
            for (t, r) in &versions {
                assert_eq!(t.iter().map(|p| p.0).collect::<Vec<_>>(), r.iter().copied().collect::<Vec<_>>());
                assert_eq!(t.size(), r.len());
            }
        }
    }
}

// ---------- minimal reproductions (each is self-contained) ----------
mod bug_min {
    #[test]
    fn multi_treap_set_iter_multiplicity() {
        use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
        let mut s = MultiTreapSet::new();
        s.insert(1);
        s.insert(2);
        assert_eq!(s.iter().copied().collect::<Vec<i32>>(), vec![1, 2]);
    }

    #[test]
    fn multi_treap_set_range_multiplicity() {
        use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
        let mut s = MultiTreapSet::new();
        s.insert(1);
        s.insert(2);
        assert_eq!(s.range(&0..&5).copied().collect::<Vec<i32>>(), vec![1, 2]);
    }

    #[test]
    fn treap_index_ref_after_reverse() {
        use algo_lib::collections::payload::PurePayload;
        use algo_lib::collections::treap::treap::Tree;
        let n = 4;
        for rep in 0..100 {
            let i = rep % n;
            // fresh tree per query (treap shape is random): any node at depth >= 3 triggers the bug
            let mut tree: Tree<PurePayload<usize>> = Tree::with_gen(n, PurePayload);
            let ids = tree.refs();
            tree.reverse();
            assert_eq!(tree.raise(&ids[i]).payload().map(|p| p.0), Some(i), "raise id {i}");
            assert_eq!(tree.index_ref(&ids[i]), n - 1 - i, "index_ref id {i}");
        }
    }

    #[test]
    fn treap_node_id_with_payload_stale() {
        use algo_lib::collections::payload::ValueDeltaPayload;
        use algo_lib::collections::treap::treap::Tree;
        use algo_lib::misc::value_delta::ValueDeltaTrait;
        #[derive(Clone, Copy)]
        struct Add;
        impl ValueDeltaTrait for Add {
            type V = i64;
            type D = i64;
            fn join(a: i64, b: i64) -> i64 {
                a.max(b)
            }
            fn accumulate(a: i64, b: i64) -> i64 {
                a + b
            }
            fn apply(v: i64, d: i64) -> i64 {
                v + d
            }
        }
        let mut tree: Tree<ValueDeltaPayload<Add>> = Tree::with_gen(2, |_| ValueDeltaPayload::new(0));
        let ids = tree.refs();
        let mut delta = ValueDeltaPayload::<Add>::new(0);
        delta.d = 5;
        tree.push(&delta); // add 5 to every element
        for id in &ids {
            assert_eq!(unsafe { id.with_payload(|p| p.self_v) }, 5);
        }
    }

    #[test]
    fn treap_split_by_sees_stale_children() {
        use algo_lib::collections::payload::ValueDeltaPayload;
        use algo_lib::collections::treap::treap::Tree;
        use algo_lib::misc::direction::Direction;
        use algo_lib::misc::value_delta::ValueDeltaTrait;
        #[derive(Clone, Copy)]
        struct SumAdd;
        impl ValueDeltaTrait for SumAdd {
            type V = (i64, i64); // (sum, count)
            type D = i64;
            fn join(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
                (a.0 + b.0, a.1 + b.1)
            }
            fn accumulate(a: i64, b: i64) -> i64 {
                a + b
            }
            fn apply(v: (i64, i64), d: i64) -> (i64, i64) {
                (v.0 + d * v.1, v.1)
            }
        }
        for _ in 0..64 {
            // treap shape is random, the failure needs a root with a left child
            let mut tree: Tree<ValueDeltaPayload<SumAdd>> =
                Tree::with_gen(2, |_| ValueDeltaPayload::new((1, 1)));
            let mut delta = ValueDeltaPayload::<SumAdd>::new((0, 0));
            delta.d = 5;
            tree.push(&delta); // elements are now [6, 6]
            let mut rem = 7; // longest prefix with sum <= 7 has length 1
            let len = tree
                .split_by_head(|p, left, _| {
                    let here = left.map_or(0, |l| l.v.0) + p.self_v.0;
                    if here > rem {
                        Direction::Left
                    } else {
                        rem -= here;
                        Direction::Right
                    }
                })
                .size();
            assert_eq!(len, 1);
        }
    }

    #[test]
    fn persistent_treap_split_by_ignores_reverse() {
        use algo_lib::collections::payload::ValueDeltaPayload;
        use algo_lib::collections::treap::persistent_treap::PersistentTree;
        use algo_lib::misc::direction::Direction;
        use algo_lib::misc::value_delta::ValueDeltaTrait;
        #[derive(Clone, Copy)]
        struct Sum;
        impl ValueDeltaTrait for Sum {
            type V = i64;
            type D = ();
            fn join(a: i64, b: i64) -> i64 {
                a + b
            }
            fn accumulate(_: (), _: ()) {}
            fn apply(v: i64, _: ()) -> i64 {
                v
            }
        }
        let tree: PersistentTree<ValueDeltaPayload<Sum>> =
            PersistentTree::with_gen(2, |i| ValueDeltaPayload::new(i as i64 + 1));
        let tree = tree.reverse(); // [2, 1]
        let mut rem = 2; // longest prefix with sum <= 2 is [2]
        let (head, tail) = tree.split_by(|p, left, _| {
            let here = left.map_or(0, |l| l.v) + p.self_v;
            if here > rem {
                Direction::Left
            } else {
                rem -= here;
                Direction::Right
            }
        });
        assert_eq!((head.size(), tail.size()), (1, 1));
    }
}

// ---------- old recursive SegmentTree ----------
mod old_segment_tree {
    use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
    use algo_lib::misc::direction::Direction;
    use algo_lib::misc::random::{Random, RandomTrait};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    enum D {
        #[default]
        None,
        Add(i64),
        Assign(i64),
    }

    #[derive(Clone, Debug, Default)]
    struct Node {
        sum: i64,
        mn: i64,
        mx: i64,
        len: i64,
        d: D,
    }

    impl Node {
        fn leaf(v: i64) -> Self {
            Node { sum: v, mn: v, mx: v, len: 1, d: D::None }
        }
        fn delta(d: D) -> Self {
            Node { d, ..Default::default() }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, l: &Self, r: &Self) {
            self.sum = l.sum + r.sum;
            self.mn = l.mn.min(r.mn);
            self.mx = l.mx.max(r.mx);
            self.len = l.len + r.len;
        }
        fn accumulate(&mut self, value: &Self) {
            match value.d {
                D::None => {}
                D::Add(v) => {
                    self.sum += v * self.len;
                    self.mn += v;
                    self.mx += v;
                    self.d = match self.d {
                        D::None => D::Add(v),
                        D::Add(w) => D::Add(v + w),
                        D::Assign(w) => D::Assign(v + w),
                    };
                }
                D::Assign(v) => {
                    self.sum = v * self.len;
                    self.mn = v;
                    self.mx = v;
                    self.d = D::Assign(v);
                }
            }
        }
        fn reset_delta(&mut self) {
            self.d = D::None;
        }
    }

    #[test]
    fn all_entry_points() {
        let mut rng = Random::new_with_seed(21);
        for iter in 0..3000 {
            let n = if iter % 7 == 0 { rng.gen_range(1..=40usize) } else { rng.gen_range(1..=9usize) };
            let mut arr: Vec<i64> = (0..n).map(|_| rng.gen_range(-5..6i64)).collect();
            let mut st: SegmentTree<Node> = match iter % 3 {
                0 => SegmentTree::with_gen(n, |i| Node::leaf(arr[i])),
                1 => SegmentTree::from_array(arr.iter().map(|&v| Node::leaf(v)).collect()),
                _ => SegmentTree::with_gen_full(n, |l, r| {
                    if l + 1 == r {
                        Node::leaf(arr[l])
                    } else {
                        Node { len: (r - l) as i64, ..Default::default() }
                    }
                }),
            };
            for step in 0..60 {
                let l = rng.gen_range(0..=n);
                let r = rng.gen_range(l..=n);
                let x = rng.gen_range(-7..8i64);
                let ctx = format!("iter {iter} step {step} n {n} {l}..{r} x {x} arr {arr:?}");
                match rng.gen_range(0..13u32) {
                    0 => {
                        st.update(l..r, &Node::delta(D::Add(x)));
                        arr[l..r].iter_mut().for_each(|v| *v += x);
                    }
                    1 => {
                        st.update(l..r, &Node::delta(D::Assign(x)));
                        arr[l..r].iter_mut().for_each(|v| *v = x);
                    }
                    2 => {
                        let q = st.query(l..r);
                        if l < r {
                            assert_eq!(q.sum, arr[l..r].iter().sum::<i64>(), "{ctx}");
                            assert_eq!(q.mn, *arr[l..r].iter().min().unwrap(), "{ctx}");
                            assert_eq!(q.mx, *arr[l..r].iter().max().unwrap(), "{ctx}");
                            assert_eq!(q.len, (r - l) as i64);
                        } else {
                            assert_eq!(q.len, 0);
                        }
                        // inclusive / unbounded forms
                        if l < r {
                            assert_eq!(st.query(l..=r - 1).sum, q.sum);
                        }
                        assert_eq!(st.query(..).sum, arr.iter().sum::<i64>());
                        assert_eq!(st.query(l..).sum, arr[l..].iter().sum::<i64>());
                        assert_eq!(st.query(..r).sum, arr[..r].iter().sum::<i64>());
                        assert_eq!(st.query(l..n + 5).sum, arr[l..].iter().sum::<i64>());
                    }
                    3 => {
                        let i = rng.gen_range(0..n);
                        assert_eq!(st.point_query(i).sum, arr[i], "{ctx}");
                        assert_eq!(st.point_query(i).len, 1);
                    }
                    4 => {
                        let i = rng.gen_range(0..n);
                        st.point_update(i, Node::leaf(x));
                        arr[i] = x;
                    }
                    5 => {
                        let i = rng.gen_range(0..n);
                        st.point_through_update(i, |node| {
                            if node.len == 1 {
                                node.sum += x;
                                node.mn += x;
                                node.mx += x;
                            }
                        });
                        arr[i] += x;
                    }
                    6 => {
                        // first index in l..r with value >= x
                        let got = st.binary_search_in(l..r, |node| node.mx >= x, |node, i| (node.sum, i));
                        let exp = (l..r).find(|&i| arr[i] >= x).map(|i| (arr[i], i));
                        assert_eq!(got, exp, "{ctx}");
                    }
                    7 => {
                        let got = st.binary_search_in_rtl(l..r, |node| node.mn <= x, |node, i| (node.sum, i));
                        let exp = (l..r).rev().find(|&i| arr[i] <= x).map(|i| (arr[i], i));
                        assert_eq!(got, exp, "{ctx}");
                    }
                    8 => {
                        // mutable: find first >= x and assign it to x - 100 .. no: set to -6 (below all x)
                        let got = st.binary_search_in_mut(l..r, |node| node.mx >= x, |node, i| {
                            *node = Node::leaf(-9);
                            i
                        });
                        let exp = (l..r).find(|&i| arr[i] >= x);
                        assert_eq!(got, exp, "{ctx}");
                        if let Some(i) = exp {
                            arr[i] = -9;
                        }
                    }
                    9 => {
                        let got = st.binary_search_in_mut_rtl(l..r, |node| node.mx >= x, |node, i| {
                            *node = Node::leaf(-9);
                            i
                        });
                        let exp = (l..r).rev().find(|&i| arr[i] >= x);
                        assert_eq!(got, exp, "{ctx}");
                        if let Some(i) = exp {
                            arr[i] = -9;
                        }
                    }
                    10 => {
                        // global descent: leftmost position of the maximum
                        let got = st.binary_search(
                            |lf, rt| if lf.mx >= rt.mx { Direction::Left } else { Direction::Right },
                            |node, i| (node.sum, i),
                        );
                        let mx = *arr.iter().max().unwrap();
                        let pos = arr.iter().position(|&v| v == mx).unwrap();
                        assert_eq!(got, (mx, pos), "{ctx}");
                    }
                    11 => {
                        // binary_search_mut: rightmost minimum, then bump it by 1
                        let got = st.binary_search_mut(
                            |_, lf, rt| if lf.mn < rt.mn { Direction::Left } else { Direction::Right },
                            |node, i| {
                                let v = node.sum;
                                *node = Node::leaf(v + 1);
                                (v, i)
                            },
                        );
                        let mn = *arr.iter().min().unwrap();
                        let pos = arr.iter().rposition(|&v| v == mn).unwrap();
                        assert_eq!(got, (mn, pos), "{ctx}");
                        arr[pos] += 1;
                    }
                    _ => {
                        let cnt = st.for_each(l..r, |acc: i64, node| acc + node.sum);
                        assert_eq!(cnt, arr[l..r].iter().sum::<i64>(), "{ctx}");
                        let cnt = st.for_each_mut(l..r, |acc: i64, node| {
                            node.accumulate(&Node::delta(D::Add(1)));
                            acc + node.len
                        });
                        assert_eq!(cnt, (r - l) as i64);
                        arr[l..r].iter_mut().for_each(|v| *v += 1);
                    }
                }
            }
            for i in 0..n {
                assert_eq!(st.point_query(i).sum, arr[i]);
            }
        }
    }

    #[test]
    fn empty_tree() {
        let mut st: SegmentTree<Node> = SegmentTree::new(0);
        assert_eq!(st.query(..).len, 0);
        st.update(.., &Node::delta(D::Add(1)));
        assert_eq!(st.binary_search_in(.., |_| true, |_, i| i), None);
    }
}

// ---------- bottom-up SegTree / LazySegTree ----------
mod acl_seg_tree {
    use algo_lib::collections::seg_tree::{LazyMonoid, LazySegTree, Monoid, SegTree};
    use algo_lib::misc::random::{Random, RandomTrait};

    const P: u64 = 998_244_353;
    const B: u64 = 911;

    // sequence hash (non-commutative): (hash, B^len, sum_{i<len} B^i)
    struct Hash;
    impl Monoid for Hash {
        type V = (u64, u64, u64);
        fn e() -> Self::V {
            (0, 1, 0)
        }
        fn op(a: Self::V, b: Self::V) -> Self::V {
            ((a.0 * b.1 + b.0) % P, a.1 * b.1 % P, (a.2 * b.1 + b.2) % P)
        }
    }
    impl LazyMonoid for Hash {
        type F = (u64, u64); // c -> f.0 * c + f.1 on every character
        fn id() -> Self::F {
            (1, 0)
        }
        fn mapping(f: Self::F, v: Self::V) -> Self::V {
            ((f.0 * v.0 + f.1 * v.2) % P, v.1, v.2)
        }
        fn composition(f: Self::F, g: Self::F) -> Self::F {
            (f.0 * g.0 % P, (f.0 * g.1 + f.1) % P)
        }
    }
    fn leaf(c: u64) -> (u64, u64, u64) {
        (c % P, B, 1)
    }
    fn naive(a: &[u64]) -> (u64, u64, u64) {
        a.iter().fold(Hash::e(), |acc, &c| Hash::op(acc, leaf(c)))
    }

    struct Sum;
    impl Monoid for Sum {
        type V = i64;
        fn e() -> i64 {
            0
        }
        fn op(a: i64, b: i64) -> i64 {
            a + b
        }
    }

    #[test]
    fn seg_tree_hash_and_searches() {
        let mut rng = Random::new_with_seed(31);
        for iter in 0..3000 {
            let n = if iter % 10 == 0 { rng.gen_range(0..=70usize) } else { rng.gen_range(0..=9usize) };
            let mut arr: Vec<u64> = (0..n).map(|_| rng.gen_range(0..5u64)).collect();
            let mut st = SegTree::<Hash>::with_gen(n, |i| leaf(arr[i]));
            let mut sums = SegTree::<Sum>::with_gen(n, |i| arr[i] as i64);
            assert_eq!(st.len(), n);
            assert_eq!(st.is_empty(), n == 0);
            for _ in 0..50 {
                let l = rng.gen_range(0..=n);
                let r = rng.gen_range(l..=n);
                match rng.gen_range(0..4u32) {
                    0 if n > 0 => {
                        let i = rng.gen_range(0..n);
                        let v = rng.gen_range(0..5u64);
                        arr[i] = v;
                        if rng.gen_bool() {
                            st.set(i, leaf(v));
                        } else {
                            st.update(i, |x| *x = leaf(v));
                        }
                        sums.set(i, v as i64);
                        assert_eq!(st.get(i), leaf(v));
                    }
                    1 => {
                        assert_eq!(st.query(l..r), naive(&arr[l..r]));
                        assert_eq!(st.query(..), naive(&arr));
                        assert_eq!(st.all(), naive(&arr));
                        if l < r {
                            assert_eq!(st.query(l..=r - 1), naive(&arr[l..r]));
                        }
                        assert_eq!(st.query(l..n + 3), naive(&arr[l..]));
                        assert_eq!(st.query(r..l), Hash::e());
                    }
                    2 => {
                        let bound = rng.gen_range(0..12i64) - if rng.gen_range(0..5u32) == 0 { 0 } else { 0 };
                        let bound = if rng.gen_range(0..6u32) == 0 { 1_000_000 } else { bound };
                        let exp = (l..=n).rev().find(|&r| arr[l..r].iter().sum::<u64>() as i64 <= bound).unwrap();
                        // with zeros present the largest r is what is documented
                        assert_eq!(sums.max_right(l, |s| s <= bound), exp, "n {n} l {l} bound {bound} {arr:?}");
                    }
                    _ => {
                        let bound = rng.gen_range(0..12i64);
                        let bound = if rng.gen_range(0..6u32) == 0 { 1_000_000 } else { bound };
                        let exp = (0..=r).find(|&l| arr[l..r].iter().sum::<u64>() as i64 <= bound).unwrap();
                        assert_eq!(sums.min_left(r, |s| s <= bound), exp, "n {n} r {r} bound {bound} {arr:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn lazy_seg_tree_hash_affine() {
        let mut rng = Random::new_with_seed(32);
        for iter in 0..4000 {
            let n = if iter % 10 == 0 { rng.gen_range(0..=70usize) } else { rng.gen_range(0..=9usize) };
            let mut arr: Vec<u64> = (0..n).map(|_| rng.gen_range(0..5u64)).collect();
            let mut st = match iter % 3 {
                0 => LazySegTree::<Hash>::with_gen(n, |i| leaf(arr[i])),
                1 => LazySegTree::<Hash>::from_slice(&arr.iter().map(|&c| leaf(c)).collect::<Vec<_>>()),
                _ => {
                    let mut st = LazySegTree::<Hash>::new(n);
                    for i in 0..n {
                        st.set(i, leaf(arr[i]));
                    }
                    st
                }
            };
            for _ in 0..60 {
                let l = rng.gen_range(0..=n);
                let r = rng.gen_range(l..=n);
                match rng.gen_range(0..6u32) {
                    0 | 1 => {
                        let f = (rng.gen_range(0..4u64), rng.gen_range(0..4u64));
                        st.apply(l..r, f);
                        arr[l..r].iter_mut().for_each(|c| *c = (f.0 * *c + f.1) % P);
                    }
                    2 if n > 0 => {
                        let i = rng.gen_range(0..n);
                        let f = (rng.gen_range(0..4u64), rng.gen_range(0..4u64));
                        st.apply_at(i, f);
                        arr[i] = (f.0 * arr[i] + f.1) % P;
                    }
                    3 if n > 0 => {
                        let i = rng.gen_range(0..n);
                        assert_eq!(st.get(i), leaf(arr[i]));
                        let v = rng.gen_range(0..5u64);
                        if rng.gen_bool() {
                            st.set(i, leaf(v));
                            arr[i] = v;
                        } else {
                            st.update(i, |x| x.0 = (x.0 + v) % P);
                            arr[i] = (arr[i] + v) % P;
                        }
                    }
                    _ => {
                        assert_eq!(st.query(l..r), naive(&arr[l..r]), "n {n} {l}..{r}");
                        assert_eq!(st.query(..), naive(&arr));
                        if n > 0 {
                            assert_eq!(st.all(), naive(&arr));
                        }
                        assert_eq!(st.query(l..n + 3), naive(&arr[l..]));
                    }
                }
            }
        }
    }
}

// ---------- the rest ----------
mod misc_structs {
    use algo_lib::collections::fast_clear_fenwick::FastClearFenwickTree;
    use algo_lib::collections::fenwick::FenwickTree;
    use algo_lib::collections::foldable_deque::FoldableDeque;
    use algo_lib::collections::int_set::IntSet;
    use algo_lib::collections::interval_heap::IntervalHeap;
    use algo_lib::collections::li_chao::LiChao;
    use algo_lib::collections::persistent_fenwick::PersistentFenwickTree;
    use algo_lib::collections::rectangle_sum::rectangle_sums;
    use algo_lib::collections::rmq::Rmq;
    use algo_lib::collections::sliding_window::SlidingWindow;
    use algo_lib::collections::sparse_table::SparseTable;
    use algo_lib::collections::sparse_table_pos::SparseTableWithPos;
    use algo_lib::collections::wavelet_matrix::WaveletMatrix;
    use algo_lib::misc::direction::Direction;
    use algo_lib::misc::random::{Random, RandomTrait};
    use std::collections::{BTreeSet, VecDeque};
    use std::ops::Bound::*;

    #[test]
    fn fenwick() {
        let mut rng = Random::new_with_seed(41);
        for _ in 0..3000 {
            let n = rng.gen_range(0..=20usize);
            let mut arr: Vec<i64> = (0..n).map(|_| rng.gen_range(-9..10i64)).collect();
            let mut ft = if rng.gen_bool() {
                FenwickTree::from(arr.as_slice())
            } else {
                let mut ft = FenwickTree::new(n);
                for i in 0..n {
                    ft.add(i, arr[i]);
                }
                ft
            };
            for _ in 0..30 {
                let l = rng.gen_range(0..=n);
                let r = rng.gen_range(l..=n);
                match rng.gen_range(0..4u32) {
                    0 if n > 0 => {
                        let i = rng.gen_range(0..n);
                        let v = rng.gen_range(-9..10i64);
                        arr[i] += v;
                        ft.add(i, v);
                    }
                    1 => {
                        assert_eq!(ft.get(l..r), arr[l..r].iter().sum::<i64>());
                        assert_eq!(ft.get(r..l), 0);
                        assert_eq!(ft.get(..), arr.iter().sum::<i64>());
                        assert_eq!(ft.get(l..n + 7), arr[l..].iter().sum::<i64>());
                        assert_eq!(ft.get_to(r), arr[..r].iter().sum::<i64>());
                        if l < r {
                            assert_eq!(ft.get(l..=r - 1), arr[l..r].iter().sum::<i64>());
                            assert_eq!(ft.get((Excluded(l), Included(r - 1))), arr[l + 1..r].iter().sum::<i64>());
                        }
                    }
                    2 => assert_eq!(ft.iter().collect::<Vec<_>>(), arr),
                    _ => {
                        if rng.gen_range(0..10u32) == 0 {
                            ft.clear();
                            arr.fill(0);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn fast_clear_fenwick() {
        let mut rng = Random::new_with_seed(42);
        for _ in 0..2000 {
            let n = rng.gen_range(0..=20usize);
            let mut arr = vec![0i64; n];
            let mut ft = FastClearFenwickTree::new(n);
            for _ in 0..60 {
                let l = rng.gen_range(0..=n);
                let r = rng.gen_range(l..=n);
                match rng.gen_range(0..5u32) {
                    0 | 1 if n > 0 => {
                        let i = rng.gen_range(0..n);
                        let v = rng.gen_range(-9..10i64);
                        arr[i] += v;
                        ft.add(i, v);
                    }
                    2 => {
                        assert_eq!(ft.get(l, r), arr[l..r].iter().sum::<i64>());
                        assert_eq!(ft.get(l, n + 3), arr[l..].iter().sum::<i64>());
                        assert_eq!(ft.get_to(r), arr[..r].iter().sum::<i64>());
                    }
                    3 => assert_eq!(ft.iter().collect::<Vec<_>>(), arr),
                    _ => {
                        if rng.gen_range(0..4u32) == 0 {
                            ft.clear();
                            arr.fill(0);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn persistent_fenwick() {
        let mut rng = Random::new_with_seed(43);
        for _ in 0..1000 {
            let n = rng.gen_range(0..=12usize);
            let base = rng.gen_range(-3..3i32);
            let mut ft = PersistentFenwickTree::new(n, base);
            let mut history: Vec<(i32, Vec<i64>)> = vec![(base, vec![0i64; n])];
            let mut epoch = base;
            for _ in 0..40 {
                if rng.gen_bool() && n > 0 {
                    epoch += rng.gen_range(0..3i32);
                    let i = rng.gen_range(0..n);
                    let v = rng.gen_range(-9..10i64);
                    ft.add(i, v, epoch);
                    let mut cur = history.last().unwrap().1.clone();
                    cur[i] += v;
                    history.push((epoch, cur));
                } else {
                    let e = rng.gen_range(base..=epoch + 2);
                    let arr = &history.iter().rev().find(|h| h.0 <= e).unwrap().1;
                    let l = rng.gen_range(0..=n);
                    let r = rng.gen_range(l..=n);
                    assert_eq!(ft.get(l, r, e), arr[l..r].iter().sum::<i64>());
                    assert_eq!(ft.get_to(r, e), arr[..r].iter().sum::<i64>());
                    assert_eq!(&ft.iter(e).collect::<Vec<_>>(), arr);
                }
            }
        }
    }

    #[test]
    fn rmq_and_sparse_tables() {
        let mut rng = Random::new_with_seed(44);
        for iter in 0..400 {
            let n = if iter % 20 == 0 { rng.gen_range(1..=400usize) } else { rng.gen_range(1..=70usize) };
            let arr: Vec<i32> = (0..n).map(|_| rng.gen_range(-3..3i32)).collect();
            let rmq = Rmq::new(arr.clone());
            let st = SparseTable::new(arr.clone(), |a: &i32, b: &i32| *a.min(b));
            let stp = SparseTableWithPos::new(&arr, |a: &i32, b: &i32| {
                if a <= b {
                    Direction::Left
                } else {
                    Direction::Right
                }
            });
            for _ in 0..200 {
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l + 1..=n);
                let pos = (l..r).min_by_key(|&i| (arr[i], i)).unwrap();
                assert_eq!(rmq.argmin(l..r), pos);
                assert_eq!(rmq.min(l..=r - 1), arr[pos]);
                assert_eq!(st.query(l..r), arr[pos]);
                assert_eq!(stp.query(l..r), (arr[pos], pos));
            }
            assert_eq!(rmq.argmin(..), (0..n).min_by_key(|&i| (arr[i], i)).unwrap());
        }
        let empty = Rmq::<i32>::new(vec![]);
        assert!(empty.is_empty());
    }

    #[test]
    fn li_chao_large_values() {
        let mut rng = Random::new_with_seed(45);
        for iter in 0..300 {
            let big = iter % 2 == 0;
            let (xr, ar, br) = if big {
                (1_000_000_000i64, 1_000_000_000i64, 1_000_000_000_000_000_000i64)
            } else {
                (6, 4, 10)
            };
            let cnt = rng.gen_range(1..=40usize);
            let points: Vec<i64> = (0..cnt).map(|_| rng.gen_range(-xr..=xr)).collect();
            let mut tree = LiChao::with_points(points.clone());
            let mut items: Vec<(i64, i64, i64, i64)> = Vec::new();
            for _ in 0..60 {
                let a = rng.gen_range(-ar..=ar);
                let b = rng.gen_range(-br..=br);
                if rng.gen_bool() {
                    tree.add_line(a, b);
                    items.push((i64::MIN, i64::MAX, a, b));
                } else {
                    let f = rng.gen_range(-xr - 1..=xr + 1);
                    let t = rng.gen_range(-xr - 1..=xr + 1);
                    tree.add_segment(f, t, a, b); // f >= t must be a no-op
                    items.push((f, t, a, b));
                }
                for _ in 0..4 {
                    let x = points[rng.gen_range(0..cnt)];
                    let exp = items
                        .iter()
                        .filter(|&&(f, t, _, _)| f <= x && x < t)
                        .map(|&(_, _, a, b)| a * x + b)
                        .min();
                    assert_eq!(tree.min(x), exp);
                }
            }
        }
    }

    #[test]
    fn wavelet_small_alphabet() {
        let mut rng = Random::new_with_seed(46);
        for iter in 0..1500 {
            let n = rng.gen_range(0..=70usize);
            let max = [0u64, 1, 2, 7, 8, u64::MAX][iter % 6];
            let arr: Vec<u64> = (0..n)
                .map(|_| match rng.gen_range(0..4u32) {
                    0 => 0,
                    1 => max,
                    2 => max / 2,
                    _ => max - max.min(rng.gen_range(0..3u64)),
                })
                .collect();
            let wm = WaveletMatrix::new(&arr);
            assert_eq!(wm.len(), n);
            for _ in 0..40 {
                let l = rng.gen_range(0..=n);
                let r = rng.gen_range(l..=n);
                let mut sorted = arr[l..r].to_vec();
                sorted.sort();
                let k = rng.gen_range(0..=r - l + 1);
                assert_eq!(wm.kth_smallest(l..r, k), sorted.get(k).copied());
                assert_eq!(wm.kth_largest(l..r, k), sorted.iter().rev().nth(k).copied());
                for x in [0, 1, max / 2, max.saturating_sub(1), max, max.saturating_add(1), u64::MAX] {
                    assert_eq!(wm.count_less(l..r, x), sorted.iter().filter(|&&v| v < x).count());
                    assert_eq!(wm.frequency(l..r, x), sorted.iter().filter(|&&v| v == x).count());
                    assert_eq!(wm.prev_value(l..r, x), sorted.iter().rev().find(|&&v| v < x).copied());
                    assert_eq!(wm.next_value(l..r, x), sorted.iter().find(|&&v| v >= x).copied());
                    for y in [0, max / 2, max, u64::MAX] {
                        assert_eq!(
                            wm.count_between(l..r, x, y),
                            sorted.iter().filter(|&&v| x <= v && v < y).count()
                        );
                    }
                }
                assert_eq!(wm.kth_smallest(r..l, 0), None);
                assert_eq!(wm.count_less(r..l, 5), 0);
                assert_eq!(wm.kth_smallest(l..n + 5, 0), arr[l..].iter().min().copied());
            }
        }
    }

    #[test]
    fn rectangle_sums_extremes() {
        let mut rng = Random::new_with_seed(47);
        let coords = [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX];
        for _ in 0..2000 {
            let n = rng.gen_range(0..=12usize);
            let q = rng.gen_range(0..=12usize);
            let pick = |rng: &mut Random| coords[rng.gen_range(0..coords.len())];
            let points: Vec<(i64, i64, i64)> = (0..n).map(|_| (pick(&mut rng), pick(&mut rng), rng.gen_range(-5..6i64))).collect();
            let queries: Vec<(i64, i64, i64, i64)> = (0..q).map(|_| (pick(&mut rng), pick(&mut rng), pick(&mut rng), pick(&mut rng))).collect();
            let exp: Vec<i64> = queries
                .iter()
                .map(|&(x1, y1, x2, y2)| {
                    points.iter().filter(|p| x1 <= p.0 && p.0 < x2 && y1 <= p.1 && p.1 < y2).map(|p| p.2).sum()
                })
                .collect();
            assert_eq!(rectangle_sums(&points, &queries), exp);
        }
    }

    #[test]
    fn sliding_window_non_commutative() {
        const P: u64 = 998_244_353;
        let comp = |a: (u64, u64), b: (u64, u64)| (a.0 * b.0 % P, (a.1 * b.0 + b.1) % P);
        let mut rng = Random::new_with_seed(48);
        for _ in 0..2000 {
            let size = rng.gen_range(1..=7usize);
            let mut sw = SlidingWindow::new(size, comp);
            let mut all: Vec<(u64, u64)> = Vec::new();
            for _ in 0..40 {
                let v = (rng.gen_range(1..P), rng.gen_range(0..P));
                sw.push(v);
                all.push(v);
                let from = all.len().saturating_sub(size);
                let exp = all[from..].iter().copied().reduce(comp).unwrap();
                assert_eq!(sw.get(), exp, "size {size} len {}", all.len());
            }
        }
    }

    #[test]
    fn foldable_deque_strings() {
        let mut rng = Random::new_with_seed(49);
        for _ in 0..500 {
            let mut dq = FoldableDeque::new(|a: &String, b: &String| format!("{a}{b}"));
            let mut reference: VecDeque<String> = VecDeque::new();
            for _ in 0..100 {
                let s = ((b'a' + rng.gen_range(0..26u8)) as char).to_string();
                match rng.gen_range(0..4u32) {
                    0 => {
                        dq.push_back(s.clone());
                        reference.push_back(s);
                    }
                    1 => {
                        dq.push_front(s.clone());
                        reference.push_front(s);
                    }
                    2 => assert_eq!(dq.pop_front(), reference.pop_front()),
                    _ => assert_eq!(dq.pop_back(), reference.pop_back()),
                }
                let exp: String = reference.iter().cloned().collect();
                assert_eq!(dq.fold().unwrap_or_default(), exp);
                assert_eq!(dq.front(), reference.front());
                assert_eq!(dq.back(), reference.back());
                assert_eq!(dq.len(), reference.len());
            }
        }
    }

    #[test]
    fn interval_heap_duplicates() {
        let mut rng = Random::new_with_seed(50);
        for _ in 0..3000 {
            let mut heap = IntervalHeap::new();
            let mut reference: Vec<i32> = Vec::new();
            let range = rng.gen_range(1..=4i32);
            for _ in 0..80 {
                match rng.gen_range(0..5u32) {
                    0..=2 => {
                        let v = rng.gen_range(-range..=range);
                        heap.push(v);
                        reference.push(v);
                    }
                    3 => {
                        reference.sort();
                        let exp = if reference.is_empty() { None } else { Some(reference.remove(0)) };
                        assert_eq!(heap.pop_min(), exp);
                    }
                    _ => {
                        reference.sort();
                        assert_eq!(heap.pop_max(), reference.pop());
                    }
                }
                assert_eq!(heap.min(), reference.iter().min());
                assert_eq!(heap.max(), reference.iter().max());
                assert_eq!(heap.len(), reference.len());
            }
            // drain alternately
            reference.sort();
            let mut dq: VecDeque<i32> = reference.into_iter().collect();
            let mut flip = false;
            while !dq.is_empty() {
                if flip {
                    assert_eq!(heap.pop_min(), dq.pop_front());
                } else {
                    assert_eq!(heap.pop_max(), dq.pop_back());
                }
                flip = rng.gen_bool();
            }
            assert!(heap.is_empty() && heap.pop_min().is_none() && heap.pop_max().is_none());
        }
    }

    #[test]
    fn int_set_boundaries() {
        let mut rng = Random::new_with_seed(51);
        for n in [0usize, 1, 2, 63, 64, 65, 127, 128, 129, 4095, 4096, 4097, 262143, 262144, 262145] {
            let mut set = IntSet::new(n);
            let mut reference = BTreeSet::new();
            assert_eq!(set.capacity(), n);
            for _ in 0..4000 {
                let x = match rng.gen_range(0..4u32) {
                    0 => 0,
                    1 => n.saturating_sub(1),
                    2 => (rng.gen_range(0..=n / 64) * 64 + [0, 63, 1][rng.gen_range(0..3usize)]).min(n.saturating_sub(1)),
                    _ => rng.gen_range(0..n.max(1)),
                };
                match rng.gen_range(0..4u32) {
                    0 if n > 0 => assert_eq!(set.insert(x), reference.insert(x)),
                    1 => assert_eq!(set.remove(x), reference.remove(&x)),
                    2 => {
                        assert_eq!(set.next(x), reference.range(x..).next().copied());
                        assert_eq!(set.prev(x), reference.range(..=x).next_back().copied());
                        assert_eq!(set.next(x + 1), reference.range(x + 1..).next().copied());
                        assert_eq!(set.contains(x), reference.contains(&x));
                        assert!(!set.contains(n));
                        assert!(!set.remove(n + 1));
                    }
                    _ => {
                        assert_eq!(set.min(), reference.iter().next().copied());
                        assert_eq!(set.max(), reference.iter().next_back().copied());
                        assert_eq!(set.len(), reference.len());
                    }
                }
            }
            assert_eq!(set.iter().collect::<Vec<_>>(), reference.iter().copied().collect::<Vec<_>>());
        }
    }

    #[test]
    fn bug_sparse_table_empty() {
        let _ = SparseTable::new(Vec::<i32>::new(), |a: &i32, b: &i32| *a.min(b));
    }

    #[test]
    fn bug_sparse_table_pos_empty() {
        let _ = SparseTableWithPos::new(&[] as &[i32], |a: &i32, b: &i32| if a <= b { Direction::Left } else { Direction::Right });
    }
}

#[test]
fn perf_treap_with_gen_then_ops() {
    use algo_lib::collections::payload::PurePayload;
    use algo_lib::collections::treap::treap::Tree;
    let n = 200_000;
    let mut rng = Random::new_with_seed(77);
    let mut tree: Tree<PurePayload<usize>> = Tree::with_gen(n, PurePayload);
    let mut max_depth = 0;
    for i in (0..n).step_by(997) {
        let mut at = i;
        let mut d = 0;
        tree.binary_search_size(|l, _| {
            d += 1;
            if at < l {
                Some(Direction::Left)
            } else if at == l {
                None
            } else {
                at -= l + 1;
                Some(Direction::Right)
            }
        });
        max_depth = max_depth.max(d);
    }
    let start = std::time::Instant::now();
    for _ in 0..200_000 {
        let l = rng.gen_range(0..n);
        let r = rng.gen_range(l..=n);
        tree.range_index(l..r).reverse();
    }
    let mut max_depth2 = 0;
    for i in (0..n).step_by(997) {
        let mut at = i;
        let mut d = 0;
        tree.binary_search_size(|l, _| {
            d += 1;
            if at < l {
                Some(Direction::Left)
            } else if at == l {
                None
            } else {
                at -= l + 1;
                Some(Direction::Right)
            }
        });
        max_depth2 = max_depth2.max(d);
    }
    eprintln!("depth after build {max_depth}, after ops {max_depth2}, ops took {:?}", start.elapsed());
    assert!(max_depth2 < 200);
}
