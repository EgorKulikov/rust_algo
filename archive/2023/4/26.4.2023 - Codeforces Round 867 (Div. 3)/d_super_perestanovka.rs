//{"name":"D. Супер-перестановка","group":"Codeforces - Codeforces Round 867 (Div. 3)","url":"https://codeforces.com/contest/1822/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n2\n3\n6\n","output":"1\n2 1\n-1\n6 5 2 3 4 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSuperPerestanovka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();

    if n == 1 {
        out_line!(1);
        return;
    }
    if n % 2 == 1 {
        out_line!(-1);
        return;
    }
    let mut ans = Vec::with_capacity(n);
    ans.push(n);
    ans.push(n - 1);
    let mut low = 2;
    let mut high = n - 1;
    for _ in 1..n / 2 {
        ans.push(low);
        low += 2;
        high -= 2;
        ans.push(high);
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
