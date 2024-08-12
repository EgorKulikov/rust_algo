//{"name":"D. Mr.Wow and Multiset","group":"Codeforces - TheForces Round #33(Wow-Forces)","url":"https://codeforces.com/gym/105293/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 0\n3 1\n4 6\n4 10\n5 -11\n","output":"YES\n3 2\n1 1\nNO\nYES\n2 3\n-1 4\n1 -5\nNO\nYES\n2 3\n-1 4\n-5 5\n-10 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMrWowAndMultiset"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let m = input.read_long();

    if m.abs() > n * (n + 1) / 2 - 2 || m.abs() % 2 != n * (n + 1) / 2 % 2 {
        out.print_line(false);
        return;
    }

    let mut sign = HashMap::new();
    let mut cur = m;
    for i in (1..=n).rev() {
        if (cur + i).abs() < (cur - i).abs() {
            sign.insert(i, false);
            cur += i;
        } else {
            sign.insert(i, true);
            cur -= i;
        }
    }
    assert_eq!(cur, 0);
    let mut positive_at = 0;
    let mut negative_at = 0;
    for i in 1..=n {
        if sign[&i] {
            positive_at = i;
        } else {
            negative_at = i;
        }
    }
    let mut cur = m;
    let mut ans = Vec::new();
    let mut s = true;
    for i in 1..=n {
        if i != positive_at && i != negative_at {
            if sign[&i] == s {
                let next_cur = i - cur;
                ans.push((i, next_cur));
                cur = next_cur;
                s ^= true;
            } else {
                let next_cur = i + cur;
                ans.push((next_cur, i));
                cur = next_cur;
            }
        }
    }
    if s {
        assert_eq!(cur, positive_at - negative_at);
        ans.push((positive_at, negative_at));
    } else {
        assert_eq!(cur, negative_at - positive_at);
        ans.push((negative_at, positive_at));
    }
    ans.reverse();
    out.print_line(true);
    out.print_per_line(&ans);
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
