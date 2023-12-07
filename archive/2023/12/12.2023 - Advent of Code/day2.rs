//{"name":"day2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day2"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::eol_string::EolString;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::scan;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut ans1 = 0;
    let mut ans2 = 0;

    while !input.is_exhausted() {
        scan!(input, "Game @: @", _id: usize, sets: EolString);
        let as_str = String::from_utf8_lossy(sets.as_slice());
        let sets = as_str.split("; ");
        let mut valid = true;
        let mut mins = DefaultHashMap::<_, i64>::new();
        for set in sets {
            let tokens = set.split(", ");
            for token in tokens {
                let mut bytes = token.as_bytes();
                scan!(&mut bytes, "@ @", qty: i64, color: Str<'static>);
                let limit = match color.as_slice() {
                    b"blue" => 14,
                    b"green" => 13,
                    b"red" => 12,
                    _ => unreachable!(),
                };
                if qty > limit {
                    valid = false;
                }
                mins[color].maxim(qty);
            }
        }
        if valid {
            ans1 += id;
        }
        let cur = mins.values().fold(1, |acc, &x| acc * x);
        ans2 += cur;
    }

    {
        // part 1
        out.print_line(ans1);
    }
    {
        // part 2
        out.print_line(ans2);
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
