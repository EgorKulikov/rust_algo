//{"name":"B. Ela's Fitness and the Luxury Number","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n8 19\n8 20\n119 121\n1 100000000000000000\n1234567891011 1000000000000000000\n","output":"5\n6\n2\n948683296\n2996666667\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BElasFitnessAndTheLuxuryNumber"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let l = input.read_long();
    let r = input.read_long();

    fn get(n: i64) -> i64 {
        if n == 0 {
            return 0;
        }
        let mut sq = (n as f64).sqrt() as i64;
        while sq * sq > n {
            sq -= 1;
        }
        while (sq + 1) * (sq + 1) <= n {
            sq += 1;
        }
        let mut ans = (sq - 1) * 3 + 1;
        if n >= sq * sq + sq {
            ans += 1;
        }
        if n >= sq * sq + 2 * sq {
            ans += 1;
        }
        ans
    }

    out_line!(get(r) - get(l - 1));
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
