//{"name":"C. Минимизируйте расстояние","group":"Codeforces - Codeforces Round #759 (Div. 2, основан на Отборочном раунде 3 Технокубка 2022)","url":"https://codeforces.com/contest/1591/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5 1\n1 2 3 4 5\n9 3\n-5 -10 -15 6 5 8 3 7 4\n5 3\n2 2 3 3 3\n4 2\n1000000000 1000000000 1000000000 1000000000\n","output":"25\n41\n7\n3000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMinimiziruiteRasstoyanie"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k: usize = input.read();
    let x: Vec<i64> = input.read_vec(n);

    let pos = x.iter().cloned().filter(|v| *v > 0).collect_vec();
    let neg = x.into_iter().filter(|v| *v < 0).map(|v| -v).collect_vec();
    let mut ans = 0u64;
    let mut delta = 0u64;
    for mut cur in [pos, neg] {
        cur.sort();
        if let Some(val) = cur.last() {
            delta.maxim(*val as u64);
        }
        ans += (cur.into_iter().rev().step_by(k).sum::<i64>() as u64) * 2;
    }
    out_line!(ans - delta);
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
