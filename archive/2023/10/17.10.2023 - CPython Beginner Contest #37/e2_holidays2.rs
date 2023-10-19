//{"name":"E2. Holidays #2","group":"CPython.uz - CPython Beginner Contest #37","url":"https://cpython.uz/competitions/contests/contest/312/problem/E2","interactive":false,"timeLimit":200,"tests":[{"input":"07-07-2020 07-07-2022\n","output":"5\n"},{"input":"01-01-2020 31-12-2023\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2Holidays2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    scan!(input, "@-@-@ @-@-@", d1: usize, m1: usize, y1: usize, d2: usize, m2: usize, y2: usize);

    let mut ans = 0;
    for y in y1..=y2 {
        let from_m = if y == y1 { m1 } else { 1 };
        let to_m = if y == y2 { m2 } else { 12 };
        for m in from_m..=to_m {
            let from_d = if y == y1 && m == m1 { d1 } else { 1 };
            let to_d = if y == y2 && m == m2 { d2 } else { 31 };
            for d in from_d..=to_d {
                let cur = match (d, m) {
                    (2, 2) | (7, 7) | (30, 12) => y >= 2021,
                    (19, 4) => y >= 2023,
                    _ => false,
                };
                if cur {
                    ans += 1;
                }
            }
        }
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
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
