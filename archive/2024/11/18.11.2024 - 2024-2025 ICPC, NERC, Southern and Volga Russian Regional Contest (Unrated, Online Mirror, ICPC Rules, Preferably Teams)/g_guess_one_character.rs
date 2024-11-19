//{"name":"G. Guess One Character","group":"Codeforces - 2024-2025 ICPC, NERC, Southern and Volga Russian Regional Contest (Unrated, Online Mirror, ICPC Rules, Preferably Teams)","url":"https://codeforces.com/contest/2038/problem/G","interactive":true,"timeLimit":2000,"tests":[{"input":"3     // 3 test cases\n3     // the length of the string is 3\n\n1     // 101 occurs once\n\n1     // guessed correctly\n2     // the length of the string is 2\n\n0     // 00 occurs zero times\n\n0     // 0 occurs zero times\n\n1     // guessed correctly\n2     // the length of the string is 2\n\n1     // 1 occurs once\n\n0     // 01 occurs zero times\n\n1     // guessed correctly\n","output":"\n\n1 101 // how many times 101 occurs\n\n0 2 0 // guess: s[2] is 0\n\n\n1 00  // how many times 00 occurs\n\n1 0   // how many times 0 occurs\n\n0 1 1 // guess: s[1] is 1\n\n\n1 1   // how many times 1 occurs\n\n1 01  // how many times 01 occurs\n\n0 2 0 // guess: s[2] is 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GGuessOneCharacter"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();

    out.print_line((1, "0"));
    out.flush();
    let q0 = input.read_size();
    out.print_line((1, "00"));
    out.flush();
    let q00 = input.read_size();
    out.print_line((1, "10"));
    out.flush();
    let q10 = input.read_size();
    let ans = if q0 == q00 + q10 { 1 } else { 0 };
    out.print_line((0, 1, ans));
    out.flush();
    input.read_int();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
