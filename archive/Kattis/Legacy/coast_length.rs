//{"name":"Coast Length","group":"Kattis","url":"https://open.kattis.com/problems/coast","interactive":false,"timeLimit":3000,"tests":[{"input":"5 6\n011110\n010110\n111000\n000010\n000000\n","output":"20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CoastLength"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::{border, D4};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut t = input.read_char_table(n, m);

    for (r, c) in border(n, m) {
        if t[(r, c)] == b'0' {
            let mut queue = vec![(r, c)];
            t[(r, c)] = b'2';
            while let Some((r, c)) = queue.pop() {
                for (nr, nc) in D4::iter(r, c, n, m) {
                    if t[(nr, nc)] == b'0' {
                        t[(nr, nc)] = b'2';
                        queue.push((nr, nc));
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for r in 0..n {
        for c in 0..m {
            if t[(r, c)] == b'1' {
                ans += 4;
                for (nr, nc) in D4::iter(r, c, n, m) {
                    if t[(nr, nc)] != b'2' {
                        ans -= 1;
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
