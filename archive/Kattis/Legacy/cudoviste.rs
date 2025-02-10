//{"name":"Cudoviste","group":"Kattis","url":"https://open.kattis.com/problems/cudoviste","interactive":false,"timeLimit":1000,"tests":[{"input":"4 4\n#..#\n..X.\n..X.\n#XX#\n","output":"1\n1\n2\n1\n0\n"},{"input":"4 4\n....\n....\n....\n....\n","output":"9\n0\n0\n0\n0\n"},{"input":"4 5\n..XX.\n.#XX.\n..#..\n.....\n","output":"2\n1\n1\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Cudoviste"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let r = input.read_size();
    let c = input.read_size();
    let map = input.read_char_table(r, c);

    let mut ans = [0; 5];
    for i in 0..r - 1 {
        for j in 0..c - 1 {
            let mut cnt = 0;
            let mut good = true;
            for x in 0..2 {
                for y in 0..2 {
                    if map[i + x][j + y] == b'X' {
                        cnt += 1;
                    }
                    if map[i + x][j + y] == b'#' {
                        good = false;
                    }
                }
            }
            if good {
                ans[cnt] += 1;
            }
        }
    }
    for i in ans {
        out.print_line(i);
    }
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
