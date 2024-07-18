//{"name":"C - Takahashi Gets Lost","group":"AtCoder - Toyota Programming Contest 2024#2（AtCoder Beginner Contest 341）","url":"https://atcoder.jp/contests/abc341/tasks/abc341_c","interactive":false,"timeLimit":3000,"tests":[{"input":"6 7 5\nLULDR\n#######\n#...#.#\n##...##\n#.#...#\n#...#.#\n#######\n","output":"2\n"},{"input":"13 16 9\nULURDLURD\n################\n##..##.#..####.#\n###.#..#.....#.#\n#..##..#####.###\n#...#..#......##\n###.##.#..#....#\n##.#####....##.#\n###.###.#.#.#..#\n######.....##..#\n#...#.#.######.#\n##..###..#..#.##\n#...#.#.#...#..#\n################\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTakahashiGetsLost"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let _n = input.read_size();
    let t = input.read_str();
    let s = input.read_char_table(h, w);

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[(i, j)] == '#' {
                continue;
            }
            let mut r = i;
            let mut c = j;
            ans += 1;
            for d in t.iter() {
                match d {
                    b'U' => r -= 1,
                    b'D' => r += 1,
                    b'L' => c -= 1,
                    b'R' => c += 1,
                    _ => unreachable!(),
                }
                if s[(r, c)] == '#' {
                    ans -= 1;
                    break;
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
    //    tester::stress_test();
}
//END MAIN
