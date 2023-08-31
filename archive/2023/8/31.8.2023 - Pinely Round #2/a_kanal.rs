//{"name":"A. Канал","group":"Codeforces - Pinely Round 2 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1863/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5 5 3\n--+\n5 2 3\n++-\n5 4 2\n-+\n5 0 7\n++++-++\n","output":"YES\nNO\nMAYBE\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKanal"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::StrReader;
use algo_lib::{out_line, when};

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size();
    let _q = input.read_size();
    let s = input.read_str();

    let mut cur = a;
    let mut max = a;
    let mut can = a == n;
    let mut should = a == n;
    for c in s {
        if c == b'+' {
            cur += 1;
            max += 1;
            can |= max == n;
            should |= cur == n;
        } else {
            cur -= 1;
        }
    }
    out_line!(when! {
        should => "YES",
        can => "MAYBE",
        else => "NO",
    });
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
