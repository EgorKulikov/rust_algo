//{"name":"A. Жизни цветов важны","group":"Codeforces - Codeforces Round #759 (Div. 2, основан на Отборочном раунде 3 Технокубка 2022)","url":"https://codeforces.com/contest/1591/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 0 1\n3\n0 1 1\n4\n1 0 0 1\n1\n0\n","output":"3\n7\n-1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AZhizniTsvetovVazhni"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<u8> = input.read_vec(n);

    let mut ans = 1;
    for i in 0..n {
        if a[i] == 1 {
            if i > 0 && a[i - 1] == 1 {
                ans += 5;
            } else {
                ans += 1;
            }
        } else if i > 0 && a[i - 1] == 0 {
            ans = -1;
            break;
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
