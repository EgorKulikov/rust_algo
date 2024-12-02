//{"name":"T494318 「CZOI-R2」迷宫","group":"Luogu","url":"https://www.luogu.com.cn/problem/T494318?contestId=203857","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n><>\n>vv\n^<<\n1 2 3\n4 5 6\n7 8 9\n","output":"3946712175731523781\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T494318CZOIR2"}}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let dir = input.read_char_table(n, m);
    let mut v = input.read_long_table(n, m);

    for i in 0..n {
        for j in 0..m {
            if (i + j) % 2 == 1 {
                v[(i, j)] *= -1;
            }
        }
    }
    let mut ans = 0u64;
    for id in 0..2 {
        let mut res = Arr2d::new(n, m, None);

        for i in 0..n {
            for j in 0..m {
                if res[(i, j)].is_some() {
                    continue;
                }
                let mut visited = FxHashMap::default();
                let mut stack = Vec::new();
                let mut r = i;
                let mut c = j;
                enum EndType {
                    Cycle,
                    Known,
                    Out,
                }
                let mut end = EndType::Out;
                for k in 0.. {
                    if visited.contains_key(&(r, c)) {
                        end = EndType::Cycle;
                        break;
                    }
                    if res[(r, c)].is_some() {
                        end = EndType::Known;
                        break;
                    }
                    visited.insert((r, c), k);
                    stack.push((r, c));
                    match dir[(r, c)] {
                        b'>' => {
                            c += 1;
                            if c == m {
                                break;
                            }
                        }
                        b'<' => {
                            if c == 0 {
                                break;
                            }
                            c -= 1;
                        }
                        b'^' => {
                            if r == 0 {
                                break;
                            }
                            r -= 1;
                        }
                        b'v' => {
                            r += 1;
                            if r == n {
                                break;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                let mut cur = match end {
                    EndType::Known => res[(r, c)].unwrap(),
                    EndType::Out => 0,
                    EndType::Cycle => {
                        let at = visited[&(r, c)];
                        let mut vvs = Vec::new();
                        for (r, c) in stack.copy_skip(at) {
                            vvs.push(v[(r, c)]);
                        }
                        for (r, c) in stack.copy_skip(at) {
                            vvs.push(v[(r, c)]);
                        }
                        let s = vvs.partial_sums();
                        #[derive(Default, Clone)]
                        struct Node {
                            max: i64,
                            min: i64,
                        }
                        impl SegmentTreeNode for Node {
                            fn new(_left: usize, _right: usize) -> Self {
                                Default::default()
                            }

                            fn join(&mut self, left_val: &Self, right_val: &Self) {
                                self.max = left_val.max.max(right_val.max);
                                self.min = left_val.min.min(right_val.min);
                            }

                            fn accumulate(&mut self, _value: &Self) {}

                            fn reset_delta(&mut self) {}
                        }
                        let mut st = SegmentTree::from_generator(s.len(), |i| Node {
                            max: s[i],
                            min: s[i],
                        });
                        let n = stack.len() - at;
                        for i in 0..n {
                            let cur = (st.query(i + 1..=i + n).max - s[i])
                                .max(s[i + n] - st.query(i..i + n).min)
                                .max(0);
                            res[stack[at + i]] = Some(cur);
                        }
                        let out = res[stack[at]].unwrap();
                        stack.truncate(at);
                        out
                    }
                };
                for (r, c) in stack.iter_rev() {
                    cur = v[(r, c)] + cur.max(0);
                    res[(r, c)] = Some(cur);
                }
            }
        }

        let mut mult = 1;
        for i in 0..n {
            for j in 0..m {
                if (i + j) % 2 == id {
                    ans = ans
                        .overflowing_add(
                            (res[(i, j)].unwrap() as u64 + 1_000_000_000_000_000)
                                .overflowing_mul(mult)
                                .0,
                        )
                        .0;
                }
                mult = mult.overflowing_mul(2_000_000_000_000_021).0;
            }
        }

        for i in 0..n {
            for j in 0..m {
                v[(i, j)] *= -1;
            }
        }
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
