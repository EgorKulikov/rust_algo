//{"name":"D - Takahashi's Expectation","group":"AtCoder - AtCoder Beginner Contest 417","url":"https://atcoder.jp/contests/abc417/tasks/abc417_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 1 4\n1 5 9\n2 6 5\n3 5 8\n11\n0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n","output":"6\n0\n0\n0\n5\n6\n0\n0\n0\n0\n0\n"},{"input":"3\n500 500 500\n500 500 500\n500 500 500\n1\n1000000000\n","output":"999998500\n"},{"input":"20\n124 370 105\n280 200 420\n425 204 302\n435 141 334\n212 287 231\n262 410 481\n227 388 466\n222 314 366\n307 205 401\n226 460 452\n336 291 119\n302 104 432\n478 348 292\n246 337 403\n102 404 371\n368 399 417\n291 416 351\n236 263 231\n170 415 482\n101 339 184\n20\n1162\n1394\n1695\n2501\n3008\n3298\n4053\n4093\n4330\n5199\n5302\n5869\n5875\n6332\n6567\n7483\n7562\n7725\n9723\n9845\n","output":"339\n339\n339\n339\n339\n339\n339\n339\n339\n339\n339\n339\n339\n389\n339\n643\n722\n885\n2883\n3005\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pab: Vec<(usize, usize, usize)> = input.read_vec(n);

    let mut mem = Memoization2d::new(n + 1, 1001, |mem, pos: usize, mood: usize| -> usize {
        if pos == n {
            mood
        } else {
            let (p, a, b) = pab[pos];
            if mood <= p {
                mem.call(pos + 1, mood + a)
            } else {
                mem.call(pos + 1, mood.saturating_sub(b))
            }
        }
    });
    let b = pab
        .copy_map(|(_, _, b)| b)
        .collect::<Vec<_>>()
        .partial_sums();

    let q = input.read_size();
    for _ in 0..q {
        let x = input.read_size();
        if x <= 1000 {
            out.print_line(mem.call(0, x));
        } else if x >= b[n] + 1000 {
            out.print_line(x - b[n]);
        } else {
            let pos = b.lower_bound(&(x - 1000));
            out.print_line(mem.call(pos, x - b[pos]));
        }
    }
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
