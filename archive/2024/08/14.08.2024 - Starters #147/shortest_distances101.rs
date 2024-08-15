//{"name":"Shortest Distances 101","group":"CodeChef - START147A","url":"https://www.codechef.com/START147A/problems/DATA101","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n7 5\n5 2 1 1 0 1 0\n1 7\n1 2\n2 6\n2 5\n6 7\n5 4\n1 3 0 1 0\n1 5\n2 4\n4 5\n3 4\n","output":"2\n1\n-1\n2\n1\n2\n1\n1\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ShortestDistances101"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    #[derive(Clone)]
    struct Node((usize, usize));
    impl SegmentTreeNode for Node {
        fn new(left: usize, _right: usize) -> Self {
            Self((left, left))
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            self.0 = left_val.0.max(right_val.0);
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    impl Pushable<&usize> for Node {
        fn push(&mut self, delta: &usize, left: usize, _right: usize) {
            self.0.maxim((*delta, left));
        }
    }

    let mut tree = SegmentTree::<Node>::new(n);
    let mut tos = vec![(0..n).map(|i| (i, i)).collect_vec(); 20];

    for i in (0..n).rev() {
        let mut to = tree.query(i..=(i + a[i])).0 .1;
        let mut end = i + a[i];
        for t in &mut tos {
            t[i] = (to, end);
            (to, end) = t[to];
        }
        tree.point_update(i, &(i + a[i]));
    }

    for _ in 0..q {
        let x = input.read_size() - 1;
        let y = input.read_size() - 1;
        let mut ans = 0;
        let mut at = x;
        for i in (0..20).rev() {
            let (can, end) = tos[i][at];
            if can < y {
                ans += 1 << i;
                if end >= y {
                    at = y;
                    break;
                }
                at = can;
            }
        }
        if at == y {
            out.print_line(ans);
        } else if a[at] + at >= y {
            out.print_line(ans + 1);
        } else {
            out.print_line(-1);
        }
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
