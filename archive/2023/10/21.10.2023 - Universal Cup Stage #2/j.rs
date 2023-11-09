//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read_size();
    let m = input.read_size();
    let p = input.read_size_vec(m);

    let mut set = p
        .into_iter()
        .map(|x| (x - 1, false))
        .collect::<HashSet<_>>();
    let mut ans = 0;
    while n > 1 {
        let mut next_set = HashSet::new();
        for &(p, is_opt) in &set {
            if p >= n / 2 {
                if !is_opt
                    && !set.contains(&(n - p - 1, false))
                    && !set.contains(&(n - p - 1, true))
                {
                    ans += 1;
                    next_set.insert((n - p - 1, true));
                }
            } else {
                if set.contains(&(n - p - 1, false)) {
                    next_set.insert((p, false));
                } else if set.contains(&(n - p - 1, true)) {
                    next_set.insert((p, is_opt));
                } else if !is_opt {
                    ans += 1;
                    next_set.insert((p, true));
                }
            }
        }
        n /= 2;
        set = next_set;
    }
    out.print_line(ans);
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
