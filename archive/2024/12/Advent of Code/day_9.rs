//{"name":"day_9","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_9"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    // part 1
    {
        let mut f = Vec::new();
        let mut is_free = false;
        let mut next = 0;
        for c in s.iter() {
            for _ in b'0'..c {
                if is_free {
                    f.push(None);
                } else {
                    f.push(Some(next));
                }
            }
            if is_free {
                is_free = false;
            } else {
                is_free = true;
                next += 1;
            }
        }
        let mut ans = 0;
        for i in 0.. {
            if i >= f.len() {
                break;
            }
            if let Some(block) = f[i] {
                ans += block * i;
            } else {
                while i < f.len() && f[Back(0)].is_none() {
                    f.pop();
                }
                if i < f.len() {
                    ans += i * f.pop().unwrap().unwrap();
                }
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut f = Vec::new();
        let mut is_free = false;
        let mut next = 0;
        for c in s.iter() {
            for _ in b'0'..c {
                if is_free {
                    f.push(None);
                } else {
                    f.push(Some(next));
                }
            }
            if is_free {
                is_free = false;
            } else {
                is_free = true;
                next += 1;
            }
        }
        for id in (0..next).rev() {
            let len = f.iter().count_eq(&&Some(id));
            for i in 1..f.len() {
                if f[i] == Some(id) {
                    break;
                }
                if f[i - 1].is_some() && f[i].is_none() {
                    let mut good = true;
                    for j in 0..len {
                        if i + j >= f.len() || f[i + j].is_some() {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        for j in f.indices() {
                            if f[j] == Some(id) {
                                f[j] = None;
                            }
                        }
                        for j in 0..len {
                            f[i + j] = Some(id);
                        }
                        break;
                    }
                }
            }
        }
        let mut ans = 0;
        for i in f.indices() {
            ans += i * f[i].unwrap_or(0);
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
