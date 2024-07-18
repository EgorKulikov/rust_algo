//{"name":"chr3_d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"chr3_d"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();

    let mut grass = Arr2d::new(n, m, true);
    let mut num_grass = n * m;
    for _ in 0..q {
        let tp = input.read_size();
        match tp {
            1 => {
                let dx: isize = input.read();
                let dy: isize = input.read();
                let mut x = input.read::<isize>() - 1;
                let mut y = input.read::<isize>() - 1;
                while x >= 0
                    && y >= 0
                    && x < n as isize
                    && y < m as isize
                    && grass[x as usize][y as usize]
                {
                    grass[x as usize][y as usize] = false;
                    num_grass -= 1;
                    x += dx;
                    y += dy;
                }
            }
            2 => {
                let x = input.read_size() - 1;
                let y = input.read_size() - 1;
                out.print_line(if grass[x][y] { 0 } else { 1 });
            }
            3 => {
                out.print_line(num_grass);
            }
            _ => unreachable!(),
        }
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
