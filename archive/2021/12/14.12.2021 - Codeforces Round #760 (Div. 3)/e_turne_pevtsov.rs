//{"name":"E. Турне певцов","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n12 16 14\n1\n1\n3\n1 2 3\n6\n81 75 75 93 93 87\n","output":"YES\n3 1 3\nYES\n1\nNO\nYES\n5 5 4 1 4 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETurnePevtsov"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let b: Vec<i64> = input.read_vec(n);

    let sum_b = b.iter().sum::<i64>();
    let den = (n * (n + 1) / 2) as i64;
    if sum_b % den != 0 {
        out_line!("NO");
        return;
    }
    let sum_a = sum_b / den;
    let mut a = Vec::new();
    for i in 0..n {
        let prev = if i == 0 { n - 1 } else { i - 1 };
        let delta = b[i] - b[prev];
        let mut c_a = sum_a - delta;
        if c_a % n as i64 != 0 {
            out_line!("NO");
            return;
        }
        c_a /= n as i64;
        if c_a <= 0 {
            out_line!("NO");
            return;
        }
        a.push(c_a);
    }
    out_line!("YES");
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
