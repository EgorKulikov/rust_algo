//{"name":"L. The Grand Contest","group":"Universal Cup - The 3rd Universal Cup. Stage 19: Shenyang","url":"https://contest.ucup.ac/contest/1865/problem/9809","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 20\n1 1 60 0\n2 2 60 0\n2 2 120 1\n1 2 180 1\n1 2 180 0\n2 2 300 1\n2 20\n1 1 300 1\n2 2 300 1\n","output":"120 160\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LTheGrandContest"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::compress::{compress, Compressed};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::{PartialSums, UpperDiv};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_long();
    let submissions = input.read_vec::<(usize, usize, i64, usize)>(n);

    let mut score = [0; 2];
    let mut penalty = [0; 2];
    let mut solve_at = vec![Vec::new(); 2];
    let mut solved = vec![FxHashSet::default(); 2];
    let mut submits = vec![DefaultHashMap::<_, i64>::new(); 2];
    for (team, problem, time, verdict) in submissions.copy_iter() {
        let team = team - 1;
        if solved[team].contains(&problem) {
            continue;
        }
        if verdict == 1 {
            penalty[team] += time + submits[team][problem] * p;
            score[team] += 1;
            solved[team].insert(problem);
            solve_at[team].push(time);
        } else {
            submits[team][problem] += 1;
        }
    }
    if score[0] != score[1] || score[0] == 0 {
        out.print_line(-1);
        return;
    }
    let need = if penalty[0] <= penalty[1] {
        penalty[1] - penalty[0] + 1
    } else {
        solve_at.swap(0, 1);
        penalty[0] - penalty[1]
    };

    let s0 = solve_at[0].clone();
    let s1 = solve_at[1].clone();
    let Compressed {
        order: poi,
        arrs: [s0, s1],
    } = compress([&s0, &s1]);
    let mut len = vec![0; poi.len()];
    for i in poi.indices() {
        len[i] = poi[i] - if i > 0 { poi[i - 1] } else { 0 };
    }
    let mut delta = vec![0; poi.len()];
    for (mult, s) in [(-1, s0), (1, s1)] {
        let mut cur = 0;
        let mut x = score[0];
        for i in s {
            for i in cur..=i {
                delta[i] += mult * x;
            }
            x -= 1;
            cur = i + 1;
        }
    }

    let mut ans = None;
    let total = poi[Back(0)];
    for reversed in [false, true] {
        #[derive(Default, Clone)]
        struct Node {
            sum: i64,
            max: i64,
        }
        impl SegmentTreeNode for Node {
            fn new(_left: usize, _right: usize) -> Self {
                Self::default()
            }

            fn join(&mut self, left_val: &Self, right_val: &Self) {
                self.sum = left_val.sum + right_val.sum;
                self.max = left_val.max.max(left_val.sum + right_val.max);
            }

            fn accumulate(&mut self, _value: &Self) {}

            fn reset_delta(&mut self) {}
        }
        let mut st = SegmentTree::<Node>::new(poi.len());
        let s = len.partial_sums();
        for i in poi.indices().rev() {
            st.point_update(
                i,
                Node {
                    sum: delta[i] * len[i],
                    max: (delta[i] * len[i]).max(0),
                },
            );
            if st.query(..).max < need {
                continue;
            }
            let mut need_rem = need;
            let pos = st.binary_search(
                |left, _right| {
                    if left.max >= need_rem {
                        Direction::Left
                    } else {
                        need_rem -= left.sum;
                        Direction::Right
                    }
                },
                |_, pos| pos,
            );
            assert!(delta[pos] > 0);
            let end = s[pos] + need_rem.upper_div(delta[pos]);
            let cur_len = end - s[i];
            let (start, end) = if reversed {
                (total - end, total - s[i])
            } else {
                (s[i], end)
            };
            ans.minim((cur_len, start, end));
        }
        len.reverse();
        delta.reverse();
    }
    if ans.is_none() {
        out.print_line(-1);
        return;
    }
    let (_, mut l, mut r) = ans.unwrap();
    let p0 = poi.lower_bound(&l);
    let p1 = poi.lower_bound(&r);
    if p0 != p1 {
        let p0_prev = if p0 > 0 { poi[p0 - 1] } else { 0 };
        let p1_prev = if p1 > 0 { poi[p1 - 1] } else { 0 };
        let mut total = 0;
        for i in p0 + 1..p1 {
            total += delta[i] * len[i];
        }
        total += (poi[p0] - l) * delta[p0] + (r - p1_prev) * delta[p1];
        let max = (poi[p0] - p0_prev).min(poi[p1] - p1_prev);
        let cur = delta[p1] - delta[p0];
        assert!(cur > 0);
        let shift = ((total - need) / cur).min(max);
        l -= shift;
        r -= shift;
    }
    out.print_line((l, r));
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
