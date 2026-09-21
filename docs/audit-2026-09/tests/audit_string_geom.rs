#![allow(clippy::all)]
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::string::aho_corasick::{ACPayload, AhoCorasickLowercase};
use algo_lib::string::concat::StrConcat;
use algo_lib::string::hash::{compare, CompositeHash, Hashable, SimpleHash, StringHash};
use algo_lib::string::palindromic_tree::PalindromicTree;
use algo_lib::string::split::StrSplit;
use algo_lib::string::str::{Str, StrReader};
use algo_lib::string::string_algorithms::lyndon::lyndon_factorization;
use algo_lib::string::string_algorithms::palindromes::Palindromes;
use algo_lib::string::string_algorithms::prefix_function::PrefixFunction;
use algo_lib::string::string_algorithms::runs::runs;
use algo_lib::string::string_algorithms::string_search::StringSearch;
use algo_lib::string::string_algorithms::wildcard_matching::wildcard_matches;
use algo_lib::string::string_algorithms::z_algorithm::ZAlgorithm;
use algo_lib::string::suffix_array::SuffixArray;
use algo_lib::string::suffix_automaton::{SuffixAutomatonHash, SuffixAutomatonTree};
use algo_lib::string::trim::StrTrim;
use algo_lib::io::input::Input;
use std::collections::BTreeSet;

fn gen_str(rng: &mut Random, n: usize, alphabet: u8, base: u8) -> Vec<u8> {
    (0..n).map(|_| base + rng.gen_bound(alphabet)).collect()
}

fn naive_lcp<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    a.iter().zip(b.iter()).take_while(|(x, y)| x == y).count()
}

// ---------- suffix array ----------

fn check_sa<T: Ord + Copy + std::fmt::Debug + algo_lib::numbers::num_traits::algebra::Zero>(
    s: &[T],
    rng: &mut Random,
) {
    let n = s.len();
    let sa = SuffixArray::new(s);
    assert_eq!(sa.len(), n + 1);
    let mut expected: Vec<usize> = (0..n).collect();
    expected.sort_by(|&a, &b| s[a..].cmp(&s[b..]));
    expected.insert(0, n);
    let actual: Vec<usize> = (&sa).into_iter().collect();
    assert_eq!(actual, expected, "sa of {:?}", s);
    for i in 0..=n {
        assert_eq!(sa.get_pos_in_array(sa[i]), i);
    }
    for i in 0..=n {
        for j in 0..=n {
            let exp = naive_lcp(&s[sa[i]..], &s[sa[j]..]);
            assert_eq!(sa.lcp(i, j), exp, "lcp({i},{j}) of {:?}", s);
        }
    }
    for _ in 0..20 {
        let l1 = rng.gen_range(0..=n);
        let r1 = rng.gen_range(l1..=n);
        let l2 = rng.gen_range(0..=n);
        let r2 = rng.gen_range(l2..=n);
        assert_eq!(sa.cmp(l1..r1, l2..r2), s[l1..r1].cmp(&s[l2..r2]), "{:?} {l1} {r1} {l2} {r2}", s);
    }
    // find
    for _ in 0..20 {
        let t: Vec<T> = if n > 0 && rng.gen_bool() {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..=n);
            s[l..r].to_vec()
        } else {
            let len = rng.gen_range(0..4usize);
            (0..len).map(|_| if n > 0 { s[rng.gen_range(0..n)] } else { T::zero() }).collect()
        };
        let (from, to) = sa.find(&t);
        let got: BTreeSet<usize> = (from..to).map(|i| sa[i]).collect();
        let exp: BTreeSet<usize> = (0..n).filter(|&i| s[i..].starts_with(&t)).collect();
        if t.is_empty() {
            continue;
        }
        assert_eq!(got, exp, "find {:?} in {:?}", t, s);
    }
}

#[test]
fn suffix_array_positive_bytes() {
    let mut rng = Random::new_with_seed(1);
    for iter in 0..3000 {
        let n = rng.gen_range(0..=30usize);
        let alphabet = [1u8, 2, 3, 26][iter % 4];
        let s = gen_str(&mut rng, n, alphabet, if iter % 8 < 4 { 1 } else { 229 });
        check_sa(&s, &mut rng);
    }
}

#[test]
fn suffix_array_with_zero_bytes() {
    let mut rng = Random::new_with_seed(2);
    for iter in 0..3000 {
        let n = rng.gen_range(0..=12usize);
        let alphabet = [1u8, 2, 3][iter % 3];
        let s = gen_str(&mut rng, n, alphabet, 0);
        check_sa(&s, &mut rng);
    }
}

#[test]
fn suffix_array_negative_values() {
    let mut rng = Random::new_with_seed(3);
    for _ in 0..3000 {
        let n = rng.gen_range(0..=12usize);
        let s: Vec<i32> = (0..n).map(|_| rng.gen_range(-2..=-1i32)).collect();
        check_sa(&s, &mut rng);
    }
}

fn all_strings<T: Copy>(alphabet: &[T], max_len: usize) -> Vec<Vec<T>> {
    let mut res = vec![vec![]];
    let mut last = vec![vec![]];
    for _ in 0..max_len {
        let mut next = Vec::new();
        for s in &last {
            for &c in alphabet {
                let mut t: Vec<T> = s.clone();
                t.push(c);
                next.push(t);
            }
        }
        res.extend(next.iter().cloned());
        last = next;
    }
    res
}

#[test]
fn suffix_array_minimize() {
    for (name, alphabet) in [("zero", vec![0i32, 1]), ("neg", vec![-2i32, -1])] {
        let strs = all_strings(&alphabet, 4);
        let mut lcp_found = false;
        let mut find_found = false;
        for s in &strs {
            let n = s.len();
            let sa = SuffixArray::new(s);
            if !lcp_found {
                for i in 0..=n {
                    for j in 0..=n {
                        let exp = naive_lcp(&s[sa[i]..], &s[sa[j]..]);
                        if sa.lcp(i, j) != exp && !lcp_found {
                            lcp_found = true;
                            println!("{name}: lcp s={:?} i={i} j={j} got={} exp={exp}", s, sa.lcp(i, j));
                        }
                    }
                }
            }
            if !find_found {
                for t in &strs {
                    if t.is_empty() || t.len() > 2 {
                        continue;
                    }
                    let (from, to) = sa.find(t);
                    let got: BTreeSet<usize> = (from..to).map(|i| sa[i]).collect();
                    let exp: BTreeSet<usize> = (0..n).filter(|&i| s[i..].starts_with(t)).collect();
                    if got != exp && !find_found {
                        find_found = true;
                        println!("{name}: find s={:?} t={:?} got={:?} exp={:?}", s, t, got, exp);
                    }
                }
            }
        }
    }
}

// ---------- suffix automaton ----------

#[test]
fn suffix_automaton_stress() {
    let mut rng = Random::new_with_seed(10);
    for iter in 0..2000 {
        let n = rng.gen_range(0..=25usize);
        let alphabet = [1u8, 2, 3, 26][iter % 4];
        let s = gen_str(&mut rng, n, alphabet, b'a');
        let mut subs: BTreeSet<&[u8]> = BTreeSet::new();
        for l in 0..n {
            for r in l + 1..=n {
                subs.insert(&s[l..r]);
            }
        }
        let t = SuffixAutomatonTree::new(&s);
        let h = SuffixAutomatonHash::new(&s);
        assert_eq!(t.distinct_substrings(), subs.len());
        assert_eq!(h.distinct_substrings(), subs.len());
        assert!(t.state_count() <= (2 * n).max(2));
        // occurrence counts via terminal-propagation: count paths to terminal states
        let term = t.terminal();
        let cnt_states = t.state_count();
        let mut order: Vec<usize> = (0..cnt_states).collect();
        order.sort_by_key(|&v| std::cmp::Reverse(t.state_len(v)));
        let mut occ = vec![0usize; cnt_states];
        for &v in &order {
            let mut c = if term[v] { 1 } else { 0 };
            for (_, to) in t.state_edges(v) {
                c += occ[to];
            }
            occ[v] = c;
        }
        for _ in 0..30 {
            let len = rng.gen_range(0..=4usize);
            let p = if n > 0 && rng.gen_bool() {
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..=n);
                s[l..r].to_vec()
            } else {
                gen_str(&mut rng, len, alphabet.min(3) + 1, b'a')
            };
            let exp_occ = if p.len() > n { 0 } else { (0..=n - p.len()).filter(|&i| s[i..].starts_with(&p)).count() };
            assert_eq!(t.contains(&p), exp_occ > 0);
            assert_eq!(h.contains(&p), exp_occ > 0);
            let mut v = 0;
            let mut ok = true;
            for &c in &p {
                match t.state_next(v, c) {
                    Some(to) => v = to,
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok && !p.is_empty() {
                assert_eq!(occ[v], exp_occ, "{:?} {:?}", s, p);
                // link: len strictly smaller
                let l = t.state_link(v).unwrap();
                assert!(t.state_len(l) < t.state_len(v));
                assert!(t.state_len(v) as usize >= p.len());
                assert!((t.state_len(l) as usize) < p.len());
            }
        }
    }
}

// ---------- aho corasick ----------

#[derive(Default, Debug, Clone)]
struct Ids(Vec<usize>);
impl ACPayload for Ids {
    fn add_single(&mut self, id: usize) {
        self.0.push(id);
    }
    fn add_node(&mut self, other: &Self) {
        self.0.extend_from_slice(&other.0);
    }
}

fn aho_check(patterns: &[Vec<u8>], text: &Vec<u8>) {
    let ac = AhoCorasickLowercase::<Ids>::new(patterns);
    let got: Vec<Vec<usize>> = ac
        .iterate(text)
        .map(|p| {
            let mut v = p.0.clone();
            v.sort();
            v
        })
        .collect();
    assert_eq!(got.len(), text.len() + 1);
    for pos in 0..=text.len() {
        let exp: Vec<usize> = (0..patterns.len())
            .filter(|&i| text[..pos].ends_with(&patterns[i]))
            .collect();
        assert_eq!(got[pos], exp, "patterns {:?} text {:?} pos {}", patterns, text, pos);
    }
}

#[test]
fn aho_corasick_nonempty_patterns() {
    let mut rng = Random::new_with_seed(20);
    for iter in 0..5000 {
        let k = rng.gen_range(0..=6usize);
        let alphabet = [1u8, 2, 3][iter % 3];
        let patterns: Vec<Vec<u8>> = (0..k)
            .map(|_| {
                let len = rng.gen_range(1..=5usize);
                gen_str(&mut rng, len, alphabet, b'a')
            })
            .collect();
        let n = rng.gen_range(0..=20usize);
        let text = gen_str(&mut rng, n, alphabet, b'a');
        aho_check(&patterns, &text);
    }
}

#[test]
fn aho_corasick_with_empty_pattern() {
    let mut rng = Random::new_with_seed(21);
    for iter in 0..2000 {
        let k = rng.gen_range(1..=4usize);
        let alphabet = [1u8, 2, 3][iter % 3];
        let patterns: Vec<Vec<u8>> = (0..k)
            .map(|_| {
                let len = rng.gen_range(0..=3usize);
                gen_str(&mut rng, len, alphabet, b'a')
            })
            .collect();
        let n = rng.gen_range(0..=6usize);
        let text = gen_str(&mut rng, n, alphabet, b'a');
        aho_check(&patterns, &text);
    }
}

// ---------- palindromic tree ----------

#[test]
fn palindromic_tree_stress() {
    let mut rng = Random::new_with_seed(30);
    for iter in 0..2000 {
        let n = rng.gen_range(0..=30usize);
        let alphabet = [1u8, 2, 3, 26][iter % 4];
        let base = if iter % 2 == 0 { 0 } else { 230 };
        let s = gen_str(&mut rng, n, alphabet, base);
        let mut tree = PalindromicTree::new();
        let mut naive: std::collections::BTreeMap<Vec<u8>, u64> = Default::default();
        for i in 0..n {
            let node = tree.push(s[i]);
            let longest = (0..=i).find(|&l| s[l..=i].iter().eq(s[l..=i].iter().rev())).unwrap();
            assert_eq!(tree.len(node), i + 1 - longest);
            // link = longest proper palindromic suffix
            let lp = (longest + 1..=i).find(|&l| s[l..=i].iter().eq(s[l..=i].iter().rev()));
            let lp_len = lp.map(|l| i + 1 - l).unwrap_or(0);
            assert_eq!(tree.len(tree.link(node)), lp_len);
        }
        for l in 0..n {
            for r in l + 1..=n {
                let w = &s[l..r];
                if w.iter().eq(w.iter().rev()) {
                    *naive.entry(w.to_vec()).or_default() += 1;
                }
            }
        }
        assert_eq!(tree.distinct_palindromes(), naive.len());
        let occ = tree.occurrences();
        let mut got: Vec<(usize, u64)> = (2..tree.node_count()).map(|v| (tree.len(v), occ[v])).collect();
        got.sort();
        let mut exp: Vec<(usize, u64)> = naive.iter().map(|(w, &c)| (w.len(), c)).collect();
        exp.sort();
        assert_eq!(got, exp);
        let t2 = PalindromicTree::from_slice(&s);
        assert_eq!(t2.node_count(), tree.node_count());
    }
}

// ---------- hash ----------

#[test]
fn hash_stress() {
    let mut rng = Random::new_with_seed(40);
    for iter in 0..1500 {
        let n = rng.gen_range(0..=20usize);
        let m = rng.gen_range(0..=20usize);
        let alphabet = [1u8, 2, 3][iter % 3];
        let a = gen_str(&mut rng, n, alphabet, b'a');
        let b = gen_str(&mut rng, m, alphabet, b'a');
        let ha = SimpleHash::new(&a);
        let mut hb = SimpleHash::new(&b[..m / 2]);
        for &c in &b[m / 2..] {
            hb.push(c);
        }
        assert_eq!(ha.len(), n);
        assert_eq!(hb.len(), m);
        let ab: Vec<u8> = a.str_concat(&b).unwrap();
        assert_eq!(ab.len(), n + m);
        let comp = CompositeHash::new(&ha, &hb);
        assert_eq!(comp.len(), n + m);
        let hab = SimpleHash::new(&ab);
        for l in 0..=n + m {
            for r in l..=n + m {
                assert_eq!(comp.hash(l..r), hab.hash(l..r));
                assert_eq!(hab.hash(l..r), ab[l..r].str_hash());
                if r > l {
                    assert_eq!(hab.hash(l..=r - 1), ab[l..r].str_hash());
                }
                let sub = comp.sub_hash(l..r);
                assert_eq!(sub.len(), r - l);
                assert_eq!(sub.hash(..), ab[l..r].str_hash());
                if r - l >= 2 {
                    assert_eq!(sub.hash(1..r - l - 1), ab[l + 1..r - 1].str_hash());
                }
            }
        }
        // equality semantic
        for _ in 0..30 {
            let l1 = rng.gen_range(0..=n);
            let r1 = rng.gen_range(l1..=n);
            let l2 = rng.gen_range(0..=m);
            let r2 = rng.gen_range(l2..=m);
            assert_eq!(ha.hash(l1..r1) == hb.hash(l2..r2), a[l1..r1] == b[l2..r2]);
            let s1 = ha.sub_hash(l1..r1);
            let s2 = hb.sub_hash(l2..r2);
            assert_eq!(compare(&s1, &s2), a[l1..r1].cmp(&b[l2..r2]));
            let c1 = CompositeHash::new(&s1, &s2);
            let c2 = CompositeHash::new(&s2, &s1);
            let v1 = [&a[l1..r1], &b[l2..r2]].concat();
            let v2 = [&b[l2..r2], &a[l1..r1]].concat();
            assert_eq!(compare(&c1, &c2), v1.cmp(&v2));
        }
    }
}

// ---------- z / prefix / palindromes / search / lyndon / runs / wildcard ----------

#[test]
fn z_prefix_manacher_search() {
    let mut rng = Random::new_with_seed(50);
    for iter in 0..5000 {
        let n = rng.gen_range(1..=30usize);
        let alphabet = [1u8, 2, 3, 26][iter % 4];
        let s = gen_str(&mut rng, n, alphabet, b'a');
        let z = s.z_algorithm();
        let pf = s.prefix_function();
        let odd = s.odd_palindromes();
        let even = s.even_palindromes();
        assert_eq!(z.len(), n);
        assert_eq!(pf.len(), n);
        for i in 0..n {
            if i > 0 {
                assert_eq!(z[i], naive_lcp(&s, &s[i..]));
            } else {
                assert_eq!(z[0], 0);
            }
            let exp_pf = (0..=i).rev().find(|&k| s[..k] == s[i + 1 - k..=i]).unwrap();
            assert_eq!(pf[i], exp_pf);
            let mut k = 0;
            while i >= k && i + k < n && s[i - k] == s[i + k] {
                k += 1;
            }
            assert_eq!(odd[i], k, "{:?}", s);
            let mut k = 0;
            while i >= k + 1 && i + k < n && s[i - k - 1] == s[i + k] {
                k += 1;
            }
            assert_eq!(even[i], k, "{:?}", s);
        }
        let m = rng.gen_range(1..=4usize);
        let p = gen_str(&mut rng, m, alphabet, b'a');
        let exp: Vec<usize> = if m > n { vec![] } else { (0..=n - m).filter(|&i| s[i..].starts_with(&p)).collect() };
        assert_eq!(s.all_matches(&p), exp);
        assert_eq!(s.index_of(&p), exp.first().copied());
        assert_eq!(s.str_contains(&p), !exp.is_empty());
    }
}

#[test]
fn z_prefix_empty() {
    let e: &[u8] = b"";
    assert_eq!(e.odd_palindromes(), Vec::<usize>::new());
    assert_eq!(e.even_palindromes(), Vec::<usize>::new());
    assert_eq!(e.index_of(b"a"), None);
    assert_eq!(e.all_matches(b"a"), Vec::<usize>::new());
    assert_eq!(e.z_algorithm(), Vec::<usize>::new());
    assert_eq!(e.prefix_function(), Vec::<usize>::new());
}

fn is_lyndon(w: &[u8]) -> bool {
    (1..w.len()).all(|i| w < &w[i..])
}

#[test]
fn lyndon_stress() {
    let mut rng = Random::new_with_seed(60);
    for iter in 0..5000 {
        let n = rng.gen_range(0..=25usize);
        let alphabet = [1u8, 2, 3, 26][iter % 4];
        let s = gen_str(&mut rng, n, alphabet, 0);
        let cuts = lyndon_factorization(&s);
        assert_eq!(cuts[0], 0);
        assert_eq!(*cuts.last().unwrap(), n);
        let f: Vec<&[u8]> = cuts.windows(2).map(|w| &s[w[0]..w[1]]).collect();
        if n == 0 {
            assert_eq!(cuts, vec![0]);
        }
        assert!(f.iter().all(|w| !w.is_empty() && is_lyndon(w)));
        assert!(f.windows(2).all(|p| p[0] >= p[1]));
    }
}

fn minimal_period(w: &[u8]) -> usize {
    (1..=w.len()).find(|&p| (p..w.len()).all(|i| w[i] == w[i - p])).unwrap()
}

fn naive_runs(s: &[u8]) -> Vec<(usize, usize, usize)> {
    let n = s.len();
    let mut res = Vec::new();
    for from in 0..n {
        for to in from + 2..=n {
            let p = minimal_period(&s[from..to]);
            if 2 * p > to - from {
                continue;
            }
            if from > 0 && s[from - 1] == s[from - 1 + p] {
                continue;
            }
            if to < n && s[to] == s[to - p] {
                continue;
            }
            res.push((p, from, to));
        }
    }
    res.sort();
    res
}

#[test]
fn runs_stress() {
    let mut rng = Random::new_with_seed(70);
    for iter in 0..4000 {
        let n = rng.gen_range(0..=30usize);
        let alphabet = [1u8, 2, 3, 4][iter % 4];
        let base = if iter % 2 == 0 { 1 } else { 250 };
        let s = gen_str(&mut rng, n, alphabet, base);
        assert_eq!(runs(&s), naive_runs(&s), "{:?}", s);
    }
}

#[test]
fn wildcard_stress() {
    let mut rng = Random::new_with_seed(80);
    for iter in 0..1500 {
        let n = rng.gen_range(0..=40usize);
        let m = rng.gen_range(0..=(n + 2).min(8));
        let wildcard = [b'*', 0u8, 255u8][iter % 3];
        let letters: Vec<u8> = vec![wildcard, 1, 254, b'a'];
        let mut gen = |len: usize, rng: &mut Random| -> Vec<u8> {
            (0..len).map(|_| letters[rng.gen_range(0..if iter % 2 == 0 { 3usize } else { 4 })]).collect()
        };
        let text = gen(n, &mut rng);
        let pattern = gen(m, &mut rng);
        let exp: Vec<bool> = if m > n {
            vec![]
        } else {
            (0..=n - m)
                .map(|i| (0..m).all(|j| text[i + j] == wildcard || pattern[j] == wildcard || text[i + j] == pattern[j]))
                .collect()
        };
        assert_eq!(wildcard_matches(&text, &pattern, wildcard), exp);
    }
}

// ---------- Str & co ----------

#[test]
fn split_overlapping_pattern() {
    let s: &[u8] = b"aaa";
    let parts = s.str_split(b"aa");
    assert_eq!(parts, vec![&b""[..], &b"a"[..]]);
}

#[test]
fn split_stress() {
    let mut rng = Random::new_with_seed(90);
    for _ in 0..3000 {
        let n = rng.gen_range(0..=12usize);
        let s = gen_str(&mut rng, n, 2, b'a');
        let m = rng.gen_range(1..=3usize);
        let p = gen_str(&mut rng, m, 2, b'a');
        let ss = String::from_utf8(s.clone()).unwrap();
        let ps = String::from_utf8(p.clone()).unwrap();
        let exp: Vec<Vec<u8>> = ss.split(ps.as_str()).map(|x| x.as_bytes().to_vec()).collect();
        let r = std::panic::catch_unwind(|| s.str_split(&p).into_iter().map(|x| x.to_vec()).collect::<Vec<_>>());
        match r {
            Ok(got) => assert_eq!(got, exp, "{ss} {ps}"),
            Err(_) => panic!("panic on split {ss:?} by {ps:?}"),
        }
    }
}

#[test]
fn trim_sanity() {
    assert_eq!(b"  a b \n".trim(), b"a b");
    assert_eq!(b"   ".trim(), b"");
    assert_eq!(b"".trim(), b"");
    assert_eq!(b"x".trim(), b"x");
}

#[test]
fn str_sanity() {
    let mut s = Str::from(b"hello");
    s += b" world";
    assert_eq!(s.len(), 11);
    s.reverse();
    assert_eq!(s.as_slice(), b"dlrow olleh");
    s.sort();
    assert_eq!(&s[..1], b" ");
    let t: Str = s.iter().copied().filter(|&c| c != b' ').collect();
    assert_eq!(t.len(), 10);
    assert!(Str::from(b"ab") < Str::from(b"b"));
    assert_eq!(format!("{}", Str::from(b"xyz")), "xyz");
    assert_eq!(Str::new().unwrap(), Vec::<u8>::new());
}

#[test]
fn read_line_without_trailing_newline() {
    let data = b"abc\ndef";
    let mut input = Input::slice(data);
    assert_eq!(input.read_line().as_slice(), b"abc");
    assert_eq!(input.read_line().as_slice(), b"def");
}

#[test]
fn read_lines_variants() {
    let mut input = Input::slice(b"abc\n\ndef\n");
    let lines = input.read_lines();
    let lines: Vec<&[u8]> = lines.iter().map(|s| s.as_slice()).collect();
    assert_eq!(lines, vec![&b"abc"[..], &b""[..], &b"def"[..]]);
    let mut input = Input::slice(b"abc\r\nde\r\n");
    let lines = input.read_lines();
    let lines: Vec<&[u8]> = lines.iter().map(|s| s.as_slice()).collect();
    assert_eq!(lines, vec![&b"abc"[..], &b"de"[..]]);
}

// =====================================================================
// GEOMETRY
// =====================================================================
mod geom {
    use algo_lib::geometry::angle::Angle;
    use algo_lib::geometry::arg_sort::{arg_cmp, sort_by_argument};
    use algo_lib::geometry::circle::Circle;
    use algo_lib::geometry::geometry_utils::{canonize_angle, canonize_angle_base};
    use algo_lib::geometry::line::Line;
    use algo_lib::geometry::manhattan_mst::manhattan_mst;
    use algo_lib::geometry::min_enclosing_circle::min_enclosing_circle;
    use algo_lib::geometry::point::Point;
    use algo_lib::geometry::point_pairs::{closest_pair, farthest_pair};
    use algo_lib::geometry::polygon::{ConvexHull, Polygon};
    use algo_lib::geometry::ray::Ray;
    use algo_lib::geometry::segment::{Segment, SegmentIntersectionResult};
    use algo_lib::misc::random::{Random, RandomTrait};
    use algo_lib::numbers::rational::Rational;
    use algo_lib::numbers::real::{IntoReal, Real};
    use std::cmp::Ordering;
    use std::collections::BTreeSet;

    type Q = Rational<i128>;
    fn q(x: i64) -> Q {
        Q::new_int(x as i128)
    }
    fn qp(p: (i64, i64)) -> Point<Q> {
        Point::new(q(p.0), q(p.1))
    }
    fn rp(p: (i64, i64)) -> Point<Real> {
        Point::new(Real(p.0 as f64), Real(p.1 as f64))
    }
    fn ip(p: (i64, i64)) -> Point<i64> {
        Point::new(p.0, p.1)
    }
    fn gen_pt(rng: &mut Random, r: i64) -> (i64, i64) {
        (rng.gen_range(-r..=r), rng.gen_range(-r..=r))
    }
    fn cross(o: (i64, i64), a: (i64, i64), b: (i64, i64)) -> i64 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    }
    fn on_seg(a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> bool {
        cross(a, b, p) == 0
            && p.0 >= a.0.min(b.0)
            && p.0 <= a.0.max(b.0)
            && p.1 >= a.1.min(b.1)
            && p.1 <= a.1.max(b.1)
    }
    fn qf(x: Q) -> f64 {
        x.num() as f64 / x.den() as f64
    }

    // point-segment distance squared in f64
    fn ps_dist2(a: (f64, f64), b: (f64, f64), p: (f64, f64)) -> f64 {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l2 = dx * dx + dy * dy;
        let t = if l2 == 0.0 { 0.0 } else { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0) };
        let (x, y) = (a.0 + t * dx, a.1 + t * dy);
        (p.0 - x) * (p.0 - x) + (p.1 - y) * (p.1 - y)
    }
    fn f(p: (i64, i64)) -> (f64, f64) {
        (p.0 as f64, p.1 as f64)
    }

    #[derive(Debug, PartialEq)]
    enum Exp {
        None,
        Point,
        Segment((i64, i64), (i64, i64)),
    }

    fn brute_intersection(a: (i64, i64), b: (i64, i64), c: (i64, i64), d: (i64, i64)) -> Exp {
        // returns classification; for Point case the point is checked separately
        if a == b {
            return if on_seg(c, d, a) { Exp::Point } else { Exp::None };
        }
        if c == d {
            return if on_seg(a, b, c) { Exp::Point } else { Exp::None };
        }
        let dir_cross = (b.0 - a.0) * (d.1 - c.1) - (b.1 - a.1) * (d.0 - c.0);
        if dir_cross == 0 {
            if cross(a, b, c) != 0 {
                return Exp::None;
            }
            let (a, b) = (a.min(b), a.max(b));
            let (c, d) = (c.min(d), c.max(d));
            let lo = a.max(c);
            let hi = b.min(d);
            return match lo.cmp(&hi) {
                Ordering::Less => Exp::Segment(lo, hi),
                Ordering::Equal => Exp::Point,
                Ordering::Greater => Exp::None,
            };
        }
        let o1 = cross(a, b, c).signum();
        let o2 = cross(a, b, d).signum();
        let o3 = cross(c, d, a).signum();
        let o4 = cross(c, d, b).signum();
        if o1 * o2 <= 0 && o3 * o4 <= 0 {
            Exp::Point
        } else {
            Exp::None
        }
    }

    #[test]
    fn segment_intersection_rational_and_real() {
        let mut rng = Random::new_with_seed(100);
        for iter in 0..200000 {
            let r = [1i64, 2, 3, 10][iter % 4];
            let (a, b, c, d) = (gen_pt(&mut rng, r), gen_pt(&mut rng, r), gen_pt(&mut rng, r), gen_pt(&mut rng, r));
            let exp = brute_intersection(a, b, c, d);
            let s1 = Segment::new(qp(a), qp(b));
            let s2 = Segment::new(qp(c), qp(d));
            match s1.intersect_segment(s2) {
                SegmentIntersectionResult::None => assert_eq!(exp, Exp::None, "{a:?} {b:?} {c:?} {d:?}"),
                SegmentIntersectionResult::Point(p) => {
                    assert_eq!(exp, Exp::Point, "{a:?} {b:?} {c:?} {d:?}");
                    assert!(s1.contains(p) && s2.contains(p), "{a:?} {b:?} {c:?} {d:?}");
                }
                SegmentIntersectionResult::Segment(s) => {
                    let (lo, hi) = (s.p1.min(s.p2), s.p1.max(s.p2));
                    match exp {
                        Exp::Segment(elo, ehi) => {
                            assert!(lo == qp(elo) && hi == qp(ehi), "{a:?} {b:?} {c:?} {d:?}");
                        }
                        _ => panic!("{a:?} {b:?} {c:?} {d:?} expected {exp:?}"),
                    }
                }
            }
            // Real variant
            let t1 = Segment::new(rp(a), rp(b));
            let t2 = Segment::new(rp(c), rp(d));
            match t1.intersect_segment(t2) {
                SegmentIntersectionResult::None => assert_eq!(exp, Exp::None, "real {a:?} {b:?} {c:?} {d:?}"),
                SegmentIntersectionResult::Point(_) => assert_eq!(exp, Exp::Point, "real {a:?} {b:?} {c:?} {d:?}"),
                SegmentIntersectionResult::Segment(_) => {
                    assert!(matches!(exp, Exp::Segment(..)), "real {a:?} {b:?} {c:?} {d:?}")
                }
            }
            // distances
            let p = gen_pt(&mut rng, r);
            let d2 = qf(s1.square_dist_point(qp(p)));
            assert!((d2 - ps_dist2(f(a), f(b), f(p))).abs() < 1e-9, "dist {a:?} {b:?} {p:?}");
            let d2r = t1.square_dist_point(rp(p)).0;
            assert!((d2r - ps_dist2(f(a), f(b), f(p))).abs() < 1e-6, "real dist {a:?} {b:?} {p:?}");
            let expd = if exp != Exp::None {
                0.0
            } else {
                ps_dist2(f(a), f(b), f(c))
                    .min(ps_dist2(f(a), f(b), f(d)))
                    .min(ps_dist2(f(c), f(d), f(a)))
                    .min(ps_dist2(f(c), f(d), f(b)))
            };
            let got = qf(s1.square_dist_segment(s2));
            assert!((got - expd).abs() < 1e-9, "segdist {a:?} {b:?} {c:?} {d:?} {got} {expd}");
            let gotr = t1.square_dist_segment(t2).0;
            assert!((gotr - expd).abs() < 1e-6, "real segdist {a:?} {b:?} {c:?} {d:?} {gotr} {expd}");
            // integer contains
            let si = Segment::new(ip(a), ip(b));
            assert_eq!(si.contains(ip(p)), on_seg(a, b, p));
            assert_eq!(si.contains(ip(c)), on_seg(a, b, c));
        }
    }

    // ---------- ray ----------
    fn ray_contains(o: (i64, i64), d: (i64, i64), p: (i64, i64)) -> bool {
        cross(o, d, p) == 0 && (d.0 - o.0) * (p.0 - o.0) + (d.1 - o.1) * (p.1 - o.1) >= 0
    }
    fn pr_dist2(o: (f64, f64), d: (f64, f64), p: (f64, f64)) -> f64 {
        let (dx, dy) = (d.0 - o.0, d.1 - o.1);
        let t = (((p.0 - o.0) * dx + (p.1 - o.1) * dy) / (dx * dx + dy * dy)).max(0.0);
        let (x, y) = (o.0 + t * dx, o.1 + t * dy);
        (p.0 - x) * (p.0 - x) + (p.1 - y) * (p.1 - y)
    }

    #[test]
    fn ray_stress() {
        let mut rng = Random::new_with_seed(110);
        for iter in 0..100000 {
            let r = [1i64, 2, 4][iter % 3];
            let (o1, d1, o2, d2) = (gen_pt(&mut rng, r), gen_pt(&mut rng, r), gen_pt(&mut rng, r), gen_pt(&mut rng, r));
            if o1 == d1 || o2 == d2 {
                continue;
            }
            let p = gen_pt(&mut rng, r + 2);
            let r1 = Ray::new(qp(o1), qp(d1));
            let r2 = Ray::new(qp(o2), qp(d2));
            assert_eq!(Ray::new(ip(o1), ip(d1)).contains(ip(p)), ray_contains(o1, d1, p));
            let got = qf(r1.square_dist_point(qp(p)));
            assert!((got - pr_dist2(f(o1), f(d1), f(p))).abs() < 1e-9);
            // ray-ray distance via ternary search
            let (dx, dy) = ((d1.0 - o1.0) as f64, (d1.1 - o1.1) as f64);
            let g = |t: f64| pr_dist2(f(o2), f(d2), (o1.0 as f64 + t * dx, o1.1 as f64 + t * dy));
            let (mut lo, mut hi) = (0.0f64, 1e3);
            for _ in 0..200 {
                let m1 = lo + (hi - lo) / 3.0;
                let m2 = hi - (hi - lo) / 3.0;
                if g(m1) < g(m2) {
                    hi = m2;
                } else {
                    lo = m1;
                }
            }
            let exp = g(lo).min(g(0.0));
            let got = qf(r1.square_dist_ray(r2));
            assert!((got - exp).abs() < 1e-5, "{o1:?} {d1:?} {o2:?} {d2:?} got {got} exp {exp}");
            if let Some(pt) = r1.intersect_ray(r2) {
                assert!(r1.contains(pt) && r2.contains(pt));
            }
        }
    }

    // ---------- circle ----------
    #[test]
    fn circle_line_noncanonical() {
        let c = Circle::new(rp((0, 0)), Real(1.0));
        let l = rp((-2, 0)).line(rp((2, 0)));
        let pts = c.intersect_line(l);
        assert_eq!(pts.len(), 2);
        for p in pts {
            assert!((p.x.0.abs() - 1.0).abs() < 1e-9 && p.y.0.abs() < 1e-9, "got ({}, {})", p.x.0, p.y.0);
        }
    }

    #[test]
    fn circle_line_stress() {
        let mut rng = Random::new_with_seed(120);
        for canonical in [true, false] {
            for _ in 0..50000 {
                let c = gen_pt(&mut rng, 5);
                let rad = rng.gen_range(1..=5i64);
                let (a, b) = (gen_pt(&mut rng, 6), gen_pt(&mut rng, 6));
                if a == b {
                    continue;
                }
                let mut l = rp(a).line(rp(b));
                if canonical {
                    l = Line::new_canonical(l.a, l.b, l.c);
                }
                let circle = Circle::new(rp(c), Real(rad as f64));
                let pts = circle.intersect_line(l);
                // expected count: compare cross^2 with r^2 * len^2
                let cr = cross(a, b, c);
                let len2 = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2);
                let exp = match (cr * cr).cmp(&(rad * rad * len2)) {
                    Ordering::Less => 2,
                    Ordering::Equal => 1,
                    Ordering::Greater => 0,
                };
                assert_eq!(pts.len(), exp, "canonical={canonical} {c:?} {rad} {a:?} {b:?}");
                for p in &pts {
                    let d = ((p.x.0 - c.0 as f64).powi(2) + (p.y.0 - c.1 as f64).powi(2)).sqrt();
                    assert!((d - rad as f64).abs() < 1e-6, "canonical={canonical} {c:?} {rad} {a:?} {b:?}: not on circle");
                    assert!(l.value(*p).0.abs() / (len2 as f64).sqrt().max(1.0) < 1e-6 || canonical && l.value(*p).0.abs() < 1e-6);
                }
                if pts.len() == 2 {
                    assert!((pts[0].x.0 - pts[1].x.0).abs() + (pts[0].y.0 - pts[1].y.0).abs() > 1e-6);
                }
            }
        }
    }

    #[test]
    fn circle_circle_and_tangents() {
        let mut rng = Random::new_with_seed(130);
        for _ in 0..100000 {
            let c1 = gen_pt(&mut rng, 5);
            let c2 = gen_pt(&mut rng, 5);
            let r1 = rng.gen_range(1..=6i64);
            let r2 = rng.gen_range(1..=6i64);
            let a = Circle::new(rp(c1), Real(r1 as f64));
            let b = Circle::new(rp(c2), Real(r2 as f64));
            let pts = a.intersect_circle(b);
            let d2 = (c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2);
            let exp = if d2 == 0 {
                0
            } else if d2 > (r1 + r2).pow(2) || d2 < (r1 - r2).pow(2) {
                0
            } else if d2 == (r1 + r2).pow(2) || d2 == (r1 - r2).pow(2) {
                1
            } else {
                2
            };
            assert_eq!(pts.len(), exp, "{c1:?} {r1} {c2:?} {r2}");
            for p in &pts {
                for (c, r) in [(c1, r1), (c2, r2)] {
                    let d = ((p.x.0 - c.0 as f64).powi(2) + (p.y.0 - c.1 as f64).powi(2)).sqrt();
                    assert!((d - r as f64).abs() < 1e-6, "{c1:?} {r1} {c2:?} {r2}");
                }
            }
            // tangents from point c2 to circle a
            let t = a.tangent_points(rp(c2));
            let exp = match d2.cmp(&(r1 * r1)) {
                Ordering::Less => 0,
                Ordering::Equal => 1,
                Ordering::Greater => 2,
            };
            assert_eq!(t.len(), exp, "tangent {c1:?} {r1} {c2:?}");
            for p in &t {
                let (vx, vy) = (p.x.0 - c1.0 as f64, p.y.0 - c1.1 as f64);
                assert!(((vx * vx + vy * vy).sqrt() - r1 as f64).abs() < 1e-6);
                let (wx, wy) = (p.x.0 - c2.0 as f64, p.y.0 - c2.1 as f64);
                assert!((vx * wx + vy * wy).abs() < 1e-6, "tangent {c1:?} {r1} {c2:?}");
            }
            if t.len() == 2 {
                assert!((t[0].x.0 - t[1].x.0).abs() + (t[0].y.0 - t[1].y.0).abs() > 1e-6);
            }
            assert_eq!(Circle::new(ip(c1), r1).contains(ip(c2)), d2 <= r1 * r1);
        }
    }

    // ---------- polygon ----------
    fn in_triangle(a: (i64, i64), b: (i64, i64), c: (i64, i64), p: (i64, i64)) -> bool {
        if cross(a, b, c) == 0 {
            return on_seg(a, b, p) || on_seg(b, c, p) || on_seg(a, c, p);
        }
        let (d1, d2, d3) = (cross(a, b, p), cross(b, c, p), cross(c, a, p));
        let neg = d1 < 0 || d2 < 0 || d3 < 0;
        let pos = d1 > 0 || d2 > 0 || d3 > 0;
        !(neg && pos)
    }
    fn in_conv(pts: &[(i64, i64)], p: (i64, i64)) -> bool {
        let n = pts.len();
        for i in 0..n {
            for j in i..n {
                for k in j..n {
                    if in_triangle(pts[i], pts[j], pts[k], p) {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[test]
    fn convex_hull_stress() {
        let mut rng = Random::new_with_seed(140);
        for iter in 0..20000 {
            let n = rng.gen_range(1..=9usize);
            let r = [0i64, 1, 2, 5][iter % 4];
            let mut pts: Vec<(i64, i64)> = (0..n).map(|_| gen_pt(&mut rng, r)).collect();
            if iter % 7 == 0 {
                // collinear
                let k = rng.gen_range(-2..=2i64);
                for p in pts.iter_mut() {
                    p.1 = p.0 * k;
                }
            }
            let distinct: BTreeSet<(i64, i64)> = pts.iter().copied().collect();
            let extreme: BTreeSet<(i64, i64)> = distinct
                .iter()
                .copied()
                .filter(|&p| {
                    let others: Vec<(i64, i64)> = distinct.iter().copied().filter(|&o| o != p).collect();
                    !in_conv(&others, p)
                })
                .collect();
            let mut v: Vec<Point<i64>> = pts.iter().map(|&p| ip(p)).collect();
            let hull = v.as_mut_slice().convex_hull();
            let got: Vec<(i64, i64)> = hull.points.iter().map(|p| (p.x, p.y)).collect();
            let got_set: BTreeSet<(i64, i64)> = got.iter().copied().collect();
            assert_eq!(got.len(), got_set.len(), "dup in hull {pts:?} -> {got:?}");
            assert_eq!(got_set, extreme, "{pts:?} -> {got:?}");
            let h = got.len();
            if h >= 3 {
                let signs: BTreeSet<i64> = (0..h).map(|i| cross(got[i], got[(i + 1) % h], got[(i + 2) % h]).signum()).collect();
                assert_eq!(signs.len(), 1, "{pts:?} -> {got:?}");
                assert!(!signs.contains(&0));
            }
            // contains + area by Pick
            let (mut interior, mut boundary) = (0i64, 0i64);
            for x in -r - 1..=r + 1 {
                for y in -r * 2 - 1..=r * 2 + 1 {
                    let p = (x, y);
                    let exp = in_conv(&pts, p);
                    assert_eq!(hull.contains(ip(p)), exp, "contains {got:?} {p:?}");
                    if exp {
                        let on_b = (0..h).any(|i| on_seg(got[i], got[(i + 1) % h], p));
                        if on_b {
                            boundary += 1;
                        } else {
                            interior += 1;
                        }
                    }
                }
            }
            if h >= 3 {
                assert_eq!(hull.double_area().abs(), 2 * interior + boundary - 2, "area {got:?}");
                let hq = Polygon::new(got.iter().map(|&p| qp(p)).collect());
                assert_eq!(hq.area().abs() * (q(2)), Rational::new_int(hull.double_area().abs() as i128));
            } else {
                assert_eq!(hull.double_area(), 0);
            }
            // farthest / closest
            let pv: Vec<Point<i64>> = pts.iter().map(|&p| ip(p)).collect();
            let mut mx = 0;
            let mut mn = i64::MAX;
            for i in 0..n {
                for j in i + 1..n {
                    let d = pv[i].square_dist_point(pv[j]);
                    mx = mx.max(d);
                    mn = mn.min(d);
                }
            }
            let (a, b) = farthest_pair(&pv);
            assert_eq!(a.square_dist_point(b), mx, "farthest {pts:?}");
            assert!(pv.contains(&a) && pv.contains(&b));
            if n >= 2 {
                let (i, j) = closest_pair(&pv);
                assert_ne!(i, j);
                assert_eq!(pv[i].square_dist_point(pv[j]), mn, "closest {pts:?}");
            }
        }
    }

    #[test]
    fn point_pairs_larger() {
        let mut rng = Random::new_with_seed(150);
        for iter in 0..3000 {
            let n = rng.gen_range(2..=80usize);
            let (rx, ry) = [(3i64, 3i64), (0, 50), (50, 0), (1, 1000), (1000, 1), (10, 10), (1_000_000_000, 1_000_000_000)][iter % 7];
            let pv: Vec<Point<i64>> = (0..n).map(|_| Point::new(rng.gen_range(-rx..=rx), rng.gen_range(-ry..=ry))).collect();
            let mut mx = 0;
            let mut mn = i64::MAX;
            for i in 0..n {
                for j in i + 1..n {
                    let d = pv[i].square_dist_point(pv[j]);
                    mx = mx.max(d);
                    mn = mn.min(d);
                }
            }
            let (a, b) = farthest_pair(&pv);
            assert_eq!(a.square_dist_point(b), mx);
            let (i, j) = closest_pair(&pv);
            assert_ne!(i, j);
            assert_eq!(pv[i].square_dist_point(pv[j]), mn);
        }
    }

    // ---------- arg sort ----------
    fn exact_arg_cmp(a: (i64, i64), b: (i64, i64)) -> Ordering {
        // angle in (-pi, pi], origin = angle 0
        let half = |p: (i64, i64)| -> u8 {
            if p.1 < 0 {
                0
            } else if p.1 == 0 && p.0 >= 0 {
                1
            } else if p.1 > 0 {
                2
            } else {
                3
            }
        };
        half(a).cmp(&half(b)).then_with(|| {
            let c = a.0 as i128 * b.1 as i128 - a.1 as i128 * b.0 as i128;
            0.cmp(&c)
        })
    }

    #[test]
    fn arg_sort_stress() {
        let mut rng = Random::new_with_seed(160);
        for iter in 0..3000 {
            let r = [1i64, 2, 3, 2_000_000_000][iter % 4];
            let n = rng.gen_range(0..=30usize);
            let pts: Vec<(i64, i64)> = (0..n).map(|_| gen_pt(&mut rng, r)).collect();
            for &a in &pts {
                for &b in &pts {
                    assert_eq!(arg_cmp(ip(a), ip(b)), exact_arg_cmp(a, b), "{a:?} {b:?}");
                    if r <= 3 {
                        let (fa, fb) = ((a.1 as f64).atan2(a.0 as f64), (b.1 as f64).atan2(b.0 as f64));
                        if (fa - fb).abs() > 1e-9 {
                            assert_eq!(arg_cmp(ip(a), ip(b)), fa.partial_cmp(&fb).unwrap());
                        } else {
                            assert_eq!(arg_cmp(ip(a), ip(b)), Ordering::Equal);
                        }
                    }
                }
            }
            let mut v: Vec<Point<i64>> = pts.iter().map(|&p| ip(p)).collect();
            sort_by_argument(&mut v);
            assert!(v.windows(2).all(|w| exact_arg_cmp((w[0].x, w[0].y), (w[1].x, w[1].y)) != Ordering::Greater));
            // i128 points with i64-scale coordinates
            let big: Vec<(i64, i64)> = (0..n).map(|_| gen_pt(&mut rng, i64::MAX / 2)).collect();
            for &a in &big {
                for &b in &big {
                    let pa = Point::new(a.0 as i128, a.1 as i128);
                    let pb = Point::new(b.0 as i128, b.1 as i128);
                    assert_eq!(arg_cmp(pa, pb), exact_arg_cmp(a, b));
                }
            }
        }
    }

    // ---------- min enclosing circle ----------
    #[test]
    fn mec_stress() {
        let mut rng = Random::new_with_seed(170);
        for iter in 0..3000 {
            let n = rng.gen_range(1..=12usize);
            let r = [0i64, 1, 3, 1000][iter % 4];
            let mut raw: Vec<(i64, i64)> = (0..n).map(|_| gen_pt(&mut rng, r)).collect();
            if iter % 5 == 0 {
                for p in raw.iter_mut() {
                    p.1 = 2 * p.0;
                }
            }
            let pts: Vec<Point<Real>> = raw.iter().map(|&p| rp(p)).collect();
            let c = min_enclosing_circle(&pts);
            let (cx, cy, rad) = (c.center.x.0, c.center.y.0, c.radius.0);
            let eps = 1e-7 * (1.0 + rad);
            for &(x, y) in &raw {
                assert!(((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt() <= rad + eps, "{raw:?}");
            }
            let encloses = |c: (f64, f64), rr: f64| raw.iter().all(|&(x, y)| ((x as f64 - c.0).powi(2) + (y as f64 - c.1).powi(2)).sqrt() <= rr + eps);
            let mut best = if raw.iter().all(|&p| p == raw[0]) { 0.0 } else { f64::MAX };
            for i in 0..n {
                for j in i + 1..n {
                    let (a, b) = (f(raw[i]), f(raw[j]));
                    let m = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
                    let rr = ((a.0 - m.0).powi(2) + (a.1 - m.1).powi(2)).sqrt();
                    if encloses(m, rr) {
                        best = best.min(rr);
                    }
                    for k in j + 1..n {
                        let cc = f(raw[k]);
                        let (bx, by) = (b.0 - a.0, b.1 - a.1);
                        let (cx, cy) = (cc.0 - a.0, cc.1 - a.1);
                        let d = 2.0 * (bx * cy - by * cx);
                        if d == 0.0 {
                            continue;
                        }
                        let (b2, c2) = (bx * bx + by * by, cx * cx + cy * cy);
                        let ctr = (a.0 + (cy * b2 - by * c2) / d, a.1 + (bx * c2 - cx * b2) / d);
                        let rr = ((a.0 - ctr.0).powi(2) + (a.1 - ctr.1).powi(2)).sqrt();
                        if encloses(ctr, rr) {
                            best = best.min(rr);
                        }
                    }
                }
            }
            assert!((rad - best).abs() <= 1e-6 * (1.0 + best), "{raw:?} rad={rad} best={best}");
        }
    }

    // ---------- manhattan mst ----------
    fn prim(points: &[(i64, i64)]) -> i64 {
        let n = points.len();
        if n == 0 {
            return 0;
        }
        let mut dist = vec![i64::MAX; n];
        let mut used = vec![false; n];
        dist[0] = 0;
        let mut total = 0;
        for _ in 0..n {
            let v = (0..n).filter(|&v| !used[v]).min_by_key(|&v| dist[v]).unwrap();
            used[v] = true;
            total += dist[v];
            for u in 0..n {
                let d = (points[v].0 - points[u].0).abs() + (points[v].1 - points[u].1).abs();
                if !used[u] && d < dist[u] {
                    dist[u] = d;
                }
            }
        }
        total
    }

    #[test]
    fn manhattan_mst_stress() {
        let mut rng = Random::new_with_seed(180);
        for iter in 0..20000 {
            let n = rng.gen_range(0..=14usize);
            let (rx, ry) = [(0i64, 0i64), (1, 1), (2, 2), (0, 5), (5, 0), (4, 4), (20, 2), (1_000_000_000, 1_000_000_000)][iter % 8];
            let pts: Vec<(i64, i64)> = (0..n).map(|_| (rng.gen_range(-rx..=rx), rng.gen_range(-ry..=ry))).collect();
            let tree = manhattan_mst(&pts);
            assert_eq!(tree.len(), n.saturating_sub(1), "{pts:?}");
            let mut dsu: Vec<usize> = (0..n).collect();
            fn find(d: &mut Vec<usize>, v: usize) -> usize {
                if d[v] != v {
                    let r = find(d, d[v]);
                    d[v] = r;
                }
                d[v]
            }
            for &(w, a, b) in &tree {
                assert_eq!(w, (pts[a].0 - pts[b].0).abs() + (pts[a].1 - pts[b].1).abs());
                let (x, y) = (find(&mut dsu, a), find(&mut dsu, b));
                assert_ne!(x, y);
                dsu[x] = y;
            }
            assert_eq!(tree.iter().map(|e| e.0).sum::<i64>(), prim(&pts), "{pts:?}");
        }
    }

    // ---------- angle / utils / line ----------
    #[test]
    fn angle_and_utils() {
        let mut rng = Random::new_with_seed(190);
        let pi = std::f64::consts::PI;
        for _ in 0..20000 {
            let (o, a, b) = (gen_pt(&mut rng, 4), gen_pt(&mut rng, 4), gen_pt(&mut rng, 4));
            if a == o || b == o {
                continue;
            }
            let ang = Angle::new(rp(o), rp(a), rp(b));
            let v = ang.value().0;
            assert!(v >= -1e-9 && v <= 2.0 * pi + 1e-9);
            let a1 = ((a.1 - o.1) as f64).atan2((a.0 - o.0) as f64);
            let a2 = ((b.1 - o.1) as f64).atan2((b.0 - o.0) as f64);
            let diff = (a2 - a1 - v) / (2.0 * pi);
            assert!((diff - diff.round()).abs() < 1e-9);
            let bis = ang.bisector();
            let ba = bis.angle().0;
            let d = (ba - (a1 + v / 2.0)) / (2.0 * pi);
            assert!((d - d.round()).abs() < 1e-9);
            let x = rng.gen_range(-100000..=100000i64) as f64 / 1000.0;
            let c = canonize_angle(Real(x)).0;
            assert!(c >= -pi - 1e-9 && c <= pi + 1e-9);
            let k = (c - x) / (2.0 * pi);
            assert!((k - k.round()).abs() < 1e-9);
            let c = canonize_angle_base(Real(x), Real(1.0)).0;
            assert!(c >= 1.0 - 1e-9 && c <= 1.0 + 2.0 * pi + 1e-9);
        }
    }

    #[test]
    fn line_stress() {
        let mut rng = Random::new_with_seed(200);
        for _ in 0..50000 {
            let (a, b, c, d) = (gen_pt(&mut rng, 4), gen_pt(&mut rng, 4), gen_pt(&mut rng, 4), gen_pt(&mut rng, 4));
            if a == b || c == d {
                continue;
            }
            let l1 = qp(a).line(qp(b));
            let l2 = qp(c).line(qp(d));
            assert!(l1.contains(qp(a)) && l1.contains(qp(b)));
            let par = (b.0 - a.0) * (d.1 - c.1) - (b.1 - a.1) * (d.0 - c.0) == 0;
            assert_eq!(l1.is_parallel(l2), par);
            let perp = (b.0 - a.0) * (d.0 - c.0) + (b.1 - a.1) * (d.1 - c.1) == 0;
            assert_eq!(l1.is_perpendicular(l2), perp);
            let same = par && cross(a, b, c) == 0;
            assert_eq!(l1 == l2, same);
            if !par {
                let p = l1.intersect(l2);
                assert!(l1.contains(p) && l2.contains(p));
            }
            let pp = l1.perpendicular(qp(c));
            assert!(pp.contains(qp(c)) && pp.is_perpendicular(l1));
            let pl = l1.parallel(qp(c));
            assert!(pl.contains(qp(c)) && pl.is_parallel(l1));
            let cr = cross(a, b, c) as f64;
            let len2 = ((b.0 - a.0).pow(2) + (b.1 - a.1).pow(2)) as f64;
            assert!((qf(l1.square_dist_point(qp(c))) - cr * cr / len2).abs() < 1e-9);
            let lr = rp(a).line(rp(b));
            assert!((lr.dist_point(rp(c)).0 - cr.abs() / len2.sqrt()).abs() < 1e-9);
            let _ = l1.square_dist_point(qp(c)).into_real();
        }
    }
}

// =====================================================================
// FIX VERIFICATION (copies of library logic with the proposed fix)
// =====================================================================
#[path = "audit_string_geom_support/ac_fixed.rs"]
mod ac_fixed;

mod fixes {
    use super::*;

    impl ac_fixed::ACPayload for Ids {
        fn add_single(&mut self, id: usize) {
            self.0.push(id);
        }
        fn add_node(&mut self, other: &Self) {
            self.0.extend_from_slice(&other.0);
        }
    }

    #[test]
    fn fixed_aho_with_empty_pattern() {
        let mut rng = Random::new_with_seed(21);
        for iter in 0..5000 {
            let k = rng.gen_range(1..=4usize);
            let alphabet = [1u8, 2, 3][iter % 3];
            let patterns: Vec<Vec<u8>> = (0..k)
                .map(|_| {
                    let len = rng.gen_range(0..=3usize);
                    gen_str(&mut rng, len, alphabet, b'a')
                })
                .collect();
            let n = rng.gen_range(0..=8usize);
            let text = gen_str(&mut rng, n, alphabet, b'a');
            let ac = ac_fixed::AhoCorasickLowercase::<Ids>::new(&patterns);
            let got: Vec<Vec<usize>> = ac
                .iterate(&text)
                .map(|p| {
                    let mut v = p.0.clone();
                    v.sort();
                    v
                })
                .collect();
            for pos in 0..=text.len() {
                let exp: Vec<usize> = (0..patterns.len()).filter(|&i| text[..pos].ends_with(&patterns[i])).collect();
                assert_eq!(got[pos], exp, "patterns {:?} text {:?} pos {}", patterns, text, pos);
            }
        }
    }

    // Kasai with the sentinel excluded from comparisons
    fn fixed_build_lcp<T: PartialEq>(s: &[T], sa: &[usize], pos: &[usize]) -> Vec<u32> {
        let n = s.len() + 1;
        let mut lcp = vec![0; n - 1];
        let mut k = 0usize;
        for i in 0..n {
            k = k.saturating_sub(1);
            if pos[i] == n - 1 {
                k = 0;
                continue;
            }
            let j = sa[pos[i] + 1];
            while i + k < n - 1 && j + k < n - 1 && s[i + k] == s[j + k] {
                k += 1;
            }
            lcp[pos[i]] = k as u32
        }
        lcp
    }

    fn fixed_find<T: Ord>(s: &[T], sa: &[usize], t: &[T]) -> (usize, usize) {
        let mut l = 0;
        let mut r = sa.len() - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if &s[sa[mid]..] < t {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        let from = l + 1;
        let mut r = sa.len() - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if s[sa[mid]..].starts_with(t) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        (from, l + 1)
    }

    #[test]
    fn fixed_suffix_array_lcp_and_find() {
        for alphabet in [vec![0i32, 1], vec![-2i32, -1], vec![-1, 0, 1]] {
            let strs = all_strings(&alphabet, if alphabet.len() == 2 { 7 } else { 5 });
            for s in &strs {
                let n = s.len();
                let sa_struct = SuffixArray::new(s);
                let sa: Vec<usize> = (&sa_struct).into_iter().collect();
                let pos: Vec<usize> = (0..=n).map(|i| sa_struct.get_pos_in_array(i)).collect();
                let lcp = fixed_build_lcp(s, &sa, &pos);
                for i in 0..n {
                    assert_eq!(lcp[i] as usize, naive_lcp(&s[sa[i]..], &s[sa[i + 1]..]));
                }
                for t in strs.iter().filter(|t| !t.is_empty() && t.len() <= 3) {
                    let (from, to) = fixed_find(s, &sa, t);
                    let got: BTreeSet<usize> = (from..to).map(|i| sa[i]).collect();
                    let exp: BTreeSet<usize> = (0..n).filter(|&i| s[i..].starts_with(t)).collect();
                    assert_eq!(got, exp);
                }
            }
        }
    }

    fn fixed_split<'a>(s: &'a [u8], pattern: &[u8]) -> Vec<&'a [u8]> {
        let mut res = Vec::new();
        let mut start = 0;
        for i in 0..s.len() {
            if i >= start && s[i..].starts_with(pattern) {
                res.push(&s[start..i]);
                start = i + pattern.len();
            }
        }
        res.push(&s[start..]);
        res
    }

    #[test]
    fn fixed_split_stress() {
        let mut rng = Random::new_with_seed(90);
        for _ in 0..20000 {
            let n = rng.gen_range(0..=12usize);
            let s = gen_str(&mut rng, n, 2, b'a');
            let m = rng.gen_range(1..=3usize);
            let p = gen_str(&mut rng, m, 2, b'a');
            let ss = String::from_utf8(s.clone()).unwrap();
            let ps = String::from_utf8(p.clone()).unwrap();
            let exp: Vec<Vec<u8>> = ss.split(ps.as_str()).map(|x| x.as_bytes().to_vec()).collect();
            let got: Vec<Vec<u8>> = fixed_split(&s, &p).into_iter().map(|x| x.to_vec()).collect();
            assert_eq!(got, exp);
        }
    }

    fn fixed_read_line(input: &mut Input) -> Vec<u8> {
        let mut res = Vec::new();
        while let Some(c) = input.get() {
            if c == b'\n' {
                break;
            }
            res.push(c);
        }
        res
    }

    #[test]
    fn fixed_read_line_check() {
        for data in [&b"abc\ndef"[..], b"abc\r\ndef", b"a", b"\nab", b"ab\n", b"ab\r\n\r\nc"] {
            let mut input = Input::slice(data);
            let text = String::from_utf8(data.to_vec()).unwrap();
            for line in text.lines() {
                assert_eq!(fixed_read_line(&mut input), line.as_bytes(), "{text:?}");
            }
        }
    }

    #[test]
    fn read_line_bug_variants() {
        for data in [&b"abc\ndef"[..], b"abc\r\ndef", b"a", b"\nab", b"ab\n", b"ab\r\n\r\nc"] {
            let mut input = Input::slice(data);
            let text = String::from_utf8(data.to_vec()).unwrap();
            for line in text.lines() {
                assert_eq!(input.read_line().as_slice(), line.as_bytes(), "{text:?}");
            }
        }
    }
}

mod extra {
    use super::*;
    use algo_lib::geometry::circle::Circle;
    use algo_lib::geometry::line::Line;
    use algo_lib::geometry::point::Point;
    use algo_lib::geometry::polygon::Polygon;
    use algo_lib::numbers::real::Real;

    fn fixed_intersect_line(c: &Circle<Real>, l: Line<Real>) -> Vec<Point<Real>> {
        let dist = l.dist_point(c.center);
        if dist > c.radius {
            return vec![];
        }
        let perp = l.perpendicular(c.center);
        let base = l.intersect(perp);
        if dist == c.radius {
            return vec![base];
        }
        let delta = (c.radius * c.radius - dist * dist).sqrt() / Real::hypot(perp.a, perp.b);
        vec![
            base + Point::new(perp.a, perp.b) * delta,
            base - Point::new(perp.a, perp.b) * delta,
        ]
    }

    #[test]
    fn fixed_circle_line() {
        let mut rng = Random::new_with_seed(120);
        for _ in 0..50000 {
            let c = (rng.gen_range(-5..=5i64), rng.gen_range(-5..=5i64));
            let rad = rng.gen_range(1..=5i64);
            let a = (rng.gen_range(-6..=6i64), rng.gen_range(-6..=6i64));
            let b = (rng.gen_range(-6..=6i64), rng.gen_range(-6..=6i64));
            if a == b {
                continue;
            }
            let pa = Point::new(Real(a.0 as f64), Real(a.1 as f64));
            let pb = Point::new(Real(b.0 as f64), Real(b.1 as f64));
            let l = pa.line(pb);
            let circle = Circle::new(Point::new(Real(c.0 as f64), Real(c.1 as f64)), Real(rad as f64));
            let pts = fixed_intersect_line(&circle, l);
            let cr = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
            let len2 = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2);
            let exp = match (cr * cr).cmp(&(rad * rad * len2)) {
                std::cmp::Ordering::Less => 2,
                std::cmp::Ordering::Equal => 1,
                std::cmp::Ordering::Greater => 0,
            };
            assert_eq!(pts.len(), exp);
            for p in &pts {
                let d = ((p.x.0 - c.0 as f64).powi(2) + (p.y.0 - c.1 as f64).powi(2)).sqrt();
                assert!((d - rad as f64).abs() < 1e-6);
                assert!(l.value(*p).0.abs() / (len2 as f64).sqrt() < 1e-6);
            }
        }
    }

    #[test]
    fn polygon_contains_with_duplicate_vertex() {
        let sq = Polygon::new(vec![
            Point::new(0i64, 0),
            Point::new(2, 0),
            Point::new(2, 0),
            Point::new(2, 2),
            Point::new(0, 2),
        ]);
        assert!(sq.contains(Point::new(1, 1)));
    }

    #[test]
    fn aho_long_patterns_no_stack_overflow() {
        let n = 300_000;
        let mut p0 = vec![b'b'];
        p0.extend(std::iter::repeat(b'a').take(n));
        let p1 = vec![b'a'; n];
        let mut p2 = vec![b'a'; n];
        p2.push(b'b');
        let ac = AhoCorasickLowercase::<Ids>::new(&[p0, p1, p2]);
        assert!(ac.len() > 2 * n);
    }

    #[test]
    fn big_inputs_speed() {
        let n = 300_000;
        let mut rng = Random::new_with_seed(5);
        for alphabet in [1u8, 2] {
            let s = gen_str(&mut rng, n, alphabet, b'a');
            let sa = SuffixArray::new(&s);
            assert_eq!(sa.len(), n + 1);
            let _ = sa.lcp(1, n);
            let r = runs(&s);
            assert!(r.len() < n);
            let t = PalindromicTree::from_slice(&s);
            assert!(t.distinct_palindromes() <= n);
            let a = SuffixAutomatonTree::new(&s);
            assert!(a.state_count() <= 2 * n);
            let _ = s.z_algorithm();
            let _ = s.prefix_function();
            let _ = s.odd_palindromes();
            let _ = s.even_palindromes();
            let _ = lyndon_factorization(&s);
        }
    }
}

// =====================================================================
// MINIMAL REPROS (expected to FAIL on the current library)
// =====================================================================
mod repro {
    #[test]
    fn repro_read_line_drops_last_char_without_trailing_newline() {
        use algo_lib::io::input::Input;
        use algo_lib::string::str::StrReader;
        let mut input = Input::slice(b"ab");
        assert_eq!(input.read_line().as_slice(), b"ab"); // got b"a"
    }

    #[test]
    fn repro_read_lines_drops_last_char() {
        use algo_lib::io::input::Input;
        use algo_lib::string::str::StrReader;
        let mut input = Input::slice(b"x y\nab");
        let lines = input.read_lines();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[1].as_slice(), b"ab"); // got b"a"
    }

    #[test]
    fn repro_str_split_overlapping_panics() {
        use algo_lib::string::split::StrSplit;
        let s: &[u8] = b"aaa";
        assert_eq!(s.str_split(b"aa"), vec![&b""[..], &b"a"[..]]); // panics
    }

    #[test]
    fn repro_circle_intersect_line_not_normalized() {
        use algo_lib::geometry::circle::Circle;
        use algo_lib::geometry::point::Point;
        use algo_lib::numbers::real::Real;
        let c = Circle::new(Point::new(Real(0.0), Real(0.0)), Real(1.0));
        let l = Point::new(Real(-2.0), Real(0.0)).line(Point::new(Real(2.0), Real(0.0)));
        let pts = c.intersect_line(l);
        assert_eq!(pts.len(), 2);
        for p in pts {
            assert!((p.x.0.abs() - 1.0).abs() < 1e-9, "x = {}", p.x.0); // got x = +-4
        }
    }

    #[test]
    fn repro_suffix_array_lcp_with_zero_element() {
        use algo_lib::string::suffix_array::SuffixArray;
        // suffixes of [1, 0]: "" (sentinel only), [0], [1, 0]
        let sa = SuffixArray::new(&[1u32, 0]);
        assert_eq!((sa[0], sa[1], sa[2]), (2, 1, 0));
        assert_eq!(sa.lcp(0, 1), 0); // got 1: the sentinel is compared equal to the real 0
    }

    #[test]
    fn repro_suffix_array_find_with_nonpositive_elements() {
        use algo_lib::string::suffix_array::SuffixArray;
        let sa = SuffixArray::new(&[-2i32, -2]);
        assert_eq!(sa.find(&[-2, -2]), (2, 3)); // got empty range
        let sa = SuffixArray::new(&[0u8]);
        let (from, to) = sa.find(&[0, 0]);
        assert_eq!(from, to); // got one (false) match
    }

    #[test]
    fn repro_aho_corasick_empty_pattern_lost_at_depth_one() {
        use algo_lib::string::aho_corasick::{ACPayload, AhoCorasickLowercase};
        #[derive(Default)]
        struct Cnt(usize);
        impl ACPayload for Cnt {
            fn add_single(&mut self, _id: usize) {
                self.0 += 1;
            }
            fn add_node(&mut self, other: &Self) {
                self.0 += other.0;
            }
        }
        let ac = AhoCorasickLowercase::<Cnt>::new(&[&b""[..], &b"ab"[..]]);
        let text = b"ab".to_vec();
        let got: Vec<usize> = ac.iterate(&text).map(|p| p.0).collect();
        assert_eq!(got, vec![1, 1, 2]); // got [1, 0, 2]
    }

    #[test]
    fn repro_z_and_prefix_function_of_empty() {
        use algo_lib::string::string_algorithms::prefix_function::PrefixFunction;
        use algo_lib::string::string_algorithms::z_algorithm::ZAlgorithm;
        let e: &[u8] = b"";
        assert_eq!(e.z_algorithm().len(), 0); // got [0]
        assert_eq!(e.prefix_function().len(), 0); // got [0]
    }
}
