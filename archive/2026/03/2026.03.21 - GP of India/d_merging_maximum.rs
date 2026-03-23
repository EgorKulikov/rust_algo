//{"name":"D. Merging Maximum","group":"Universal Cup - GP of India","url":"https://contest.ucup.ac/contest/3516/problem/17409","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n2 1\n3\n2 1 3\n5\n3 1 4 5 2\n","output":"1\n2\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n);

    type Mod = ModIntF;
    let mut mem = Memoization2d::new(n, 2, |mem, pos, oddity| -> Mod {
        let mut good = (n - pos) % 2 != oddity;
        for j in pos + 1..n {
            if p[j] > p[pos] {
                good = false;
                break;
            }
        }
        let mut res = Mod::new(good as u32);
        if oddity == 1 {
            if pos + 1 == n || p[pos + 1] > p[pos] {
                return Mod::new(0);
            }
        }
        let mut max = 0;
        let mut n_oddity = 0;
        for j in pos + oddity + 1..n {
            if p[j].max(p[pos]) > max && (n_oddity == 0 || p[j] > p[j - 1]) {
                res += mem.call(j, n_oddity);
            }
            max.maxim(p[j]);
            n_oddity ^= 1;
        }
        res
    });
    let mut max = 0;
    let mut ans = Mod::new(0);
    for i in 0..n {
        if p[i] > max {
            ans += mem.call(i, i % 2);
        }
        max.maxim(p[i]);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
