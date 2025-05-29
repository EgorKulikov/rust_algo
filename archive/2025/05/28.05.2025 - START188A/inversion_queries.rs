//{"name":"Inversion Queries","group":"CodeChef - START188A","url":"https://www.codechef.com/START188A/problems/INVQUER","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 4\n4 2 3 5 1 6\n1 6\n3 6\n2 2\n4 6\n3 1\n3 2 1\n1 3\n","output":"6\n4\n0\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();

    let inv = a.inv();
    let mut ft = FenwickTree::new(n);
    let mut front = Vec::new();
    let mut total = 0;
    for i in inv.copy_iter() {
        total += ft.get(i..);
        ft.add(i, 1usize);
        front.push(total);
    }
    ft.clear();
    let mut back = Vec::new();
    total = 0;
    for i in inv.copy_rev() {
        total += ft.get(..=i);
        ft.add(i, 1);
        back.push(total);
    }
    back.reverse();

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size() - 1;
        if l == r {
            out.print_line(0);
            continue;
        }
        out.print_line(front[n - 1] - front[l] - back[r]);
    }
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
