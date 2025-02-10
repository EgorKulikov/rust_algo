//{"name":"Getting Gold","group":"Kattis","url":"https://open.kattis.com/problems/gold","interactive":false,"timeLimit":1000,"tests":[{"input":"7 4\n#######\n#P.GTG#\n#..TGG#\n#######\n","output":"1\n"},{"input":"8 6\n########\n#...GTG#\n#..PG.G#\n#...G#G#\n#..TG.G#\n########\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GettingGold"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let w = input.read_size();
    let h = input.read_size();
    let map = input.read_char_table(h, w);

    let mut queue = Vec::new();
    let mut visited = Arr2d::new(h, w, false);
    for i in 0..h {
        for j in 0..w {
            if map[(i, j)] == b'P' {
                queue.push((i, j));
                visited[(i, j)] = true;
            }
        }
    }
    let mut ans = 0;
    while let Some((r, c)) = queue.pop() {
        if map[(r, c)] == b'G' {
            ans += 1;
        }
        let mut trap = false;
        for (nr, nc) in D4::iter(r, c, h, w) {
            if map[(nr, nc)] == b'T' {
                trap = true;
            }
        }
        if trap {
            continue;
        }
        for (nr, nc) in D4::iter(r, c, h, w) {
            if map[(nr, nc)] != b'#' && !visited[(nr, nc)] {
                visited[(nr, nc)] = true;
                queue.push((nr, nc));
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
