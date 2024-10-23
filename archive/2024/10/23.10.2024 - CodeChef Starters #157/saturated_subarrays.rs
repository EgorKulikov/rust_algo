//{"name":"Saturated Subarrays","group":"CodeChef - START157A","url":"https://www.codechef.com/START157A/problems/SATSUB","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 3\n17 -15 3 18\n2 3\n1 4\n2 4\n10 3\n8 5 7 20 -18 12 -13 -3 -6 -16\n5 7\n3 8\n1 10\n","output":"1\n5\n3\n1\n4\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SaturatedSubarrays"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);
    let queries = input.read_size_pair_vec(q).dec();

    #[derive(Default, Clone)]
    struct Node {
        value: i64,
        mult: i64,
        delta: i64,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.value = left_val.value + right_val.value;
            self.mult = left_val.mult + right_val.mult;
        }

        fn accumulate(&mut self, value: &Self) {
            self.value += self.mult * value.delta;
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    impl Pushable<i64> for Node {
        fn push(&mut self, delta: i64) {
            self.mult += delta;
        }
    }

    let mut st = SegmentTree::<Node>::new(n + 1);
    let mut by_r = vec![Vec::new(); n + 1];
    for (i, (l, r)) in queries.into_iter().enumerate() {
        by_r[r + 1].push((l, i));
    }
    let s = a.partial_sums();
    let mut mins = vec![0];
    let mut maxs = vec![0];
    let mut ans = vec![0; q];
    st.point_update(0, 1);
    for i in 1..=n {
        while let Some(&max) = maxs.last() {
            if s[max] > s[i] {
                break;
            }
            maxs.pop();
        }
        let from = maxs.last().map(|&x| x + 1).unwrap_or(0);
        st.update(
            from..=i,
            &Node {
                delta: 1,
                ..Default::default()
            },
        );
        maxs.push(i);
        for &(l, j) in by_r[i].iter() {
            let res = st.query(l..=i).value;
            ans[j] = res;
        }
        while let Some(&min) = mins.last() {
            if s[min] <= s[i] {
                break;
            }
            st.point_update(min, -1);
            mins.pop();
        }
        mins.push(i);
        st.point_update(i, 1);
    }
    out.print_per_line(&ans);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
