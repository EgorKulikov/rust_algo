//{"name":"A. AB to C","group":"Universal Cup - The 3rd Universal Cup. Stage 33: India","url":"https://contest.ucup.ac/contest/1954/problem/10265","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\nBC\n3\nAAA\n3\nCAA\n","output":"A\nAAA\nAB\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::VecDeque;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let qa = s.copy_count(b'A');
    let qb = s.copy_count(b'B');
    let qc = n - (qa + qb);

    if qa == n || qb == n || qc == n {
        out.print_line(s);
        return;
    }
    if (qb + qc) % 2 == 0 {
        let q = (qa + qb) % 2;
        if q == 0 {
            out.print_line("AA");
            return;
        } else {
            out.print_line("A");
            return;
        }
    }
    let mut aa = VecDeque::new();
    let mut bb = VecDeque::new();
    let mut cc = VecDeque::new();
    for i in 0..n {
        match s[i] {
            b'A' => aa.push_back(i),
            b'B' => bb.push_back(i),
            b'C' => cc.push_back(i),
            _ => unreachable!(),
        }
    }
    let mut front_a = 0;
    while !bb.is_empty() && !cc.is_empty() {
        bb.pop_front();
        cc.pop_front();
        front_a += 1;
    }
    while bb.len() >= 2 {
        bb.pop_front();
        bb.pop_front();
        if aa.pop_back().is_some() {
            front_a += 1;
        }
    }
    while cc.len() >= 2 {
        cc.pop_front();
        cc.pop_front();
        if aa.pop_back().is_some() {
            front_a += 1;
        }
    }
    if !bb.is_empty() {
        let b_pos = bb.pop_front().unwrap();
        let as_after = aa.copy_filter(|&x| x > b_pos).count();
        if as_after > 1 {
            let qa = front_a + aa.len() - 1;
            let mut ans = Str::from(vec![b'A'; qa]);
            ans.push(b'C');
            out.print_line(ans);
        } else {
            let mut ans = Str::from(vec![b'A'; front_a + aa.len() - as_after]);
            ans.push(b'B');
            if as_after == 1 {
                ans.push(b'A');
            }
            out.print_line(ans);
        }
    } else {
        let c_pos = cc.pop_front().unwrap();
        let as_after = aa.copy_filter(|&x| x > c_pos).count();
        if as_after > 0 {
            let qa = front_a + aa.len() - 1;
            let mut ans = Str::from(vec![b'A'; qa]);
            ans.push(b'B');
            out.print_line(ans);
        } else {
            let mut ans = Str::from(vec![b'A'; front_a + aa.len()]);
            ans.push(b'C');
            out.print_line(ans);
        }
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
