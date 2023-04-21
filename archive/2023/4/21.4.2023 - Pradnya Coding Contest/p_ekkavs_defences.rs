//{"name":"PEKKA Vs Defences","group":"CodeChef - PRAD2023","url":"https://www.codechef.com/PRAD2023/problems/PEK_VS_DEF","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 100 2\n2 4 6\n1 3 5\n3 10 2\n2 4 6\n1 3 5\n","output":"6\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PEKKAVsDefences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let h = input.read_size();
    let k = input.read_size();
    let d = input.read_size_vec(n);
    let t = input.read_size_vec(n);

    let mut left = 0;
    let mut right = k * n;

    while left < right {
        let mid = (left + right) / 2;
        let mut total_damage = 0;
        for i in 0..n {
            let time = ((i + 1) * k).min(mid);
            total_damage += (time / t[i]) * d[i];
        }
        if total_damage >= h {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out_line!(left);
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
