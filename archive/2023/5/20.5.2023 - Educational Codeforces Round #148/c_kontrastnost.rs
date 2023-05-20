//{"name":"C. Контрастность","group":"Codeforces - Educational Codeforces Round 148 (Rated for Div. 2)","url":"https://codeforces.com/contest/1832/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\n1 3 3 3 7\n2\n4 2\n4\n1 1 1 1\n7\n5 4 2 1 0 0 4\n","output":"2\n2\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKontrastnost"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_int_vec(n);

    a.dedup();
    let n = a.len();
    let mut ans = 0;
    for i in 0..n {
        if (i == 0 || a[i] > a[i - 1]) && (i == n - 1 || a[i] > a[i + 1]) {
            ans += 1;
        } else if (i == 0 || a[i] < a[i - 1]) && (i == n - 1 || a[i] < a[i + 1]) {
            ans += 1;
        }
    }
    out_line!(ans);
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
