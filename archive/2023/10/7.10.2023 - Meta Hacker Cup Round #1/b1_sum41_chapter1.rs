//{"name":"B1: Sum 41 (Chapter 1)","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-1/problems/B1","interactive":false,"timeLimit":360000,"tests":[{"input":"7\n2023\n114\n41\n175\n434\n666\n1872\n","output":"Case #1: 3 7 17 17\nCase #2: 2 3 38\nCase #3: 1 41\nCase #4: 3 1 5 35\nCase #5: 4 1 2 7 31\nCase #6: -1\nCase #7: 5 1 2 6 6 26\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"sum_.*chapter_.*_.*input[.]txt"},"output":{"type":"file","fileName":"sum_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"B1Sum41Chapter1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use std::collections::HashMap;

type PreCalc = HashMap<i64, Vec<i64>>;

fn solve(input: &mut Input, output: &mut Output, data: &PreCalc) {
    let t = input.read_size();
    eprintln!("t = {}", t);
    for i in 0..t {
        let p = input.read_long();

        output.print_line((format!("Case #{}:", i + 1), data.get(&p)));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = HashMap::new();
    let mut rec = RecursiveFunction4::new(|rec, sum, max: i64, prod, mut v: Vec<i64>| {
        if sum == 0 {
            v.push(v.len() as i64);
            v.reverse();
            match pre_calc.get_mut(&prod) {
                None => {
                    pre_calc.insert(prod, v);
                }
                Some(was) => {
                    if was.len() > v.len() {
                        *was = v;
                    }
                }
            }
        } else {
            for i in 1..=max.min(sum) {
                let mut v = v.clone();
                v.push(i);
                rec.call(sum - i, i, prod * i, v);
            }
        }
    });
    rec.call(41, 41, 1, vec![]);
    solve(&mut input, &mut output, &pre_calc);
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
