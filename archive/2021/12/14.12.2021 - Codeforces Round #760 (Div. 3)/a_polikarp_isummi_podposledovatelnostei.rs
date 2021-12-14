//{"name":"A. Поликарп и суммы подпоследовательностей","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 3 4 4 5 7 8\n1 2 3 4 5 6 7\n300000000 300000000 300000000 600000000 600000000 600000000 900000000\n1 1 2 999999998 999999999 999999999 1000000000\n1 2 2 3 3 4 5\n","output":"1 4 3\n4 1 2\n300000000 300000000 300000000\n999999998 1 1\n1 2 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APolikarpISummiPodposledovatelnostei"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let b: Vec<u32> = input.read_vec(7);

    let mut a = vec![b[0], b[1]];
    if b[2] == a[0] + a[1] {
        a.push(b[3]);
    } else {
        a.push(b[2]);
    }
    out_line!(a);
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
