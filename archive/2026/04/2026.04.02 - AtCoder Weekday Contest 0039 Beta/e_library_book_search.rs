//{"name":"E - Library Book Search","group":"AtCoder - AtCoder Weekday Contest 0039 Beta","url":"https://atcoder.jp/contests/awc0039/tasks/awc0039_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4 3 1\n1 100\n1 50\n2 200\n3 150\n1 2 100\n2 3 150\n1 3 10\n","output":"1\n1\n3\n"},{"input":"4 3 4 0\n4 120\n2 80\n4 200\n1 1 50\n1 3 100\n3 4 150\n2 2 1\n","output":"0\n0\n1\n1\n"},{"input":"8 12 8 2\n1 300\n2 120\n2 500\n3 450\n3 200\n4 600\n5 50\n5 700\n6 400\n7 800\n8 10\n8 900\n1 8 400\n2 5 200\n5 8 600\n1 3 1000\n4 4 1\n6 8 350\n3 7 750\n8 8 5\n","output":"5\n3\n1\n0\n0\n1\n0\n0\n"},{"input":"20 30 15 3\n1 100\n1 500\n2 250\n2 800\n3 150\n3 900\n4 400\n4 50\n5 1000\n5 300\n6 600\n6 610\n7 200\n7 700\n8 720\n9 330\n9 340\n10 100\n10 10000\n11 450\n12 460\n12 470\n13 480\n14 490\n15 5000\n16 510\n17 520\n18 530\n19 540\n20 550\n1 20 500\n1 5 200\n6 10 600\n10 10 1000\n11 15 460\n16 20 525\n3 7 700\n8 12 50\n13 17 480\n2 2 1\n4 9 1000\n15 15 4999\n1 20 10001\n9 14 335\n5 16 510\n","output":"12\n4\n2\n0\n2\n0\n0\n5\n2\n0\n0\n0\n0\n4\n5\n"},{"input":"1 1 5 1\n1 1000000000\n1 1 1\n1 1 999999999\n1 1 1000000000\n1 1 1000000000\n1 1 1000000000\n","output":"0\n0\n0\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let k = input.read_size();
    let sd = input.read_size_pair_vec(m);

    #[derive(Default)]
    struct Node {
        books: MultiTreapSet<usize>,
    }

    impl SegmentTreeNode for Node {}

    let mut books = vec![Vec::new(); n];
    for (s, d) in sd {
        books[s - 1].push(d);
    }

    let mut st = SegmentTree::with_gen_full(n, |l, r| {
        let mut res = Node::default();
        for i in l..r {
            for d in books[i].copy_iter() {
                res.books.insert(d);
            }
        }
        res
    });
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let t = input.read_size();
        out.print_line(
            st.for_each_mut(l..r, |res: usize, node| res + node.books.more_or_eq(&t))
                .saturating_sub(k),
        );
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
