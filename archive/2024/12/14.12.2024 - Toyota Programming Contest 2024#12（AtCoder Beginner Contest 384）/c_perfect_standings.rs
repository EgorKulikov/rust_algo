//{"name":"C - Perfect Standings","group":"AtCoder - Toyota Programming Contest 2024#12（AtCoder Beginner Contest 384）","url":"https://atcoder.jp/contests/abc384/tasks/abc384_c","interactive":false,"timeLimit":2000,"tests":[{"input":"400 500 600 700 800\n","output":"ABCDE\nBCDE\nACDE\nABDE\nABCE\nABCD\nCDE\nBDE\nADE\nBCE\nACE\nBCD\nABE\nACD\nABD\nABC\nDE\nCE\nBE\nCD\nAE\nBD\nAD\nBC\nAC\nAB\nE\nD\nC\nB\nA\n"},{"input":"800 800 900 900 1000\n","output":"ABCDE\nACDE\nBCDE\nABCE\nABDE\nABCD\nCDE\nACE\nADE\nBCE\nBDE\nABE\nACD\nBCD\nABC\nABD\nCE\nDE\nAE\nBE\nCD\nAC\nAD\nBC\nBD\nAB\nE\nC\nD\nA\nB\n"},{"input":"128 256 512 1024 2048\n","output":"ABCDE\nBCDE\nACDE\nCDE\nABDE\nBDE\nADE\nDE\nABCE\nBCE\nACE\nCE\nABE\nBE\nAE\nE\nABCD\nBCD\nACD\nCD\nABD\nBD\nAD\nD\nABC\nBC\nAC\nC\nAB\nB\nA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPerfectStandings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let score = input.read_size_vec(5);

    let mut ans = Vec::new();
    for i in 1..32 {
        let mut cur = 0;
        let mut name = Str::new();
        for j in 0..5 {
            if i.is_set(j) {
                cur += score[j];
                name.push(b'A' + j as u8);
            }
        }
        ans.push((Reverse(cur), name));
    }
    ans.sort();
    out.print_per_line_iter(ans.into_iter().map(|x| x.1));
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
