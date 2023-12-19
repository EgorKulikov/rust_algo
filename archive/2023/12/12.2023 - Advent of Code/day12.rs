//{"name":"day12","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day12"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut inp = Vec::new();
    while !input.is_exhausted() {
        let mask = input.read_str();
        let groups = input.read_str();
        let groups = groups
            .split(b",")
            .into_iter()
            .map(|s| s.parse::<usize>())
            .collect_vec();
        inp.push((mask, groups));
    }

    let solve = |repeat: usize| -> i64 {
        let mut ans = 0;
        for (mask_base, groups_base) in &inp {
            let mut mask = Str::new();
            let mut groups = Vec::new();
            for i in 0..repeat {
                if i != 0 {
                    mask += b'?';
                }
                mask += mask_base;
                groups.extend_from_slice(groups_base);
            }
            let mut mem =
                Memoization2d::new(mask.len() + 1, groups.len() + 1, |mem, at, pos| -> i64 {
                    if at == mask.len() {
                        return if pos == groups.len() { 1 } else { 0 };
                    }
                    let mut res = 0;
                    if mask[at] != b'#' {
                        res += mem.call(at + 1, pos);
                    }
                    if pos < groups.len()
                        && at + groups[pos] <= mask.len()
                        && mask[at..at + groups[pos]].iter().all(|&c| c != b'.')
                        && (at + groups[pos] == mask.len() || mask[at + groups[pos]] != b'#')
                    {
                        res += mem.call((at + groups[pos] + 1).min(mask.len()), pos + 1);
                    }
                    res
                });
            ans += mem.call(0, 0);
        }
        ans
    };

    {
        // part 1
        out.print_line(solve(1));
    }

    {
        // part 2
        out.print_line(solve(5));
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
