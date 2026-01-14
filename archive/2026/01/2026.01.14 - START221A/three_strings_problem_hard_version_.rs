//{"name":"Three Strings Problem (Hard Version)","group":"CodeChef - START221A","url":"https://www.codechef.com/START221A/problems/LMP8H","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n0\n0\n0\n3\n010\n010\n010\n4\n1100\n1010\n1001\n5\n00000\n11111\n01010\n6\n101010\n010101\n111000\n8\n00110101\n00100111\n11110000\n","output":"0\n0\n4\n19\n26\n49\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::payload::ValueDeltaPayload;
use algo_lib::collections::treap::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_delta::ValueTrait;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_str();
    let b = input.read_str();
    let c = input.read_str();

    let mut ans = 0;
    let mut points = vec![(0, 0)];
    let mut q0 = 0i64;
    let mut q1 = 0i64;
    for j in 0..n {
        if b[j] != c[j] {
            if a[j] == b'0' {
                if b[j] == b'0' {
                    q0 += 1;
                } else {
                    q0 -= 1;
                }
            } else {
                if b[j] == b'0' {
                    q1 += 1;
                } else {
                    q1 -= 1;
                }
            }
        }
        points.push((q0, q1));
    }
    points.sort();
    struct Node1;
    impl ValueTrait for Node1 {
        type V = (i64, usize, i64, i64, i64);

        fn join(v1: Self::V, v2: Self::V) -> Self::V {
            (v1.0, v1.1, v1.2 + v2.2, v1.3 + v2.3, v1.4 + v2.4)
        }
    }
    let mut tree = Tree::<ValueDeltaPayload<Node1>>::new();
    for (i, (x, y)) in points.copy_enumerate() {
        let p = tree
            .range(..&(y, n + 1, 0, 0, 0))
            .payload()
            .map(|x| x.v)
            .unwrap_or((0, 0, 0, 0, 0));
        ans += p.2 * x - p.3 + p.2 * y - p.4;
        tree.insert(ValueDeltaPayload::new((y, i, 1, x, y)));
    }
    struct Node2;
    impl ValueTrait for Node2 {
        type V = (i64, usize, i64, i64);

        fn join(v1: Self::V, v2: Self::V) -> Self::V {
            (v1.0, v1.1, v1.2 + v2.2, v1.3 + v2.3)
        }
    }
    points.sort_by_key(|&(x, y)| x + y);
    let mut tree = Tree::<ValueDeltaPayload<Node2>>::new();
    for (i, (x, y)) in points.copy_enumerate() {
        let p = tree
            .range(&(y, n + 1, 0, 0)..)
            .payload()
            .map(|x| x.v)
            .unwrap_or((0, 0, 0, 0));
        ans += p.2 * x - p.3;
        tree.insert(ValueDeltaPayload::new((y, i, 1, x)));
    }
    let mut tree = Tree::<ValueDeltaPayload<Node2>>::new();
    points.reverse();
    for (i, (x, y)) in points.copy_enumerate() {
        let p = tree
            .range(..&(x, 0, i64::MIN, i64::MIN))
            .payload()
            .map(|x| x.v)
            .unwrap_or((0, 0, 0, 0));
        ans += p.3 - p.2 * y;
        tree.insert(ValueDeltaPayload::new((x, i, 1, y)));
    }
    out.print_line(ans);
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
