//{"name":"Busy Airport","group":"HackerEarth - April Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/april-circuits-23/algorithm/busy-airport-9fa40749/","interactive":false,"timeLimit":1000,"tests":[{"input":"2 5 1 3\n3\n1 1 6\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BusyAirport"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read_long();
    let b = input.read_size();
    let c = input.read_long();
    let d = input.read_size();
    let n = input.read_size();
    let t = input.read_long_vec(n);

    let mut pass_free_at = vec![0; b];
    let mut luggage_free_at = vec![0; d];
    let mut ans = 0;
    for (i, x) in t.into_iter().enumerate() {
        let n_pass_free_at = pass_free_at[i % b].max(x) + a;
        pass_free_at[i % b] = n_pass_free_at;
        let n_luggage_free_at = luggage_free_at[i % d].max(n_pass_free_at) + c;
        luggage_free_at[i % d] = n_luggage_free_at;
        ans.maxim(n_luggage_free_at);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
