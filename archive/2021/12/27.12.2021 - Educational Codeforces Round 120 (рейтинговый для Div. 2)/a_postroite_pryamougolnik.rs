//{"name":"A. Постройте прямоугольник","group":"Codeforces - Educational Codeforces Round 120 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1622/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6 1 5\n2 5 2\n2 4 2\n5 5 4\n","output":"YES\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APostroitePryamougolnik"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut l = input.read_unsigned_vec(3);

    l.sort_unstable();
    if l[0] + l[1] == l[2] || l[0] == l[1] && l[2] % 2 == 0 || l[1] == l[2] && l[0] % 2 == 0 {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
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
