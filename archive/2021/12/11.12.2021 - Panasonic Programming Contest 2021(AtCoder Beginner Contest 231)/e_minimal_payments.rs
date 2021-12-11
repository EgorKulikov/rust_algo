//{"name":"E - Minimal payments","group":"AtCoder - Panasonic Programming Contest 2021(AtCoder Beginner Contest 231)","url":"https://atcoder.jp/contests/abc231/tasks/abc231_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 87\n1 10 100\n","output":"5\n"},{"input":"2 49\n1 7\n","output":"7\n"},{"input":"10 123456789012345678\n1 100 10000 1000000 100000000 10000000000 1000000000000 100000000000000 10000000000000000 1000000000000000000\n","output":"233\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMinimalPayments"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line};
use std::collections::HashMap;

fn solve(input: &mut Input) {
    let n = input.read();
    let x: u64 = input.read();
    let a: Vec<u64> = input.read_vec(n);

    let mut ans = HashMap::new();
    let mut rec = RecursiveFunction2::new(|f, step: usize, rem: u64| -> u64 {
        if ans.contains_key(&(step, rem)) {
            ans[&(step, rem)]
        } else if step == 0 {
            ans.insert((step, rem), rem);
            rem
        } else {
            let res = (rem / a[step] + f.call(step - 1, rem % a[step]))
                .min(rem / a[step] + 1 + f.call(step - 1, a[step] - rem % a[step]));
            ans.insert((step, rem), res);
            res
        }
    });
    out_line!(rec.call(n - 1, x));
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
