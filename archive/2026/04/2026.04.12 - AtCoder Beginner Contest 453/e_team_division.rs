//{"name":"E - Team Division","group":"AtCoder - AtCoder Beginner Contest 453","url":"https://atcoder.jp/contests/abc453/tasks/abc453_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 1\n1 2\n2 2\n","output":"2\n"},{"input":"6\n1 5\n1 5\n2 5\n1 3\n3 5\n2 5\n","output":"30\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let lr = input.read_size_pair_vec(n);

    let mut bl = vec![Vec::new(); n];
    let mut br = vec![Vec::new(); n];
    for (i, (l, r)) in lr.iter_enumerate() {
        bl[l].push(i);
        br[r].push(i);
    }
    let mut none = n;
    let mut only_first = 0;
    let mut only_second = 0;
    let mut either = 0;
    let mut in_first = vec![false; n];
    let mut in_second = vec![false; n];

    type Mod = ModIntF;
    let mut ans = Mod::new(0);
    let c = Combinations::<Mod>::new(n + 1);
    for i in 1..n {
        for j in bl[i].copy_iter() {
            if in_second[j] {
                only_second -= 1;
                either += 1;
            } else {
                none -= 1;
                only_first += 1;
            }
            in_first[j] = true;
        }
        for j in br[n - i].copy_iter() {
            if in_first[j] {
                only_first -= 1;
                either += 1;
            } else {
                none -= 1;
                only_second += 1;
            }
            in_second[j] = true;
        }
        if none == 0 && only_first <= i && only_second <= n - i {
            ans += c.c(either, i - only_first);
        }
        for j in br[i].copy_iter() {
            if in_second[j] {
                either -= 1;
                only_second += 1;
            } else {
                none += 1;
                only_first -= 1;
            }
            in_first[j] = false;
        }
        for j in bl[n - i].copy_iter() {
            if in_first[j] {
                only_first += 1;
                either -= 1;
            } else {
                none += 1;
                only_second -= 1;
            }
            in_second[j] = false;
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
