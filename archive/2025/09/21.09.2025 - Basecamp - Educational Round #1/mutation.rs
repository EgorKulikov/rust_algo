//{"name":"Mutation","group":"Eolymp - Basecamp - Educational Round 1","url":"https://basecamp.eolymp.com/en/compete/1v399r4bst3f1apjnuj8as5pbc/problem/8","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n1 2 1 3 1 3 2 4 5\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let g = input.read_size_vec(n).dec();

    let mut last = vec![None; n];
    let mut is_first = BitSet::new(n);
    for i in 0..n {
        if last[g[i]].is_none() {
            is_first.set(i);
        }
        last[g[i]] = Some(i);
    }
    let mut mem = Memoization1d::new(n + 1, |mem, pos| -> usize {
        if pos == n {
            0
        } else {
            let mut res = mem.call(pos + 1);
            if is_first[pos] {
                res.maxim(1 + mem.call(last[g[pos]].unwrap() + 1));
            }
            res
        }
    });
    out.print_line(is_first.count_ones() - mem.call(0));
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
