//{"name":"B. Отрезок последовательных точек","group":"Codeforces - Educational Codeforces Round 127 (Rated for Div. 2)","url":"https://codeforces.com/contest/1671/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1 4\n3\n1 2 3\n4\n1 2 3 7\n1\n1000000\n3\n2 5 6\n","output":"YES\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOtrezokPosledovatelnikhTochek"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_usize_vec(n);

    for i in x[0]..x[0] + 2 {
        let mut good = true;
        for (j, &x) in x.iter().enumerate() {
            if ((i + j).into_isize() - x.into_isize()).abs() > 1 {
                good = false;
                break;
            }
        }
        if good {
            out_line!("YES");
            return;
        }
    }
    out_line!("NO");
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
