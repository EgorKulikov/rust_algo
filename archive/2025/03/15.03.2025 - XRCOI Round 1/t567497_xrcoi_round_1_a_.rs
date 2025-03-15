//{"name":"T567497 [XRCOI Round 1] A. 相聚相逢本无意","group":"Luogu","url":"https://www.luogu.com.cn/problem/T567497?contestId=221170","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n2 3\n3 1\n4 1\n","output":"6\n0 1 1 1 2 3\n"},{"input":"4\n1 1\n2 3\n3 0\n4 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let xy = input.read_size_pair_vec(k);

    let mut val = vec![None; 101];
    for (x, y) in xy {
        if let Some(yy) = val[x] {
            if yy != y {
                out.print_line(-1);
                return;
            }
        } else {
            val[x] = Some(y);
        }
    }
    if let Some(y) = val[0] {
        if y != 0 {
            for i in 1..=100 {
                if let Some(yy) = val[i] {
                    if yy != 0 {
                        out.print_line(-1);
                        return;
                    }
                }
            }
            out.print_line(1);
            out.print_line(1);
            return;
        }
    }
    for i in 1..=100 {
        if let Some(y) = val[i] {
            if y == 0 {
                for j in i + 1..=100 {
                    if let Some(yy) = val[j] {
                        if yy != 0 {
                            out.print_line(-1);
                            return;
                        }
                    }
                }
            }
        }
    }
    let mut ans = Vec::new();
    for i in 1..=100 {
        if let Some(y) = val[i] {
            if y == 0 {
                break;
            }
            for _ in 0..y {
                ans.push(i - 1);
            }
        } else {
            ans.push(i - 1);
        }
    }
    out.print_line(ans.len());
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
