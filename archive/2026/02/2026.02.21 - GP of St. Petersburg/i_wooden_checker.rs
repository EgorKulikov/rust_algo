//{"name":"I. Wooden Checker","group":"Universal Cup - GP of St. Petersburg","url":"https://contest.ucup.ac/contest/3384/problem/17169","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 2\n3 4\n3 1\n5 6\n5 1\n","output":"Forest 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::{BTreeMap, VecDeque};
use std::io::Write;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut dsu = DSU::new(n);
    struct Node {
        left: VecDeque<usize>,
        right: VecDeque<usize>,
    }
    let mut roots = BTreeMap::new();
    for i in 0..n {
        roots.insert(
            i,
            Node {
                left: VecDeque::new(),
                right: VecDeque::new(),
            },
        );
    }
    let mut good = BitSet::new(n);
    good.fill(true);

    for _ in 0..n - 1 {
        let a = input.read_size() - 1;
        let b = input.read_size() - 1;
        if !roots.contains_key(&b) {
            out.print_line("Bad oriented forest");
            return;
        }
        if !good[a] {
            writeln!(out, "Bad segment at {}", a + 1).unwrap();
            return;
        }
        let a_root = dsu.find(a);
        if a_root == b {
            out.print_line("Bad oriented forest");
            return;
        }
        if a < b {
            if *roots.next(&a_root).unwrap().0 != b {
                writeln!(out, "Bad segment at {}", a + 1).unwrap();
                return;
            }
            let mut node_b = roots.remove(&b).unwrap();
            let node_a = roots.get_mut(&a_root).unwrap();
            while let Some(x) = node_a.right.back() {
                if *x == a {
                    break;
                }
                good.unset(*x);
                node_a.right.pop_back();
            }
            for x in node_b.left.copy_iter() {
                good.unset(x);
            }
            node_a.right.push_back(b);
            if node_a.right.len() >= node_b.right.len() {
                node_a.right.extend(node_b.right);
            } else {
                swap(&mut node_a.right, &mut node_b.right);
                while let Some(x) = node_b.right.pop_back() {
                    node_a.right.push_front(x);
                }
            }
        } else {
            if *roots.prev(&a_root).unwrap().0 != b {
                writeln!(out, "Bad segment at {}", a + 1).unwrap();
                return;
            }
            let mut node_b = roots.remove(&b).unwrap();
            let node_a = roots.get_mut(&a_root).unwrap();
            while let Some(x) = node_a.left.back() {
                if *x == a {
                    break;
                }
                good.unset(*x);
                node_a.left.pop_back();
            }
            for x in node_b.right.copy_iter() {
                good.unset(x);
            }
            node_a.left.push_back(b);
            if node_a.left.len() >= node_b.left.len() {
                node_a.left.extend(node_b.left);
            } else {
                swap(&mut node_a.left, &mut node_b.left);
                while let Some(x) = node_b.left.pop_back() {
                    node_a.left.push_front(x);
                }
            }
        }
        dsu.union(a, b);
        out.print_line("Good");
        out.flush();
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
