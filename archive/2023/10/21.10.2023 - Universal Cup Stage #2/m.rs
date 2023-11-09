//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();
    let t = input.read_str();

    let mut qt = vec![0; 26];
    for c in t.iter() {
        qt[c as usize - 'a' as usize] += 1;
    }
    let mut qs = vec![0; 26];
    for c in s.iter() {
        qs[c as usize - 'a' as usize] += 1;
    }
    for i in 0..26 {
        if qt[i] > qs[i] {
            out.print_line(0);
            return;
        }
    }
    if t.len() == 1 {
        for i in 0..26 {
            if qt[i] > 0 {
                out.print_line(qs[i]);
                return;
            }
        }
    }
    for i in (2..=t.len()).rev() {
        let mut ok = true;
        for j in 0..26 {
            if qt[j] == 0 {
                continue;
            }
            let rem = (i - qt[j] % i) % i;
            if rem > qt[j].upper_div(i) {
                ok = false;
                break;
            }
        }
        if ok {
            let mut ans = None;
            for j in 0..26 {
                if qt[j] == 0 {
                    continue;
                }
                let cur = qt[j].upper_div(i);
                ans.minim((qs[j] - qt[j]) / cur + 1);
            }
            out.print_line(ans);
            return;
        }
    }
    unreachable!();
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
