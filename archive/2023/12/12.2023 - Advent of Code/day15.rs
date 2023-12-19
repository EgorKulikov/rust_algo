//{"name":"day15","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day15"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let line = input.read_line();
    let parts = line.split(b',');

    {
        // part 1
        let mut ans = 0;
        for s in &parts {
            let mut hash = 0;
            for c in s.iter() {
                hash += c as usize;
                hash *= 17;
                hash %= 256;
            }
            ans += hash;
        }
        out.print_line(ans);
    }

    {
        // part 2
        let mut boxes = vec![Vec::<(Str<'static>, usize)>::new(); 256];
        for s in &parts {
            let label;
            let value;
            let tp;
            if s.contains(&b'=') {
                let tokens = s.split(b'=');
                label = tokens[0].clone();
                value = tokens[1].clone().parse::<usize>();
                tp = true;
            } else {
                let tokens = s.split(b'-');
                label = tokens[0].clone();
                value = 0;
                tp = false;
            }
            let mut id = 0;
            for c in label.iter() {
                id += c as usize;
                id *= 17;
                id %= 256;
            }
            let mut found = false;
            for i in boxes[id].indices() {
                if boxes[id][i].0 == label {
                    if tp {
                        boxes[id][i].1 = value;
                        found = true;
                    } else {
                        boxes[id].remove(i);
                    }
                    break;
                }
            }
            if tp && !found {
                boxes[id].push((label, value));
            }
        }
        let mut ans = 0;
        for (i, b) in boxes.enumerate() {
            for (j, (_, v)) in b.enumerate() {
                ans += (1 + i) * (1 + j) * v;
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
