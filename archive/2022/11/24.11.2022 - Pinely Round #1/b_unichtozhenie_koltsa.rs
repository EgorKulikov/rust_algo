//{"name":"B. Уничтожение кольца","group":"Codeforces - Pinely Round 1 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1761/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 2 3 2\n4\n1 2 1 2\n1\n1\n","output":"4\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BUnichtozhenieKoltsa"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut bad = true;
    for i in 0..n {
        if a[i] != a[(i + 2) % n] {
            bad = false;
            break;
        }
    }
    out_line!(if bad { n / 2 + 1 } else { n });
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
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
