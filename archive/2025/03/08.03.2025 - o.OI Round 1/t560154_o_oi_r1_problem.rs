//{"name":"T560154 「o.OI R1」α+β Problem","group":"Luogu","url":"https://www.luogu.com.cn/problem/T560154?contestId=224200","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n3 4\n2 2 17\nDWSAWDWSSDWAWDWAW\n","output":"..S.\nSXS.\n..S.\n"},{"input":"1\n3 4\n3 4 9\nWSAWWDWDW\n","output":"|.|.\n|SSS\n..SX\n"},{"input":"1\n3 4\n2 1 8\nDWASDWAW\n","output":"||.|\nXS.|\n|.S.\n"},{"input":"2\n5 4\n5 1 9\nWDWAWDWAW\n5 19\n5 8 12\nDDSAADDADADW\n","output":"....\n..S.\n.SS.\nSS..\nX...\n.........||||||||||\n.........||......||\n||||||...||......||\n||..||...||......||\n||||||.X.||||||||||\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let x = input.read_size() - 1;
    let y = input.read_size() - 1;
    let _k = input.read_size();
    let s = input.read_str();

    let mut visited = Arr2d::new(n, m, false);
    let mut dx = -1;
    let mut dy = 0;
    let mut cx = x as isize;
    let mut cy = y as isize;
    let mut boxes = Arr2d::new(n, m, None);
    let mut ans = Arr2d::new(n, m, b'.');
    for c in s.copy_iter() {
        visited[(cx as usize, cy as usize)] = true;
        match c {
            b'W' => {
                cx += dx;
                cy += dy;
                if !visited[(cx as usize, cy as usize)]
                    && boxes[(cx as usize, cy as usize)].is_none()
                {
                    ans[(cx as usize, cy as usize)] = b'S';
                    boxes[(cx as usize, cy as usize)] = Some((cx as usize, cy as usize));
                }
                if let Some((bx, by)) = boxes[(cx as usize, cy as usize)] {
                    boxes[(cx as usize, cy as usize)] = None;
                    let nx = cx + dx;
                    let ny = cy + dy;
                    if nx < 0
                        || ny < 0
                        || nx >= n as isize
                        || ny >= m as isize
                        || boxes[(nx as usize, ny as usize)].is_some()
                    {
                        ans[(bx, by)] = b'.';
                    } else {
                        boxes[(nx as usize, ny as usize)] = Some((bx, by));
                    }
                }
            }
            b'S' => {
                cx -= dx;
                cy -= dy;
                if let Some((bx, by)) = boxes[(cx as usize, cy as usize)] {
                    ans[(bx, by)] = b'.';
                    boxes[(cx as usize, cy as usize)] = None;
                }
            }
            b'A' => (dx, dy) = (-dy, dx),
            b'D' => (dx, dy) = (dy, -dx),
            _ => unreachable!(),
        }
    }
    ans[(x, y)] = b'X';
    out.print_table(&ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
