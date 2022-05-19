//{"name":"A - Antichain of Integer Strings","group":"AtCoder - AtCoder Grand Contest 057","url":"https://atcoder.jp/contests/agc057/tasks/agc057_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 8\n3 18\n1 1000\n","output":"6\n10\n900\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAntichainOfIntegerStrings"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::number_ext::{NumDigs, Power};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let t = input.read_usize();

    for _ in 0..t {
        let mut l = input.read_usize();
        let r = input.read_usize();
        let lnd = l.num_digs();
        let rnd = r.num_digs();
        if lnd == rnd {
            out_line!(r - l + 1);
            continue;
        }
        let r_start = 10.power(rnd - 1);
        if r >= 2 * r_start {
            out_line!(r - r_start + 1);
            continue;
        }
        l.maxim(r / 10 + 1);
        out_line!((r - l + 1).min(r_start));
    }
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
