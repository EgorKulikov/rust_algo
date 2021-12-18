//{"name":"Finding strings","group":"HackerEarth - Data Structures and Algorithms Coding Contest","url":"https://www.hackerearth.com/challenges/competitive/data-structures-and-algorithms-coding-contest-dec/algorithm/string-finder-2-9aafc475/","interactive":false,"timeLimit":5000,"tests":[{"input":"5 4\nab ba ab abca abcaba\nA 1 2 ca\nA 1 4 ba\n? 1 5 abcaba\n? 1 5 abba\n","output":"3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FindingStrings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::value::ConstValue;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line, value};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let q = input.read();
    let s: Vec<Str> = input.read_vec(n);
    let queries: Vec<(char, usize, usize, Str)> = input.read_vec(q);

    value!(Module, i64, 29996224275833);
    type Mod = ModInt<i64, Module>;

    fn hash(s: &Str) -> (Mod, Mod) {
        let mut hash = Mod::zero();
        let mut mult = Mod::one();
        let step = Mod::new(101);

        for c in s.iter() {
            hash *= step;
            mult *= step;
            hash += Mod::new(c as i64);
        }

        (hash, mult)
    }

    struct Seg {
        from: usize,
        to: usize,
        vals: Vec<Mod>,
        map: HashMap<Mod, usize>,
        mult: Mod,
        add: Mod,
    }

    impl Seg {
        pub fn new(from: usize, to: usize, s: &[Str]) -> Self {
            let s = &s[from..to];
            let mut vals = Vec::with_capacity(to - from);
            for i in 0..s.len() {
                vals.push(hash(&s[i]).0);
            }
            let mut res = Seg {
                from,
                to,
                vals,
                map: HashMap::new(),
                mult: Mod::one(),
                add: Mod::zero(),
            };
            res.build_map();
            res
        }

        fn build_map(&mut self) {
            self.map.clear();
            for v in self.vals.iter() {
                if self.map.contains_key(v) {
                    *self.map.get_mut(v).unwrap() += 1;
                } else {
                    self.map.insert(*v, 1);
                }
            }
        }

        fn reset(&mut self) {
            for i in 0..self.vals.len() {
                self.vals[i] *= self.mult;
                self.vals[i] += self.add;
            }
            self.mult = Mod::one();
            self.add = Mod::zero();
        }

        pub fn query(&mut self, from: usize, to: usize, hash: Mod) -> usize {
            if self.to <= from || self.from >= to {
                0
            } else if self.from >= from && self.to <= to {
                let key = (hash - self.add) / self.mult;
                *self.map.get(&key).unwrap_or(&0)
            } else {
                self.reset();
                self.build_map();
                let mut res = 0;
                for i in self.from.max(from)..self.to.min(to) {
                    if self.vals[i - self.from] == hash {
                        res += 1;
                    }
                }
                res
            }
        }

        pub fn update(&mut self, from: usize, to: usize, hash: Mod, mult: Mod) {
            if self.to <= from || self.from >= to {
                return;
            }
            if self.from >= from && self.to <= to {
                self.mult *= mult;
                self.add *= mult;
                self.add += hash;
            } else {
                self.reset();
                for i in self.from.max(from)..self.to.min(to) {
                    self.vals[i - self.from] *= mult;
                    self.vals[i - self.from] += hash;
                }
                self.build_map();
            }
        }
    }

    let buben = 300;

    let mut segs = Vec::new();

    for i in (0..n).step_by(buben) {
        segs.push(Seg::new(i, (i + buben).min(n), &s));
    }

    for (t, l, r, st) in queries {
        if t == 'A' {
            let (hash, mult) = hash(&st);
            for seg in segs.iter_mut() {
                seg.update(l - 1, r, hash, mult);
            }
        } else {
            let hash = hash(&st).0;
            let mut ans = 0;
            for seg in segs.iter_mut() {
                ans += seg.query(l - 1, r, hash);
            }
            out_line!(ans);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
