//{"name":"Sheba's Amoebas","group":"Kattis","url":"https://open.kattis.com/problems/amoebas","interactive":false,"timeLimit":1000,"tests":[{"input":"12 12\n.##########.\n#..........#\n#..#...##..#\n#.##..#..#.#\n#......#.#.#\n#....#..#..#\n#...#.#....#\n#..#...#...#\n.#..#.#....#\n#....#.....#\n#.........#.\n.#########..\n","output":"4\n"},{"input":"12 10\n.#####....\n#.....#...\n#..#..#...\n#.#.#.#...\n#..#..#...\n.#...#....\n..###.....\n......#...\n.##..#.#..\n#..#..#...\n.##.......\n..........\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ShebasAmoebas"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D8;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut grid = input.read_char_table(n, m);

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] == b'#' {
                ans += 1;
                let mut queue = vec![(i, j)];
                grid[(i, j)] = b'.';
                while let Some((r, c)) = queue.pop() {
                    for (nr, nc) in D8::iter(r, c, n, m) {
                        if grid[(nr, nc)] == b'#' {
                            grid[(nr, nc)] = b'.';
                            queue.push((nr, nc));
                        }
                    }
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
