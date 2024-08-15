//{"name":"H. Ксюша и загруженное множество","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/H","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n5\n1 2 5 905 2000000\n15\n- 2\n? 2\n? 1\n- 1\n? 1\n+ 4\n+ 2\n? 2\n+ 6\n- 4\n+ 7\n? 2\n? 3\n? 4\n? 2000000\n5\n3 4 5 6 8\n9\n? 5\n- 5\n? 5\n+ 1\n? 2\n- 6\n- 8\n+ 6\n? 5\n5\n6 7 8 9 10\n10\n? 5\n- 6\n? 4\n- 10\n+ 5\n- 8\n+ 3\n+ 2\n- 3\n+ 10\n","output":"2 2 1 6 3 8 8 2000001\n9 9 9 7\n1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HKsyushaIZagruzhennoeMnozhestvo"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::cell::Cell;
use std::collections::BTreeSet;

type PreCalc = SegmentTree<Node>;

#[derive(Default)]
struct Node {
    added: BTreeSet<usize>,
    reset: bool,
}

enum Operation {
    Add(usize),
    Remove(usize),
    Reset,
}

impl SegmentTreeNode for Node {
    fn new(_left: usize, _right: usize) -> Self {
        Default::default()
    }

    fn join(
        &mut self,
        _left_val: &Self,
        _right_val: &Self,
        _left: usize,
        _mid: usize,
        _right: usize,
    ) {
    }

    fn accumulate(&mut self, value: &Self, _left: usize, _right: usize) {
        if value.reset {
            self.reset = true;
            self.added.clear();
        }
    }

    fn reset_delta(&mut self, _left: usize, _right: usize) {
        self.reset = false;
    }
}

impl Pushable<&Operation> for Node {
    fn push(&mut self, op: &Operation, _left: usize, _right: usize) {
        match op {
            Operation::Add(x) => {
                assert!(self.added.insert(*x));
            }
            Operation::Remove(x) => {
                assert!(self.added.remove(x));
            }
            Operation::Reset => {
                self.reset = true;
                self.added.clear();
            }
        }
    }
}

impl Pushable<&Cell<Option<usize>>> for Node {
    fn push(&mut self, delta: &Cell<Option<usize>>, _left: usize, _right: usize) {
        if let Some(&val) = self.added.first() {
            let mut d = delta.get();
            d.minim(val);
            delta.set(d);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, tree: &mut PreCalc) {
    tree.update(.., &Operation::Reset);

    let n = input.read_size();
    let a = input.read_size_vec(n);

    tree.update(..a[0] - 1, &Operation::Add(1));
    for (&f, &t) in a.consecutive_iter() {
        tree.update(..t - f - 1, &Operation::Add(f + 1));
    }
    tree.update(.., &Operation::Add(a[n - 1] + 1));
    let mut a = a.into_iter().collect::<BTreeSet<_>>();
    a.insert(0);

    let m = input.read_size();
    for _ in 0..m {
        let tp = input.read_char();
        match tp {
            '+' => {
                let x = input.read_size();
                let last = *a.last().unwrap();
                if last < x {
                    tree.update(.., &Operation::Remove(last + 1));
                    tree.update(.., &Operation::Add(x + 1));
                    tree.update(..x - last - 1, &Operation::Add(last + 1));
                } else {
                    let prev = a.prev(&x).unwrap();
                    let next = a.next(&x).unwrap();
                    tree.update(..next - prev - 1, &Operation::Remove(prev + 1));
                    tree.update(..x - prev - 1, &Operation::Add(prev + 1));
                    tree.update(..next - x - 1, &Operation::Add(x + 1));
                }
                a.insert(x);
            }
            '-' => {
                let x = input.read_size();
                let last = *a.last().unwrap();
                if last == x {
                    let last = a.prev(&x).unwrap();
                    tree.update(.., &Operation::Remove(x + 1));
                    tree.update(..x - last - 1, &Operation::Remove(last + 1));
                    tree.update(.., &Operation::Add(last + 1));
                } else {
                    let prev = a.prev(&x).unwrap();
                    let next = a.next(&x).unwrap();
                    tree.update(..x - prev - 1, &Operation::Remove(prev + 1));
                    tree.update(..next - x - 1, &Operation::Remove(x + 1));
                    tree.update(..next - prev - 1, &Operation::Add(prev + 1));
                }
                a.remove(&x);
            }
            '?' => {
                let k = input.read_size();
                let ans = Cell::new(None);
                tree.point_through_update(k - 1, &ans);
                out.print_line(ans.get());
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = SegmentTree::new(2_000_000);

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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
