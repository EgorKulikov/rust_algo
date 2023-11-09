//{"name":"A. Cyclicity in art","group":"Yandex - Yandex Cup 2023 — Algorithm — Semifinal","url":"https://contest.yandex.com/contest/54740/problems/A/","interactive":false,"timeLimit":1000,"tests":[{"input":"6 0\n","output":"Yes\naaaaaa\n"},{"input":"3 2\n","output":"No\n"},{"input":"6 4\n","output":"Yes\naabbaaabba\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACyclicityInArt"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read_size();
    let b = input.read_size();

    out.set_bool_output(BoolOutput::YesNo);
    if a != 6 {
        out.print_line(false);
        return;
    }
    if b == 1 {
        out.print_line(false);
        return;
    }
    let mut add = vec![0; 6];
    if b % 2 == 0 {
        add[0] = b / 2;
        add[3] = b / 2;
    } else {
        add[0] = b / 2;
        add[3] = b / 2 - 1;
        add[2] = 1;
        add[4] = 1;
    }
    let mut ans = Str::with_capacity(a + b);
    for i in add {
        ans += b'a';
        for _ in 0..i {
            ans += b'b';
        }
    }
    out.print_line(true);
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
