//{"name":"F. The more KEP, the better","group":"KEP.uz","url":"https://kep.uz/competitions/contests/contest/452/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"kepkep\n0\n","output":"2\n"},{"input":"kdp\n1\n","output":"1\n"},{"input":"keokdpzz\n2\n","output":"2\n"},{"input":"abcdef\n10\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::usize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let mut k = input.read_size();

    let n = s.len();
    if n < 3 {
        out.print_line(0);
        return;
    }
    fn dist(a: u8, b: u8) -> usize {
        if a <= b {
            (b - a) as usize
        } else {
            (b + 26 - a) as usize
        }
    }
    let cost = Vec::with_gen(n - 2, |i| {
        dist(s[i], b'k') + dist(s[i + 1], b'e') + dist(s[i + 2], b'p')
    });
    let mut rec = RecursiveFunction2::new(|rec, from: usize, to: usize| -> Arr2d<Vec<usize>> {
        if from + 1 == to {
            let mut res = Arr2d::new(3, 3, vec![]);
            res[(0, 2)] = vec![cost[from]];
            res
        } else {
            let mut ret = Arr2d::new(3, 3, vec![]);
            let mid = (from + to) / 2;
            let left = rec.call(from, mid);
            let right = rec.call(mid, to);
            for i in 0..3 {
                for j in 0..3 {
                    if from + i > to + j {
                        continue;
                    }
                    let mut cur = vec![0];
                    for k in 0..3 {
                        if from + i > mid + k || mid + k > to + j {
                            continue;
                        }
                        let left = &left[(i, k)];
                        let right = &right[(k, j)];
                        let mut cand = left.clone();
                        cand.extend_from_slice(right);
                        cand.sort();
                        let mut sum = 0;
                        for i in cand.indices() {
                            if cur.len() <= i + 1 {
                                cur.push(usize::MAX);
                            }
                            sum += cand[i];
                            cur[i + 1].minim(sum);
                        }
                    }
                    let mut res = Vec::with_capacity(cur.len() - 1);
                    for (a, b) in cur.consecutive_iter_copy() {
                        res.push(b - a);
                    }
                    ret[(i, j)] = res;
                }
            }
            ret
        }
    });
    let ans = rec.call(0, n - 2);
    let val = &ans[(0, 2)];
    let mut res = 0;
    for &i in val {
        if k >= i {
            k -= i;
            res += 1;
        } else {
            break;
        }
    }
    out.print_line(res);
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
