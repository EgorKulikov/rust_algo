//{"name":"WOSS Dual Olympiad 2021 S2: Light Show","group":"DMOJ","url":"https://dmoj.ca/problem/wossoly2021s2","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nRGBG\nRGGR\nGBGR\nBGRB\n","output":"10\n"},{"input":"5\nGGGGG\nBBBBB\nGGGGG\nRRRRR\n","output":"25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WOSSDualOlympiad2021S2LightShow"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    input.read::<usize>();
    let top: Str = input.read();
    let right: Str = input.read();
    let bottom: Str = input.read();
    let left: Str = input.read();

    fn convert(c: u8) -> usize {
        match c {
            b'R' => 1,
            b'G' => 2,
            b'B' => 4,
            _ => panic!("unreachable"),
        }
    }

    fn join(one: Str, two: Str) -> [u64; 8] {
        let mut qty = [0u64; 8];
        for (i, j) in one.into_iter().zip(two.into_iter()) {
            qty[convert(i) | convert(j)] += 1;
        }
        qty
    }

    let qty_vertical = join(top, bottom);
    let qty_horizontal = join(left, right);
    let mut ans = 0u64;
    for (i, q_v) in qty_vertical.into_iter().enumerate() {
        for (j, q_h) in qty_horizontal.into_iter().enumerate() {
            if (i | j) == 7 {
                ans += q_v * q_h;
            }
        }
    }
    out_line!(ans);
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
