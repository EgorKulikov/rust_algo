//{"name":"A - Multiples in the String","group":"AtCoder - AtCoder Grand Contest 070","url":"https://atcoder.jp/contests/agc070/tasks/agc070_a","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMultiplesInTheString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::unsigned_big_int::UBigInt;
use algo_lib::string::str::Str;
use algo_lib::string::string_algorithms::string_search::StringSearch;

type PreCalc = ();

fn solve(_input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut p = UBigInt::one();
    for _ in 0..1018 {
        p *= 10;
    }
    p -= UBigInt::one();
    p /= 1019;
    let ans = Str::from(format!("000{}000{}", p.to_string(), p.to_string()).as_bytes());
    for i in 1..=1000 {
        let mut q = p.clone();
        q *= i;
        let s = Str::from(q.to_string().as_bytes());
        if !ans.str_contains(&s) {
            eprintln!("Not found {}", i);
        }
    }
    out.print_line(p);
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
