//{"name":"N. Nene is You","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/N","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2\nAA\nAA\nAA\n2\nA is B\nB is A\n","output":"YES\n0\n"},{"input":"3 4\nAABB\nBBCC\nAABB\n4\nA is D\nE is F\nC is A\nD is C\n","output":"YES\n3\nA is E\nC is A\nE is C\n"},{"input":"1 3\nABC\n2\nA is D\nC is D\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NNeneIsYou"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let r = input.read_usize();
    let c = input.read_usize();
    let map = input.read_table::<char>(r, c);
    let n = input.read_usize();

    let mut state = [None; 26];
    for c in map {
        let i = ((c as u8) - b'A').into_usize();
        state[i] = Some(i);
    }

    let mut ans = Vec::new();
    for _ in 0..n {
        let from = ((input.read_char()) as u8 - b'A').into_usize();
        input.next_token();
        let to = ((input.read_char()) as u8 - b'A').into_usize();
        if from == to {
            continue;
        }
        if state[from].is_some() {
            match state[to].as_ref() {
                None => {
                    state[to] = state[from].take();
                    ans.push(format!(
                        "{} is {}",
                        (b'A' + to.into_u8()) as char,
                        (b'A' + from.into_u8()) as char
                    ));
                }
                Some(_) => {
                    out_line!("NO");
                    return;
                }
            }
        }
    }
    ans.reverse();

    out_line!("YES");
    out_line!(ans.len());
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
