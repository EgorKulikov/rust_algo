//{"name":"B. Выворачивание массива","group":"Codeforces - Codeforces Round #759 (Div. 2, основан на Отборочном раунде 3 Технокубка 2022)","url":"https://codeforces.com/contest/1591/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n2 4 1 5 3\n5\n5 3 2 4 1\n4\n1 1 1 1\n","output":"1\n2\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVivorachivanieMassiva"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<u32> = input.read_vec(n);

    let mut ans = 0;
    let mut last = *a.last().unwrap();
    for i in a.into_iter().rev() {
        if i > last {
            last = i;
            ans += 1;
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
