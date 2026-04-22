//{"name":"Nasser's OR Removal","group":"Eolymp - Basecamp - Blitz Round #18","url":"https://eolymp.com/en/compete/r3hr8hsui169n3libfqfgmb0no/problem/5","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n3 3\n1 2 3\n2 5\n4 7\n4 6\n2 7 4 1\n6 7\n1 3 8 8 5 4\n5 5\n5 5 5 5 5\n","output":"0 3\n-1 -1\n1 2\n2 4\n0 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let a = input.read_int_vec(n);

    #[derive(Default, Copy, Clone)]
    struct Node {
        or: i32,
        len: i32,
        after: i32,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.or = left_val.or | right_val.or;
            self.len = left_val.len + right_val.len;
            self.after = left_val.after + right_val.after;
        }
    }

    let mut nodes = Vec::new();
    let mut len = 0;
    let mut in_len = 0;
    let mut or = 0;
    let mut first = true;
    for i in a {
        if (k & i) == i {
            if len > 0 || first {
                if first {
                    first = false;
                } else {
                    nodes.push(Node {
                        or,
                        after: len,
                        len: in_len,
                    });
                }
                or = 0;
                len = 0;
                in_len = 0;
            }
            or |= i;
            in_len += 1;
        } else {
            len += 1;
        }
    }
    if !first {
        nodes.push(Node {
            or,
            after: len,
            len: in_len,
        });
    }
    if nodes.is_empty() {
        out.print_line((-1, -1));
        return;
    }
    let mut st = SegmentTree::with_gen(nodes.len(), |i| nodes[i]);
    let mut j = 0;
    let mut ans = None;
    for i in nodes.indices() {
        while j < nodes.len() && st.query(i..=j).or != k {
            j += 1;
        }
        if j == nodes.len() {
            break;
        }
        ans.minim((st.query(i..j).after, Reverse(st.query(i..=j).len)));
    }
    if let Some((after, Reverse(len))) = ans {
        out.print_line((after, len));
    } else {
        out.print_line((-1, -1));
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
