//{"name":"B. Segment Trees ?","group":"Codeforces - AlgoChief Sprint Round 3","url":"https://codeforces.com/gym/105705/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n8\n1 2 3 4 5 6 7 8\n8\n2 10\n1 1 6\n1 2 6\n2 10\n1 1 5\n1 2 6\n1 3 7\n1 4 1\n","output":"5 6 7 1 10 10 10 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    #[derive(Default, Clone)]
    struct Node {
        delta: i32,
    }
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            self.delta.maxim(value.delta);
        }
        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }
    let mut st = SegmentTree::with_gen(n, |i| Node { delta: a[i] });

    let q = input.read_size();
    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let x = input.read_size() - 1;
                let p = input.read_int();
                st.point_update(x, Node { delta: p });
            }
            2 => {
                let v = input.read_int();
                st.update(.., &Node { delta: v });
            }
            _ => unreachable!(),
        }
    }
    out.print_line_iter((0..n).map(|i| st.point_query(i).delta));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
