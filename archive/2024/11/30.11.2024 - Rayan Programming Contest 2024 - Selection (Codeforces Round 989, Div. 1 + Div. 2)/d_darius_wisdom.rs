//{"name":"D. Darius' Wisdom","group":"Codeforces - Rayan Programming Contest 2024 - Selection (Codeforces Round 989, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2034/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n0 2 0 1\n3\n1 2 0\n6\n0 1 1 2 2 2\n","output":"2\n2 4\n2 3\n2\n3 1\n2 3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDariusWisdom"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);

    let mut twos = BTreeSet::new();
    let mut ones = BTreeSet::new();
    for i in 0..n {
        if a[i] == 1 {
            ones.insert(i);
        } else if a[i] == 2 {
            twos.insert(i);
        }
    }
    let mut ans = Vec::new();
    for i in (0..n).rev() {
        if a[i] == 0 {
            if let Some(&x) = ones.iter().next() {
                ones.remove(&x);
                a[x] = 0;
                ans.push((x + 1, i + 1));
                a[i] += 1;
            }
        }
        if a[i] == 1 {
            ones.remove(&i);
            if let Some(&x) = twos.iter().next() {
                twos.remove(&x);
                ones.insert(x);
                a[x] = 1;
                ans.push((x + 1, i + 1));
                a[i] += 1;
            }
        }
        if a[i] == 2 {
            twos.remove(&i);
            continue;
        }
    }
    out.print_line(ans.len());
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
