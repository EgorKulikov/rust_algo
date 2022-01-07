//{"name":"L. Laser Circus","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/L","interactive":false,"timeLimit":1500,"tests":[{"input":"1 1\n/\n4\nR 1\nL 1\nT 1\nB 1\n","output":"B 1\nT 1\nL 1\nR 1\n"},{"input":"4 5\n\\/\\/\\\n\\/\\//\n\\\\/\\\\\n\\\\\\//\n4\nT 2\nR 1\nL 3\nB 4\n","output":"T 1\nT 5\nB 2\nL 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LLaserCircus"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let r = input.read_usize();
    let c = input.read_usize();
    let grid = input.read_table::<char>(r, c);
    let q = input.read_usize();

    for _ in 0..q {
        let mut dir = input.read_char();
        let (mut row, mut col) = match dir {
            'L' => (input.read_usize(), 1),
            'R' => (input.read_usize(), c),
            'T' => (1, input.read_usize()),
            'B' => (r, input.read_usize()),
            _ => unreachable!(),
        };
        while row >= 1 && row <= r && col >= 1 && col <= c {
            let t = grid[(row - 1, col - 1)];
            match dir {
                'L' => match t {
                    '/' => dir = 'B',
                    '\\' => dir = 'T',
                    _ => unreachable!(),
                },
                'R' => match t {
                    '/' => dir = 'T',
                    '\\' => dir = 'B',
                    _ => unreachable!(),
                },
                'T' => match t {
                    '/' => dir = 'R',
                    '\\' => dir = 'L',
                    _ => unreachable!(),
                },
                'B' => match t {
                    '/' => dir = 'L',
                    '\\' => dir = 'R',
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
            match dir {
                'L' => col += 1,
                'R' => col -= 1,
                'T' => row += 1,
                'B' => row -= 1,
                _ => unreachable!(),
            }
        }
        out_line!(match dir {
            'L' => ('R', row),
            'R' => ('L', row),
            'T' => ('B', col),
            'B' => ('T', col),
            _ => unreachable!(),
        });
    }
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
