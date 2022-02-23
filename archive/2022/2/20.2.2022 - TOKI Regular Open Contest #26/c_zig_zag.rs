//{"name":"C. Zig-Zag","group":"TLX - TOKI Regular Open Contest #26","url":"https://tlx.toki.id/contests/troc-26/problems/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2 3\n","output":"YES\n"},{"input":"2\n2 1\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CZigZag"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let len: usize = a.iter().sum();
    if a[n - 1] * 2 > len + 1 || len % 2 == 0 && a[n - 1] * 2 > len {
        out_line!("NO");
        return;
    }
    for i in a.into_iter().take(n - 1) {
        if 2 * i > len {
            out_line!("NO");
            return;
        }
    }
    out_line!("YES");
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
