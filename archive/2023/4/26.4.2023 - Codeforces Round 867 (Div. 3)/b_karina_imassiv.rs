//{"name":"B. Карина и массив","group":"Codeforces - Codeforces Round 867 (Div. 3)","url":"https://codeforces.com/contest/1822/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4\n5 0 2 1\n3\n-1 1 0\n5\n2 0 -1 -4 0\n6\n-8 4 3 7 1 -9\n6\n0 3 -2 5 -4 -4\n2\n1000000000 910000000\n7\n-1 -7 -2 -5 -4 -6 -3\n","output":"10\n0\n4\n72\n16\n910000000000000000\n42\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BKarinaIMassiv"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);

    a.sort();
    out_line!((a[0] * a[1]).max(a[n - 1] * a[n - 2]));
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
