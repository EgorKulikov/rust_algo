//{"name":"day_4","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_4"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(input.read_line());
    }

    // part 1
    {
        let pattern = b"XMAS";
        let mut ans = 0;
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                for di in 0..=2 {
                    for dj in 0..=2 {
                        let mut good = true;
                        for k in 0..4 {
                            if i + di * k < k
                                || i + di * k >= data.len() + k
                                || j + dj * k < k
                                || j + dj * k >= data[i].len() + k
                                || data[i + di * k - k][j + dj * k - k] != pattern[k]
                            {
                                good = false;
                                break;
                            }
                        }
                        if good {
                            ans += 1;
                        }
                    }
                }
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut ans = 0;
        for i in 0..data.len() - 2 {
            for j in 0..data[i].len() - 2 {
                if data[i + 1][j + 1] == b'A' {
                    let mut good = true;
                    for k in [0, 2] {
                        let left = data[i][j + k];
                        let right = data[i + 2][j + 2 - k];
                        if left != b'M' && left != b'S'
                            || right != b'M' && right != b'S'
                            || left == right
                        {
                            good = false;
                            break;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
