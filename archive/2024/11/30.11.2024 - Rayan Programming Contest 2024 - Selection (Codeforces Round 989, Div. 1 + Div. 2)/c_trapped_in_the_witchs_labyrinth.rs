//{"name":"C. Trapped in the Witch's Labyrinth","group":"Codeforces - Rayan Programming Contest 2024 - Selection (Codeforces Round 989, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2034/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3 3\nUUU\nL?R\nDDD\n2 3\n???\n???\n3 3\n?U?\nR?L\nRDL\n","output":"0\n6\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTrappedInTheWitchsLabyrinth"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let t = input.read_char_table(n, m);

    let mut good = Vec::new();
    let mut from = Arr2d::new(n, m, Vec::new());
    for i in 0..n {
        for j in 0..m {
            match t[(i, j)] {
                b'U' => {
                    if i > 0 {
                        from[(i - 1, j)].push((i, j));
                    } else {
                        good.push((i, j));
                    }
                }
                b'D' => {
                    if i + 1 < n {
                        from[(i + 1, j)].push((i, j));
                    } else {
                        good.push((i, j));
                    }
                }
                b'L' => {
                    if j > 0 {
                        from[(i, j - 1)].push((i, j));
                    } else {
                        good.push((i, j));
                    }
                }
                b'R' => {
                    if j + 1 < m {
                        from[(i, j + 1)].push((i, j));
                    } else {
                        good.push((i, j));
                    }
                }
                _ => {}
            }
        }
    }
    let mut is_good = Arr2d::new(n, m, false);
    let mut ans = n * m;
    while let Some((i, j)) = good.pop() {
        is_good[(i, j)] = true;
        ans -= 1;
        for (r, c) in from[(i, j)].copy_iter() {
            good.push((r, c));
        }
    }
    for i in 0..n {
        for j in 0..m {
            if t[(i, j)] == b'?' {
                let mut good = true;
                for (r, c) in D4::iter(i, j, n, m) {
                    if !is_good[(r, c)] {
                        good = false;
                        break;
                    }
                }
                if good {
                    ans -= 1;
                }
            }
        }
    }
    out.print_line(ans);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
