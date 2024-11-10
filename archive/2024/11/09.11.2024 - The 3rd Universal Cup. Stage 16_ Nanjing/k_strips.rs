//{"name":"K. Strips","group":"Universal Cup - The 3rd Universal Cup. Stage 16: Nanjing","url":"https://contest.ucup.ac/contest/1828/problem/9574","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5 2 3 16\n7 11 2 9 14\n13 5\n3 2 4 11\n6 10 2\n1 11\n2 1 2 6\n1 5\n3\n2 1 2 6\n1 5\n2\n","output":"4\n2 7 10 14\n-1\n2\n1 5\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KStrips"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let w = input.read_size();
    let a = input.read_size_vec(n).sorted();
    let b = input.read_size_vec(m).sorted();

    let mut at = Vec::new();
    let mut j = 0;
    for &i in &a {
        let mut seen_b = false;
        while j < b.len() && b[j] < i {
            j += 1;
            seen_b = true;
        }
        if at.is_empty() || *at.last().unwrap() + k <= i || seen_b {
            at.push(i);
        }
    }
    let mut last = w + 1;
    let mut j = b.len();
    for i in at.indices().rev() {
        while j > 0 && b[j - 1] >= at[i] {
            last = b[j - 1];
            j -= 1;
        }
        if last <= k {
            out.print_line(-1);
            return;
        }
        at[i].minim(last - k);
        if j > 0 && b[j - 1] >= at[i] {
            out.print_line(-1);
            return;
        }
        last = at[i];
    }
    let mut j = 0;
    for &i in &a {
        while j < at.len() && at[j] <= i {
            j += 1;
        }
        if j == 0 || at[j - 1] + k <= i {
            out.print_line(-1);
            return;
        }
    }
    out.print_line(at.len());
    out.print_line(at);
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
