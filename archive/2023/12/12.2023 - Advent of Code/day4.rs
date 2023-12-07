//{"name":"day4","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day4"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::string::str::StrReader;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut qty = Vec::new();
    while !input.is_exhausted() {
        input.read_str();
        input.read_str();
        let mut winning = HashSet::new();

        loop {
            let s = input.read_str();
            if s == b"|".into() {
                break;
            }
            winning.insert(s);
        }
        let all = input.read_line();
        let tokens = all.split(b' ');
        // let mut cur = 1i64;
        let mut cur = 0i64;
        for token in tokens {
            if winning.contains(&token) {
                // cur *= 2;
                cur += 1;
            }
        }
        // cur /= 2;
        // ans += cur;
        qty.push(cur);
    }

    {
        // part 1
        let mut ans = 0i64;
        for &i in &qty {
            if i > 0 {
                ans += 1 << (i - 1);
            }
        }
        out.print_line(ans);
    }

    {
        // part 2
        let mut mem = Memoization1d::new(qty.len(), |mem, n| {
            if qty[n] == 0 {
                return 1;
            }
            let mut res = 1;
            for i in n + 1..=n + (qty[n] as usize) {
                res += mem.call(i);
            }
            res
        });
        let mut ans = 0i64;
        for i in 0..qty.len() {
            ans += mem.call(i);
        }
        out.print_line(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
