//{"name":"Moortal Cowmbat","group":"Eolymp - Basecamp - Educational Round #5","url":"https://eolymp.com/en/compete/saaq21a9v514tbj0spj7bmgjpk/problem/9","interactive":false,"timeLimit":1000,"tests":[{"input":"5 5 2\nabcde\n0 1 4 4 4\n2 0 4 4 4\n6 5 0 3 2\n5 5 5 0 4\n3 7 0 5 0\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let s = input.read_str();
    let mut a = input.read_long_table(m, m);

    for i in 0..m {
        for j in 0..m {
            for p in 0..m {
                let cand = a[(j, i)] + a[(i, p)];
                a[(j, p)].minim(cand);
            }
        }
    }
    let cost = Vec::with_gen(m, |i| {
        Vec::with_gen(n, |j| {
            let from = s[j] as usize - b'a' as usize;
            a[(from, i)]
        })
    });
    let sum_cost = Vec::with_gen(m, |i| cost[i].partial_sums());
    let mut mem = Memoization2d::new(n + 1, m, |mem, pos, last| {
        if pos == n {
            0
        } else {
            let mut res = i64::MAX;
            if pos != 0 {
                res.minim(mem.call(pos + 1, last) + cost[last][pos]);
            }
            if pos + k <= n {
                for nxt in 0..m {
                    res.minim(mem.call(pos + k, nxt) + sum_cost[nxt][pos + k] - sum_cost[nxt][pos]);
                }
            }
            res
        }
    });
    out.print(mem.call(0, 0));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
