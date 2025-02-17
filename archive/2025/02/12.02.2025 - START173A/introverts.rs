//{"name":"Introverts","group":"CodeChef - START173A","url":"https://www.codechef.com/START173A/problems/INTROVERTS","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n2 1\n6\n1 4 3 6 5 2\n7\n1 7 3 5 4 6 2\n","output":"YES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let o = p.inv();
    if o[0] != 0 && o[0] != n - 1 {
        out.print_line(false);
        return;
    }
    if n > 1 && o[1] + o[0] != n - 1 {
        out.print_line(false);
        return;
    }
    let mut lens = MultiTreeSet::new();
    lens.insert(n - 1);
    let mut segs = BTreeSet::new();
    segs.insert((0, n - 1));
    for i in o.iter_skip(2) {
        let seg = *segs.prev(&(i, 0)).unwrap();
        let max = *lens.last().unwrap();
        let seg_len = seg.1 - seg.0;
        if seg_len != max && (seg_len % 2 != 0 || seg_len + 1 < max) {
            out.print_line(false);
            return;
        }
        lens.remove(&seg_len);
        segs.remove(&seg);
        let (a, b) = seg;
        if (i - a).abs_diff(b - i) > 1 {
            out.print_line(false);
            return;
        }
        if a + 1 != i {
            lens.insert(i - a);
            segs.insert((a, i));
        }
        if i + 1 != b {
            lens.insert(b - i);
            segs.insert((i, b));
        }
    }
    out.print_line(true);
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
