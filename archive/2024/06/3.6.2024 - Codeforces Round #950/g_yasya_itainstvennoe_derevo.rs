//{"name":"G. Яся и таинственное дерево","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3 7\n1 2 1\n3 1 8\n^ 5\n? 2 9\n^ 1\n? 1 10\n^ 6\n? 3 1\n? 2 9\n5 6\n1 2 777\n3 2 2812\n4 1 16\n5 3 1000000000\n^ 4\n? 3 123\n? 5 1000000000\n^ 1000000000\n? 1 908070\n? 2 1\n","output":"13 15 11 10\n1000000127 2812 999756331 999999756\n"},{"input":"3\n8 4\n8 6 3\n6 3 4\n2 5 4\n7 6 2\n7 1 10\n4 1 4\n5 1 2\n^ 4\n^ 7\n? 7 8\n? 4 10\n5 6\n3 1 4\n2 3 9\n4 3 6\n5 2 10\n? 5 7\n^ 1\n^ 8\n? 4 10\n? 1 9\n? 3 6\n4 2\n2 1 4\n4 3 5\n2 3 4\n^ 13\n? 1 10\n","output":"14 13\n13 8 11 11\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GYasyaITainstvennoeDerevo"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, u32)>(n - 1).dec();

    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Mark {
        None,
        Single(u32),
        More,
    }
    struct Node {
        zero: Option<Box<Node>>,
        one: Option<Box<Node>>,
        single: Mark,
    }

    impl Node {
        fn empty() -> Self {
            Self {
                zero: None,
                one: None,
                single: Mark::None,
            }
        }

        fn insert(&mut self, x: u32, bit: usize) {
            match &mut self.single {
                Mark::None => {
                    self.single = Mark::Single(x);
                }
                Mark::Single(_) => {
                    self.single = Mark::More;
                }
                Mark::More => {}
            }
            if bit == 0 {
                return;
            }
            if !x.is_set(bit - 1) {
                if self.zero.is_none() {
                    self.zero = Some(Box::new(Node::empty()));
                }
                self.zero.as_mut().unwrap().insert(x, bit - 1);
            } else {
                if self.one.is_none() {
                    self.one = Some(Box::new(Node::empty()));
                }
                self.one.as_mut().unwrap().insert(x, bit - 1);
            }
        }

        fn find_furthest(&mut self, x: u32, bit: usize, forbidden_single: Option<u32>) -> u32 {
            if forbidden_single.is_some() && self.single == Mark::Single(forbidden_single.unwrap())
            {
                assert_eq!(bit, 30);
                return 0;
            }
            if bit == 0 {
                return 0;
            }
            if x.is_set(bit - 1) {
                if self.zero.is_none()
                    || forbidden_single.is_some()
                        && self.zero.as_ref().unwrap().single
                            == Mark::Single(forbidden_single.unwrap())
                {
                    self.one
                        .as_mut()
                        .unwrap()
                        .find_furthest(x, bit - 1, forbidden_single)
                } else {
                    self.zero
                        .as_mut()
                        .unwrap()
                        .find_furthest(x, bit - 1, forbidden_single)
                        .with_bit(bit - 1)
                }
            } else {
                if self.one.is_none()
                    || forbidden_single.is_some()
                        && self.one.as_ref().unwrap().single
                            == Mark::Single(forbidden_single.unwrap())
                {
                    self.zero
                        .as_mut()
                        .unwrap()
                        .find_furthest(x, bit - 1, forbidden_single)
                } else {
                    self.one
                        .as_mut()
                        .unwrap()
                        .find_furthest(x, bit - 1, forbidden_single)
                        .with_bit(bit - 1)
                }
            }
        }
    }

    let mut even_root = Node::empty();
    let mut odd_root = Node::empty();
    let graph = Graph::from_biedges_with_payload(n, &edges);
    let mut is_even = BitSet::new(n);
    let mut val = vec![0; n];
    let mut dfs = RecursiveFunction4::new(|f, vert: usize, prev: usize, xor: u32, even: bool| {
        is_even.change(vert, even);
        val[vert] = xor;
        if even {
            even_root.insert(xor, 30);
        } else {
            odd_root.insert(xor, 30);
        }
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert, xor ^ e.payload(), !even);
        }
    });
    dfs.call(0, n, 0, true);
    let mut xor = 0;
    let mut ans = Vec::new();
    for _ in 0..m {
        let tp = input.read_char();
        match tp {
            '^' => {
                let x = input.read_unsigned();
                xor ^= x;
            }
            '?' => {
                let vert = input.read_size() - 1;
                let x = input.read_unsigned();
                let (a1, a2) = if is_even[vert] {
                    (
                        even_root.find_furthest(val[vert] ^ x, 30, Some(val[vert])),
                        odd_root.find_furthest(val[vert] ^ x ^ xor, 30, None),
                    )
                } else {
                    (
                        odd_root.find_furthest(val[vert] ^ x, 30, Some(val[vert])),
                        even_root.find_furthest(val[vert] ^ x ^ xor, 30, None),
                    )
                };
                ans.push(a1.max(a2));
            }
            _ => unreachable!(),
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
