//{"name":"I. Dumb Problem II","group":"Universal Cup - GP of Zhengzhou","url":"https://contest.ucup.ac/contest/2661/problem/15309","interactive":false,"timeLimit":1000,"tests":[{"input":"1 1\n","output":"1\n"},{"input":"3 2\n","output":"388206139\n"},{"input":"3 4\n","output":"408232647\n"},{"input":"112 646\n","output":"928854225\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    type Mod = ModIntF;
    let mut p = vec![Mod::one(); k + 1];
    for i in 2..=n {
        let p1 = Mod::one() / i;
        let p2 = Mod::from(i - 1) / i;
        let mut c1 = Mod::one();
        let mut c2 = Mod::one();
        for j in 1..=k {
            c1 *= p1;
            c2 *= p2;
            p[j] *= c1 + c2;
        }
    }
    let c = Combinations::<Mod>::new(k + 1);
    for i in (1..k).rev() {
        p[i] *= c.c(k, i);
        for j in i + 1..=k {
            let v = p[j] * c.c(j, i);
            p[i] -= v;
        }
    }
    let mut ans = Mod::from(k);
    for i in 2..=k {
        ans -= p[i] * (i - 1);
    }
    out.print_line(ans);
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
