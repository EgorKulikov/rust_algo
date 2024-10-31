//{"name":"K. Farm Management","group":"Universal Cup - The 3rd Universal Cup. Stage 14: Harbin","url":"https://contest.ucup.ac/contest/1817/problem/9529","interactive":false,"timeLimit":1000,"tests":[{"input":"5 17\n2 3 4\n6 1 5\n8 2 4\n4 3 3\n7 5 5\n","output":"109\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KFarmManagement"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let jobs = input.read_vec::<(usize, usize, usize)>(n).sorted();

    struct Node {
        min: usize,
        max: usize,
        base_profit: usize,
        add_profit: usize,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Node {
                min: 0,
                max: 0,
                base_profit: 0,
                add_profit: 0,
            }
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.min = left_val.min + right_val.min;
            self.max = left_val.max + right_val.max;
            self.base_profit = left_val.base_profit + right_val.base_profit;
            self.add_profit = left_val.add_profit + right_val.add_profit;
        }

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    let mut ans = 0;
    let mut rem = m;
    let mut sum_r = 0;
    for i in 0..n - 1 {
        let (w, l, r) = jobs[i];
        ans += w * l;
        rem -= l;
        sum_r += r;
    }
    ans += jobs[n - 1].0 * rem;
    sum_r += jobs[n - 1].2;

    let mut st = SegmentTree::from_generator(n, |i| {
        let (w, l, r) = jobs[i];
        Node {
            min: l,
            max: r,
            base_profit: w * l,
            add_profit: w * (r - l),
        }
    });
    for i in 0..n - 1 {
        let x = m.saturating_sub(sum_r - jobs[i].2);
        st.point_update(
            i,
            Node {
                min: x,
                max: x,
                base_profit: x * jobs[i].0,
                add_profit: 0,
            },
        );
        let mut left = 0;
        let mut right = 0;
        let mut res = 0;
        let mut rem = m;
        let add = st.binary_search(
            |l, r| {
                let cur_left = left + l.min;
                let cur_right = right + r.max;
                if cur_left + cur_right > m {
                    res += l.base_profit;
                    left += l.min;
                    rem -= l.min;
                    Direction::Right
                } else {
                    res += r.base_profit + r.add_profit;
                    right += r.max;
                    rem -= r.max;
                    Direction::Left
                }
            },
            |_, pos| jobs[pos].0,
        );
        res += add * rem;
        ans.maxim(res);
        st.point_update(
            i,
            Node {
                min: jobs[i].1,
                max: jobs[i].2,
                base_profit: jobs[i].0 * jobs[i].1,
                add_profit: jobs[i].0 * (jobs[i].2 - jobs[i].1),
            },
        )
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
