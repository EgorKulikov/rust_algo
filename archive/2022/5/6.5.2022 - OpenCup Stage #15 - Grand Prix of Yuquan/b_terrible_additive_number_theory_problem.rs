//{"name":"B. Terrible Additive Number Theory Problem","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/B/","interactive":false,"timeLimit":1000,"tests":[{"input":"100\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTerribleAdditiveNumberTheoryProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
// use algo_lib::numbers::primes::primes;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();

    out_line!(if n >= 2431 { 1 } else { 0 });

    /*let p = primes::<i128>(1000000);
    let lim = 1_000_000_000_000_000_000i128;
    for i in 0..p.len() - 2 {
        let mut prod = 1;
        for j in i..p.len() - 1 {
            prod *= p[j];
            if prod > lim {
                break;
            }
            if (prod + 1) % p[j + 1] != 0 {
                continue;
            }
            let d = (prod + 1) / p[j + 1];
            if d.count_ones() == 1 {
                out_line!(prod);
                output().flush();
            }
        }
    }*/
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
