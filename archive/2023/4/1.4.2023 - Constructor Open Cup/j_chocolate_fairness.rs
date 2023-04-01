//{"name":"J. Chocolate Fairness","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/J","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n12 11\n12 4\n12 1\n12 24\n","output":"18\n12\n3\n24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JChocolateFairness"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_size();

    if n >= 3 * k {
        out_line!(3 * k);
        return;
    }
    let total = 6 * k;
    let min_q = total / n;
    let min_r = n - total % n;
    let max_q = min_q + 1;
    let max_r = n - min_r;

    fn solve(q: usize) -> usize {
        if q % 6 != 1 {
            q / 6 + q % 6 / 2
        } else {
            let ans = q / 6 - 1;
            ans + 3
        }
    }

    let min = min_r * solve(min_q);
    let max = max_r * solve(max_q);
    out_line!(min + max);
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
