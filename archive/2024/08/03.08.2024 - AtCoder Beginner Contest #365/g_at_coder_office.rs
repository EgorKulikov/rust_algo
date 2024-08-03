//{"name":"G - AtCoder Office","group":"AtCoder - Toyota Programming Contest 2024#8（AtCoder Beginner Contest 365）","url":"https://atcoder.jp/contests/abc365/tasks/abc365_g","interactive":false,"timeLimit":5000,"tests":[{"input":"3 8\n10 1\n20 2\n30 1\n40 3\n60 3\n70 1\n80 2\n90 1\n3\n1 2\n1 3\n2 3\n","output":"20\n0\n20\n"},{"input":"10 20\n10257 9\n10490 4\n19335 1\n25893 5\n32538 9\n33433 3\n38522 9\n40629 9\n42896 5\n52106 1\n53024 3\n55610 5\n56721 9\n58286 9\n63128 3\n70513 3\n70977 4\n74936 5\n79883 9\n95116 9\n7\n1 3\n3 9\n1 9\n4 9\n1 5\n5 9\n3 5\n","output":"18673\n2107\n15310\n25720\n17003\n10317\n16848\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAtCoderOffice"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::value_ref;
use std::collections::HashMap;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let entries = input.read_vec::<(i64, usize)>(m).dec();

    value_ref!(TimesContainer TIME: Vec<i64> = entries.iter().map(|&(t, _)| t).collect());
    let mut times = vec![Vec::new(); n];
    for (i, &(_, p)) in entries.iter().enumerate() {
        times[p].push(i);
    }
    const BUBEN: usize = 1000;

    #[derive(Clone)]
    struct Node {
        len: i64,
        enabled: bool,
    }
    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                len: 0,
                enabled: false,
            }
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            self.len = left_val.len + right_val.len;
        }

        fn accumulate(&mut self, value: &Self, left: usize, right: usize) {
            if value.enabled {
                self.enabled = true;
                self.len = TimesContainer::val()[right] - TimesContainer::val()[left];
            }
        }

        fn reset_delta(&mut self, _left: usize, _right: usize) {
            self.enabled = false;
        }
    }
    let mut res = vec![SegmentTree::<Node>::new(1); n];
    for i in 0..n {
        if times[i].len() >= BUBEN {
            res[i] = SegmentTree::new(m);
            for j in times[i].indices().step_by(2) {
                res[i].update(
                    times[i][j]..times[i][j + 1],
                    &Node {
                        len: 0,
                        enabled: true,
                    },
                );
            }
        }
    }
    let mut cache = HashMap::new();

    let q = input.read_size();
    for _ in 0..q {
        let mut a = input.read_size() - 1;
        let mut b = input.read_size() - 1;

        if times[b].len() >= BUBEN {
            swap(&mut a, &mut b);
        }

        if let Some(&ans) = cache.get(&(a, b)) {
            out.print_line(ans);
            continue;
        }

        let mut ans = 0;

        if times[a].len() >= BUBEN {
            for j in times[b].indices().step_by(2) {
                let l = times[b][j];
                let r = times[b][j + 1];
                let res = res[a].query(l..r);
                ans += res.len;
            }
        } else {
            let mut a_enabled = false;
            let mut b_enabled = false;

            let mut at_a = 0;
            let mut at_b = 0;
            while at_a < times[a].len() && at_b < times[b].len() {
                let cur_time = TimesContainer::val()[times[a][at_a].min(times[b][at_b])];
                if a_enabled && b_enabled {
                    ans += cur_time;
                }
                if times[a][at_a] < times[b][at_b] {
                    at_a += 1;
                    a_enabled = !a_enabled;
                } else {
                    at_b += 1;
                    b_enabled = !b_enabled;
                }
                if a_enabled && b_enabled {
                    ans -= cur_time;
                }
            }
        }

        out.print_line(ans);
        cache.insert((a, b), ans);
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
    let test_type = TestType::Single;
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
