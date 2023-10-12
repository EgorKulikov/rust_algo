//{"name":"G. Аня и таинственная строка","group":"Codeforces - Codeforces Round 903 (Div. 3)","url":"https://codeforces.com/contest/1881/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n12 8\ntedubcyyxopz\n2 2 5\n1 4 8 2\n2 1 7\n1 3 5 40\n1 9 11 3\n1 10 10 9\n2 4 10\n2 10 12\n10 4\nubnxwwgzjt\n2 4 10\n2 10 10\n1 6 10 8\n2 7 7\n11 3\nhntcxfxyhtu\n1 4 6 1\n2 4 10\n1 4 10 21\n13 2\nyxhlmzfhqctir\n1 5 9 3\n1 8 13 15\n2 3\nbp\n1 1 2 15\n1 1 2 18\n1 2 2 1000000000\n","output":"YES\nNO\nNO\nYES\nNO\nYES\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAnyaITainstvennayaStroka"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_str();

    #[derive(Copy, Clone)]
    struct Node(i64);

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.0 % 26 == other.0 % 26
        }
    }

    impl Eq for Node {}

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self(0)
        }

        fn join(
            &mut self,
            _left_val: &Self,
            _right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
        }

        fn accumulate(&mut self, value: &Self, _left: usize, _right: usize) {
            self.0 += value.0;
        }

        fn reset_delta(&mut self, _left: usize, _right: usize) {
            self.0 = 0;
        }
    }

    let mut tree = SegmentTree::from_generator(n, |i| Node(s[i] as i64));
    let mut d2 = BTreeSet::new();
    let mut d3 = BTreeSet::new();

    for i in 0..n {
        if i + 1 < n && s[i] == s[i + 1] {
            d2.insert(i);
        }
        if i + 2 < n && s[i] == s[i + 2] {
            d3.insert(i);
        }
    }

    for _ in 0..m {
        let q_type = input.read_int();
        let l = input.read_size() - 1;
        let r = input.read_size();
        match q_type {
            1 => {
                let x = input.read_long();
                for i in (l.max(1) - 1)..l {
                    d2.remove(&i);
                }
                for i in (l.max(2) - 2)..l {
                    d3.remove(&i);
                }
                for i in (r.max(1) - 1)..r {
                    d2.remove(&i);
                }
                for i in (r.max(2) - 2)..r {
                    d3.remove(&i);
                }
                tree.update(l..r, &Node(x));
                for i in (l.max(1) - 1)..l {
                    let x1 = *tree.point_query(i);
                    if i + 1 < n && x1 == *tree.point_query(i + 1) {
                        d2.insert(i);
                    }
                }
                for i in (l.max(2) - 2)..l {
                    let x2 = *tree.point_query(i);
                    if i + 2 < n && x2 == *tree.point_query(i + 2) {
                        d3.insert(i);
                    }
                }
                for i in (r.max(1) - 1)..r {
                    let x3 = *tree.point_query(i);
                    if i + 1 < n && x3 == *tree.point_query(i + 1) {
                        d2.insert(i);
                    }
                }
                for i in (r.max(2) - 2)..r {
                    let x4 = *tree.point_query(i);
                    if i + 2 < n && x4 == *tree.point_query(i + 2) {
                        d3.insert(i);
                    }
                }
            }
            2 => {
                let mut ans = true;
                if l + 1 < r && d2.range(l..r - 1).next().is_some() {
                    ans = false;
                }
                if l + 2 < r && d3.range(l..r - 2).next().is_some() {
                    ans = false;
                }
                out.print_line(ans);
            }
            _ => unreachable!(),
        }
    }
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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
