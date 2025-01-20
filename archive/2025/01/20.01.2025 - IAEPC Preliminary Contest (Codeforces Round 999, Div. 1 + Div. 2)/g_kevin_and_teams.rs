//{"name":"G. Kevin and Teams","group":"Codeforces - IAEPC Preliminary Contest (Codeforces Round 999, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2061/problem/G","interactive":true,"timeLimit":4000,"tests":[{"input":"2\n3\n\n\n1\n\n5\n\n\n1\n\n0\n\n1\n\n0\n\n0\n","output":"\n\n1\n? 1 2\n\n! 1 2\n\n2\n? 1 2\n\n? 3 4\n\n? 3 5\n\n? 1 3\n\n? 2 4\n\n! 1 2 3 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let k = (n + 1) / 3;
    out.print_line(k);
    out.flush();

    let mut query = |i: usize, j: usize| -> bool {
        out.print_line(('?', i, j));
        out.flush();
        input.read_int() == 1
    };

    let mut vert_stack = Vec::new();
    let mut edge_stack = Vec::new();
    let mut good = Vec::new();
    for i in 1..=n {
        if vert_stack.is_empty() {
            vert_stack.push(i);
            continue;
        }
        let res = query(vert_stack[Back(0)], i);
        if edge_stack.is_empty() || edge_stack[Back(0)] == res {
            edge_stack.push(res);
            vert_stack.push(i);
        } else {
            let head = vert_stack.pop().unwrap();
            let other = vert_stack.pop().unwrap();
            edge_stack.pop();
            edge_stack.pop();
            if res {
                good.push((head, i, other));
            } else {
                good.push((head, other, i));
            }
        }
        if vert_stack.len() / 2 + good.len() == k {
            let mut ans = Vec::with_capacity(2 * k);
            ans.extend_from_slice(&vert_stack[..vert_stack.len() / 2 * 2]);
            for &(a, b, c) in good.iter() {
                ans.push(a);
                if edge_stack[0] {
                    ans.push(b);
                } else {
                    ans.push(c);
                }
            }
            out.print_line(('!', ans));
            out.flush();
            return;
        }
    }
    unreachable!();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
//END MAIN
