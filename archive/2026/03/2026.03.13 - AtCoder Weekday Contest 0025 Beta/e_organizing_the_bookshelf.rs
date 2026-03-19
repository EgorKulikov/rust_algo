//{"name":"E - Organizing the Bookshelf","group":"AtCoder - AtCoder Weekday Contest 0025 Beta","url":"https://atcoder.jp/contests/awc0025/tasks/awc0025_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\n1 2 3 1 2\n2\n4\n2\n1\n3\n","output":"5\n4\n3\n2\n2\n"},{"input":"3 4\n1 1 1\n1\n1\n1\n1\n","output":"2\n1\n0\n0\n"},{"input":"10 10\n2 1 3 1 2 1 4 2 1 3\n3\n5\n1\n7\n2\n4\n3\n1\n2\n1\n","output":"10\n10\n10\n10\n9\n8\n7\n6\n5\n5\n"},{"input":"20 25\n3 1 4 1 5 9 2 6 5 3 5 8 9 7 9 3 2 3 8 4\n1\n3\n5\n7\n2\n10\n15\n1\n3\n5\n8\n12\n1\n1\n2\n4\n6\n3\n2\n1\n5\n4\n3\n2\n1\n","output":"20\n20\n20\n20\n19\n19\n19\n19\n18\n17\n17\n17\n16\n16\n16\n16\n16\n16\n16\n16\n16\n16\n16\n16\n15\n"},{"input":"1 1\n1000000000\n1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::payload::PurePayload;
use algo_lib::collections::treap::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let d = input.read_size_vec(n);

    let mut treap = Tree::with_gen(n, |i| PurePayload(d[i]));
    for _ in 0..q {
        let t = input.read_size() - 1;
        let pos = treap.get_at(t);
        if let Some(node) = pos.payload_mut() {
            node.0 -= 1;
            if node.0 == 0 {
                pos.detach();
            }
        }
        out.print_line(treap.size());
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
