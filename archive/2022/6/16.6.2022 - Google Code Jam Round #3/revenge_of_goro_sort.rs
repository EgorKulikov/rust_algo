//{"name":"Revenge of GoroSort","group":"Google Coding Competitions - Round 3 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008779b4/0000000000b45189","interactive":true,"timeLimit":20000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"RevengeOfGoroSort"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let t = input.read_usize();
    let n = input.read_usize();
    let _k = input.read_usize();

    const BUBEN: usize = 3;

    for _ in 0..t {
        loop {
            let p = input.read_usize_vec(n).dec_by_one();
            let mut color = vec![0; n];
            let mut next = 1;
            for i in 0..n {
                if color[i] != 0 {
                    continue;
                }
                let mut j = i;
                let mut pos = Vec::new();
                loop {
                    pos.push(j);
                    j = p[j];
                    if j == i {
                        break;
                    }
                }
                for i in (0..pos.len()).step_by(BUBEN) {
                    for j in i..(i + BUBEN).min(pos.len()) {
                        color[pos[j]] = next;
                    }
                    next += 1;
                }
                // if pos.len() % 2 == 1 {
                //     color[pos[pos.len() - 1]] = next;
                //     next += 1;
                // }
            }
            out_line!(color);
            let res = input.read_int();
            if res == -1 {
                return;
            }
            if res == 1 {
                break;
            }
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 0);
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
