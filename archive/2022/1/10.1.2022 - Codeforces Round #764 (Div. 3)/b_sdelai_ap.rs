//{"name":"B. Сделай АП","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"11\n10 5 30\n30 5 10\n1 2 3\n1 6 3\n2 6 3\n1 1 1\n1 1 2\n1 1 3\n1 100000000 1\n2 1 1\n1 2 2\n","output":"YES\nYES\nYES\nYES\nNO\nYES\nNO\nYES\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSdelaiAP"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read_unsigned();
    let b = input.read_unsigned();
    let c = input.read_unsigned();

    if c < 2 * b {
        let tar_a = 2 * b - c;
        if tar_a % a == 0 {
            out_line!("YES");
            return;
        }
    }
    if a < 2 * b {
        let tar_c = 2 * b - a;
        if tar_c % c == 0 {
            out_line!("YES");
            return;
        }
    }
    if a % 2 == c % 2 {
        let tar_b = (a + c) / 2;
        if tar_b % b == 0 {
            out_line!("YES");
            return;
        }
    }
    out_line!("NO");
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
