//{"name":"day5","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day5"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    input.read_str();
    let seeds = input.read_line();
    let seeds = seeds.split(' ');

    let mut maps = Vec::with_capacity(7);
    input.read_line();
    for _ in 0..7 {
        input.read_line();
        let mut cur = Vec::new();
        loop {
            let line = input.read_line();
            if line.is_empty() {
                break;
            }
            let mut slice = line.as_slice();
            let mut c_inp = Input::new(&mut slice);
            cur.push((c_inp.read_long(), c_inp.read_long(), c_inp.read_long()));
        }
        maps.push(cur);
    }

    {
        // part 1
        let mut ans = None;
        for seed in seeds {
            let mut seed: i64 = seed.parse();
            for map in &maps {
                for &(to, from, len) in map {
                    if seed >= from && seed < from + len {
                        seed -= from;
                        seed += to;
                        break;
                    }
                }
            }
            ans.minim(seed);
        }
        out.print_line(ans);
    }
    {
        // part2
        let mut ans = None;
        for i in seeds.indices().step_by(2) {
            let mut start: i64 = seeds[i].clone().parse();
            let mut len: i64 = seeds[i + 1].clone().parse();
            while len > 0 {
                let mut seed = start;
                let mut cur_len = len;
                for map in &maps {
                    for &(to, from, len) in map {
                        if seed >= from && seed < from + len {
                            cur_len.minim(from + len - seed);
                            seed -= from;
                            seed += to;
                            break;
                        } else if seed < from {
                            cur_len.minim(from - seed);
                        }
                    }
                }
                ans.minim(seed);
                start += cur_len;
                len -= cur_len;
            }
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
