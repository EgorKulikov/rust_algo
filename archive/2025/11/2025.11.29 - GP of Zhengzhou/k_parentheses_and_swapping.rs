//{"name":"K. Parentheses and Swapping","group":"Universal Cup - GP of Zhengzhou","url":"https://contest.ucup.ac/contest/2661/problem/15311","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n6\n4 1 5 4 1 1\n4\n1 2 3 2\n4\n1 3 1 2\n2\n2 1\n8\n8 5 2 6 1 4 3 7\n","output":"((()))\n()()\n(())\n()\n((()))()\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, ValueDeltaNode};
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_delta::ValueTrait;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    #[derive(Copy, Clone)]
    struct Val(usize, usize);
    impl Default for Val {
        fn default() -> Self {
            Val(usize::MAX, usize::MAX)
        }
    }
    struct Value;
    impl ValueTrait for Value {
        type V = Val;
        fn join(v1: Self::V, v2: Self::V) -> Self::V {
            if v1.0 < v2.0 {
                v1
            } else if v2.0 < v1.0 {
                v2
            } else if v1.1 > v2.1 {
                v1
            } else {
                v2
            }
        }
    }
    let mut even = SegmentTree::with_gen(n, |i| {
        if i % 2 == 0 {
            ValueDeltaNode::<Value>::new(Val(a[i], i))
        } else {
            ValueDeltaNode::default()
        }
    });
    let mut odd = SegmentTree::with_gen(n, |i| {
        if i % 2 == 1 {
            ValueDeltaNode::<Value>::new(Val(a[i], i))
        } else {
            ValueDeltaNode::default()
        }
    });
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut until = vec![n];
    let mut b = Str::new();
    for i in 0..n {
        let end: usize = if i % 2 == 0 {
            odd.query(i..until.last().copied().unwrap()).v.1
        } else {
            even.query(i..until.last().copied().unwrap()).v.1
        };
        if end == usize::MAX {
            assert_eq!(stack[Back(0)].0, a[i]);
            stack.pop();
            until.pop();
            b.push(b')');
            continue;
        }
        if let Some(&(val, pos)) = stack.last() {
            if val == a[i] && a[pos] <= a[end] {
                stack.pop();
                until.pop();
                b.push(b')');
                continue;
            }
        }
        stack.push((a[end], i));
        until.push(end);
        b.push(b'(');
    }
    out.print_line(b);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
