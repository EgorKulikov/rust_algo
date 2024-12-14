//{"name":"E - Takahashi is Slime 2","group":"AtCoder - Toyota Programming Contest 2024#12（AtCoder Beginner Contest 384）","url":"https://atcoder.jp/contests/abc384/tasks/abc384_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3 2\n2 2\n14 6 9\n4 9 20\n17 15 7\n","output":"28\n"},{"input":"3 4 1\n1 1\n5 10 1 1\n10 1 1 1\n1 1 1 1\n","output":"5\n"},{"input":"8 10 2\n1 5\n388 130 971 202 487 924 247 286 237 316\n117 166 918 106 336 928 493 391 235 398\n124 280 425 955 212 988 227 222 307 226\n336 302 478 246 950 368 291 236 170 101\n370 200 204 141 287 410 388 314 205 460\n291 104 348 337 404 399 416 263 415 339\n105 420 302 334 231 481 466 366 401 452\n119 432 292 403 371 417 351 231 482 184\n","output":"1343\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETakahashiIsSlime2"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let x = input.read_size();
    let p = input.read_size() - 1;
    let q = input.read_size() - 1;
    let s = input.read_size_table(h, w);

    let mut added = Arr2d::new(h, w, false);
    added[(p, q)] = true;
    let mut score = s[(p, q)];
    let mut queue = vec![(p, q)];
    let mut candidates = BinaryHeap::new();
    while let Some((r, c)) = queue.pop() {
        for (nr, nc) in D4::iter(r, c, h, w) {
            if !added[(nr, nc)] {
                added[(nr, nc)] = true;
                candidates.push((Reverse(s[(nr, nc)]), nr, nc));
            }
        }
        while let Some((Reverse(v), r, c)) = candidates.peek().copied() {
            if v.saturating_mul(x) >= score {
                break;
            }
            candidates.pop();
            score += v;
            queue.push((r, c));
        }
    }
    out.print_line(score);
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
