//{"name":"C. Числовой шаблон строки","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n3 5 2 1 3\n2\nabfda\nafbfa\n2\n1 2\n3\nab\nabc\naa\n4\n5 -3 5 -3\n4\naaaa\nbcbc\naba\ncbcb\n","output":"YES\nNO\nYES\nNO\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CChislovoiShablonStroki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::string::str::StrReader;
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let m = input.read_size();
    let s = input.read_str_vec(m);

    for s in s {
        if s.len() != n {
            out.print_line(false);
            continue;
        }
        let mut a_to_s = HashMap::new();
        let mut s_to_a = HashMap::new();
        let mut good = true;
        for i in 0..n {
            if let Some(c) = a_to_s.get(&a[i]) {
                if *c != s[i] {
                    good = false;
                    continue;
                }
            } else {
                a_to_s.insert(a[i], s[i]);
            }
            if let Some(c) = s_to_a.get(&s[i]) {
                if *c != a[i] {
                    good = false;
                    continue;
                }
            } else {
                s_to_a.insert(s[i], a[i]);
            }
        }
        out.print_line(good);
    }
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
