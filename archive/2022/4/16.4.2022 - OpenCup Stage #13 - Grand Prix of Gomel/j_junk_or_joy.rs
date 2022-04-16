//{"name":"J. Junk or Joy","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/J/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n22\n","output":"3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JJunkOrJoy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::primes::divisors;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let k = input.read_long();

    let mut set = HashSet::new();
    let mut get = |n: i64| -> i32 {
        if n <= 1 || set.contains(&n) {
            return 0;
        }
        set.insert(n);
        let kpm = n * n - 1;
        if kpm % k != 0 {
            return 0;
        }
        let pm = kpm / k;
        let d = divisors(pm);
        if d.len() != 1 {
            0
        } else {
            1
        }
    };
    let mut ans = 0;
    let mut i = 1;
    loop {
        if i * i > 2 * k {
            break;
        }
        if 2 * k % i == 0 {
            ans += get(i + 1);
            ans += get(i - 1);
            ans += get(2 * k / i + 1);
            ans += get(2 * k / i - 1);
        }
        i += 1;
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
}
//END MAIN
