//{"name":"P5 - Coffee Jelly","group":"DMOJ - OTHS Coding Competition 2","url":"https://dmoj.ca/problem/othscc2p5","interactive":false,"timeLimit":1000,"tests":[{"input":"6 10\nXXXXXXXXXX\nX.*.X..X.X\nXX.XXXXXXX\nXXXX..XXXX\n*.......XX\nXXXXXXXXXX\n","output":"2\n"},{"input":"7 6\nXXX..X\n..XXX*\nXXXXXX\n..*...\nXXXXXX\nXXXXX.\n......\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5CoffeeJelly"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let map = input.read_char_table(n, m);

    let mut visited = Arr2d::new(n, m, false);
    let mut queue = Vec::new();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if map[(i, j)] != 'X' && !visited[(i, j)] {
                queue.push((i, j));
                visited[(i, j)] = true;
                let mut good = true;
                while let Some((r, c)) = queue.pop() {
                    if map[(r, c)] == '*' {
                        good = false;
                    }
                    for (nr, nc) in D4::iter(r, c, n, m) {
                        if !visited[(nr, nc)] && map[(nr, nc)] != 'X' {
                            visited[(nr, nc)] = true;
                            queue.push((nr, nc));
                        }
                    }
                }
                if good {
                    ans += 1;
                }
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
