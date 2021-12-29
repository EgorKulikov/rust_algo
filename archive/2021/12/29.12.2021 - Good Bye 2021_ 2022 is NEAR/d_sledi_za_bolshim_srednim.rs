//{"name":"D. Следи за большим средним","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"https://codeforces.com/contest/1616/problem/D","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n5\n1 2 3 4 5\n2\n10\n2 4 2 4 2 4 2 4 2 4\n3\n3\n-10 -5 -10\n-8\n3\n9 9 -3\n5\n","output":"4\n8\n2\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSlediZaBolshimSrednim"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_long_vec(n);
    let x = input.read_long();

    for i in a.iter_mut() {
        *i -= x;
    }
    let mut cur_sum = 0;
    let mut cur_len = 0;
    let mut ans = 0;
    for a in a {
        if cur_len == 0 || cur_sum + a >= 0 {
            cur_sum = (cur_sum + a).min(a);
            ans += 1;
            cur_len += 1;
        } else {
            cur_sum = 0;
            cur_len = 0;
        }
    }
    out_line!(ans);
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
