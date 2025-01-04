//{"name":"D. Заказ мерча","group":"Codeforces - Hello 2025","url":"https://codeforces.com/contest/2057/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2\n1 10\n1 10\n2 2\n5 3\n1 2 3 4 5\n3 7\n1 4\n5 2\n8 5\n7 4 2 4 8 2 1 4\n5 4\n1 10\n3 2\n8 11\n7 7\n","output":"8\n0\n7\n0\n4\n4\n4\n5\n3\n6\n6\n9\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DZakazMercha"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    #[derive(Clone, Default)]
    struct Node {
        max_r: i64,
        max_l: i64,
        min_r: i64,
        min_l: i64,
        ans: i64,
    }

    impl Node {
        fn single(a: i64, pos: i64) -> Self {
            Self {
                max_r: a - pos,
                max_l: a + pos,
                min_r: a + pos,
                min_l: a - pos,
                ans: 0,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.max_r = left_val.max_r.max(right_val.max_r);
            self.max_l = left_val.max_l.max(right_val.max_l);
            self.min_r = left_val.min_r.min(right_val.min_r);
            self.min_l = left_val.min_l.min(right_val.min_l);
            self.ans = left_val
                .ans
                .max(right_val.ans)
                .max(right_val.max_r - left_val.min_l)
                .max(left_val.max_l - right_val.min_r);
        }
    }

    let mut st = SegmentTree::gen(n, |i| Node::single(a[i], i as i64));
    out.print_line(st.query(..).ans);
    for _ in 0..q {
        let p = input.read_size() - 1;
        let x = input.read_long();
        st.point_update(p, Node::single(x, p as i64));
        out.print_line(st.query(..).ans);
    }
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
