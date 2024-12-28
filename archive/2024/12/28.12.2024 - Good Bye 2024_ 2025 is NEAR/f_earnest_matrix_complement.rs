//{"name":"F. Earnest Matrix Complement","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"9\n3 3 3\n1 2 2\n3 1 3\n3 2 1\n2 3 3\n-1 3 3\n2 2 -1\n3 3 6\n-1 -1 1\n1 2 -1\n-1 -1 4\n3 4 5\n1 3 2 3\n-1 -1 2 -1\n3 1 5 1\n5 3 8\n5 -1 2\n1 8 -1\n-1 5 6\n7 7 -1\n4 4 4\n6 6 5\n-1 -1 5 -1 -1 -1\n-1 -1 -1 -1 2 -1\n-1 1 3 3 -1 -1\n-1 1 -1 -1 -1 4\n4 2 -1 -1 -1 4\n-1 -1 1 2 -1 -1\n6 6 4\n-1 -1 -1 -1 1 -1\n3 -1 2 2 4 -1\n3 1 2 2 -1 -1\n3 3 3 3 -1 2\n-1 3 3 -1 1 3\n3 -1 2 2 3 -1\n5 5 3\n1 1 3 -1 1\n2 2 -1 -1 3\n-1 -1 -1 2 -1\n3 -1 -1 -1 2\n-1 1 2 3 -1\n6 2 7\n-1 7\n-1 6\n7 -1\n-1 -1\n-1 -1\n2 2\n","output":"4\n4\n10\n10\n8\n102\n93\n58\n13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEarnestMatrixComplement"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let a = input.read_int_table(n, m);

    #[derive(Default, Clone)]
    struct Node {
        val: i64,
        at_least: i64,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Default::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.max(right_val.val);
        }

        fn accumulate(&mut self, value: &Self) {
            self.val.maxim(value.at_least);
            self.at_least.maxim(value.at_least);
        }

        fn reset_delta(&mut self) {
            self.at_least = 0;
        }
    }

    impl Pushable<i64> for Node {
        fn push(&mut self, delta: i64) {
            self.val += delta;
        }
    }

    let mut st: SegmentTree<Node> = SegmentTree::new(k);
    let mut bonus = 0;
    for i in 0..n {
        let mut wildcards = 0;
        let mut qty = DefaultHashMap::new(0);
        for j in 0..m {
            if a[(i, j)] == -1 {
                wildcards += 1;
            } else {
                qty[a[(i, j)]] += 1;
            }
        }
        if i > 0 {
            for j in 0..m {
                if a[(i - 1, j)] != -1 {
                    st.point_update(a[(i - 1, j)] as usize - 1, wildcards);
                }
            }
        }
        if i + 1 < n {
            for j in 0..m {
                if a[(i + 1, j)] != -1 {
                    st.point_update(a[(i + 1, j)] as usize - 1, wildcards);
                }
            }
            let mut next_wildcards = 0;
            for j in 0..m {
                if a[(i + 1, j)] == -1 {
                    next_wildcards += 1;
                } else {
                    bonus += qty[a[(i + 1, j)]];
                }
            }
            bonus += wildcards * next_wildcards;
            let max = st.query(..).val;
            st.update(
                ..,
                &Node {
                    at_least: max - wildcards * next_wildcards,
                    val: 0,
                },
            );
        }
    }
    out.print_line(bonus + st.query(..).val);
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
