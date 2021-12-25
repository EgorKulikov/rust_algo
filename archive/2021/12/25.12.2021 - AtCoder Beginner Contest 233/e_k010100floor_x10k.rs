//{"name":"E - Σ[k=0..10^100]floor(X／10^k)","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_e","interactive":false,"timeLimit":2000,"tests":[{"input":"1225\n","output":"1360\n"},{"input":"99999\n","output":"111105\n"},{"input":"314159265358979323846264338327950288419716939937510\n","output":"349065850398865915384738153697722542688574377708317\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EK010100floorX10k"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let x: Str = input.read();

    let mut a = Vec::with_capacity(x.len());
    let mut sum = 0u32;
    for c in x {
        sum += (c - b'0') as u32;
        a.push(sum);
    }
    a.reverse();
    for i in 0..(a.len() - 1) {
        let d = a[i] / 10;
        a[i + 1] += d;
        a[i] %= 10;
    }
    while *a.last().unwrap() >= 10 {
        let d = *a.last().unwrap() / 10;
        *a.last_mut().unwrap() %= 10;
        a.push(d);
    }
    a.reverse();
    for i in a {
        out!(i);
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
