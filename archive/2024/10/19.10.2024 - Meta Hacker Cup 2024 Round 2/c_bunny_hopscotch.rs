//{"name":"C: Bunny Hopscotch","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-2/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n1 3 3\n1 1 2\n1 4 12\n1 2 3 4\n2 2 5\n1 2\n2 1\n2 3 17\n1 1 2\n1 2 2\n","output":"Case #1: 2\nCase #2: 3\nCase #3: 1\nCase #4: 2\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"bunny_hopscotch_.*input[.]txt"},"output":{"type":"file","fileName":"bunny_hopscotch_output.txt","pattern":null},"languages":{"java":{"taskClass":"CBunnyHopscotch"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

// use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
// use algo_lib::collections::segment_tree::{Pushable, QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let r = input.read_size();
    let c = input.read_size();
    let k = input.read_size();
    let b = input.read_int_table(r, c);
    drop(input);

    let mut bad = vec![0; r.max(c) + 1];
    let mut by_color = vec![Vec::new(); r * c + 1];
    for i in 0..r {
        for j in 0..c {
            by_color[b[(i, j)] as usize].push((i, j));
        }
    }
    let mut big_colors = Vec::new();
    let mut bit_qty = vec![Arr2d::new(0, 0, 0); r * c + 1];
    for i in 1..=r * c {
        if by_color[i].len() > r.max(c) {
            let mut qty = Arr2d::new(r + 1, c + 1, 0);
            for x in 0..r {
                for y in 0..c {
                    qty[(x + 1, y + 1)] = qty[(x, y + 1)] + qty[(x + 1, y)] - qty[(x, y)]
                        + (b[(x, y)] == i as i32) as i32;
                }
            }
            big_colors.push(i);
            bit_qty[i] = qty;
        } else {
            for j in by_color[i].indices() {
                let (x1, y1) = by_color[i][j];
                for k in 0..j {
                    let (x2, y2) = by_color[i][k];
                    let d = x1.abs_diff(x2).max(y1.abs_diff(y2));
                    bad[d] += 2;
                }
            }
        }
    }

    let mut left = 1;
    let mut right = r.max(c);
    while left < right {
        let mid = (left + right) / 2;
        let mut total = 0;
        for i in 0..r {
            let fi = i.saturating_sub(mid);
            let ti = (i + mid + 1).min(r);
            for j in 0..c {
                let fj = j.saturating_sub(mid);
                let tj = (j + mid + 1).min(c);
                total += (ti - fi) * (tj - fj) - 1;
            }
        }
        for i in 0..=mid {
            total -= bad[i];
        }
        if total < k {
            left = mid + 1;
            continue;
        }
        for &i in &big_colors {
            for &(x, y) in &by_color[i] {
                let fi = x.saturating_sub(mid);
                let ti = (x + mid + 1).min(r);
                let fj = y.saturating_sub(mid);
                let tj = (y + mid + 1).min(c);
                let qty = bit_qty[i][(ti, tj)] - bit_qty[i][(fi, tj)] - bit_qty[i][(ti, fj)]
                    + bit_qty[i][(fi, fj)];
                total -= qty as usize - 1;
            }
        }
        if total >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    /*#[derive(Default, Clone)]
    struct Node {
        qty: DefaultHashMap<i32, usize>,
    }
    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(&mut self, _left_val: &Self, _right_val: &Self) {}

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }
    impl Pushable<&i32> for Node {
        fn push(&mut self, delta: &i32) {
            self.qty[*delta] += 1;
        }
    }
    impl QueryResult<usize, i32> for Node {
        fn empty_result(_args: &i32) -> usize {
            0
        }

        fn result(&self, args: &i32) -> usize {
            self.qty[*args]
        }

        fn join_results(
            left_res: usize,
            right_res: usize,
            _args: &i32,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> usize {
            left_res + right_res
        }
    }
    let mut rows = vec![SegmentTree::<Node>::new(c); r];
    let mut cols = vec![SegmentTree::<Node>::new(r); c];
    for i in 0..r {
        for j in 0..c {
            rows[i].point_through_update(j, &b[i][j]);
            cols[j].point_through_update(i, &b[i][j]);
        }
    }
    let mut ans = 0;
    'outer: for cur in 1..=r.max(c) {
        for i in 0..r {
            for j in 0..c {
                if i + cur < r {
                    let from = j.saturating_sub(cur);
                    let to = (j + cur + 1).min(c);
                    let delta = to - from - rows[i + cur].query_with_args(from..to, &b[(i, j)]);
                    if delta >= k {
                        ans = cur;
                        break 'outer;
                    }
                    k -= delta;
                }
                if i >= cur {
                    let from = j.saturating_sub(cur);
                    let to = (j + cur + 1).min(c);
                    let delta = to - from - rows[i - cur].query_with_args(from..to, &b[(i, j)]);
                    if delta >= k {
                        ans = cur;
                        break 'outer;
                    }
                    k -= delta;
                }
                if j + cur < c {
                    let from = i.saturating_sub(cur - 1);
                    let to = (i + cur).min(r);
                    let delta = to - from - cols[j + cur].query_with_args(from..to, &b[(i, j)]);
                    if delta >= k {
                        ans = cur;
                        break 'outer;
                    }
                    k -= delta;
                }
                if j >= cur {
                    let from = i.saturating_sub(cur - 1);
                    let to = (i + cur).min(r);
                    let delta = to - from - cols[j - cur].query_with_args(from..to, &b[(i, j)]);
                    if delta >= k {
                        ans = cur;
                        break 'outer;
                    }
                    k -= delta;
                }
            }
        }
    }*/

    out.print_line((format!("Case #{}:", test_case), left));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    output.flush();
    is_exhausted
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
