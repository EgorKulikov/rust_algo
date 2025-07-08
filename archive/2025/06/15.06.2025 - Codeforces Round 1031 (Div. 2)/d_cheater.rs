//{"name":"D. Cheater","group":"Codeforces - Codeforces Round 1031 (Div. 2)","url":"https://codeforces.com/contest/2113/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n7\n13 7 4 9 12 10 2\n6 1 14 3 8 5 11\n3\n1 6 5\n2 3 4\n5\n8 6 3 10 1\n7 9 5 2 4\n","output":"6\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let b = input.read_size_vec(n);
    let a = input.read_size_vec(n);

    let min_front = Vec::with_gen_prefix(
        n,
        |i, v: &Vec<usize>| if i == 0 { b[0] } else { v[i - 1].min(b[i]) },
    );
    let mut second_min_front = Vec::with_capacity(n);
    let mut min = 2 * n;
    let mut second_min = 2 * n;
    for i in 0..n {
        if b[i] < min {
            second_min = min;
            min = b[i];
        } else if b[i] < second_min {
            second_min = b[i];
        }
        second_min_front.push(second_min);
    }
    let max_back = Vec::with_gen_back(
        n + 1,
        |i, v: &Vec<usize>| if i == n { 0 } else { v[i + 1].max(b[i]) },
    );
    min = 2 * n;
    for i in 0..n {
        min.minim(a[i]);
        let best = second_min_front[Back(i)].min(min_front[Back(i)].max(max_back[Back(i)]));
        if best > min {
            out.print_line(n - i);
            return;
        }
    }
    out.print_line(0);
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
