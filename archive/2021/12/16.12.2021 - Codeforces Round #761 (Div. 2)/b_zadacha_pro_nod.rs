//{"name":"B. Задача про НОД","group":"Codeforces - Codeforces Round #761 (Div. 2)","url":"https://codeforces.com/contest/1617/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n18\n63\n73\n91\n438\n122690412\n","output":"6 9 3\n21 39 3\n29 43 1\n49 35 7\n146 219 73\n28622 122661788 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BZadachaProNOD"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: u32 = input.read();

    let c = 1;
    let mut a = 2;
    while gcd(a, n - a - c) != 1 {
        a += 1;
    }
    out_line!(a, n - a - c, c);
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
