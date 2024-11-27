//{"name":"E. Palindromic String","group":"Toph","url":"https://toph.co/arena?practice=67470fb3aca7a845c3abf7a2#!/p/67417e886ca53f1880b565c8","interactive":false,"timeLimit":1000,"tests":[{"input":"8\nabaxyaba\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPalindromicString"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    for i in 0..n / 2 {
        if s[i] != s[Back(i)] {
            let mut add = 0;
            for j in i + 1..n - i {
                let ss = &s[i..j];
                let mut good = true;
                for k in 0..ss.len() / 2 {
                    if ss[k] != ss[Back(k)] {
                        good = false;
                        break;
                    }
                }
                if good {
                    add.maxim(j - i);
                }
                let ss = &s[n - j - 1..n - i - 1];
                let mut good = true;
                for k in 0..ss.len() / 2 {
                    if ss[k] != ss[Back(k)] {
                        good = false;
                        break;
                    }
                }
                if good {
                    add.maxim(j - i);
                }
            }
            out.print_line(i * 2 + add);
            return;
        }
    }
    out.print_line(n);
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
