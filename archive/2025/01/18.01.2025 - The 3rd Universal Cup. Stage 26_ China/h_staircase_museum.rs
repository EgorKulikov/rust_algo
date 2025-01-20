//{"name":"H. Staircase Museum","group":"Universal Cup - The 3rd Universal Cup. Stage 26: China","url":"https://contest.ucup.ac/contest/1894/problem/9982","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2\n1 3\n1 3\n3\n1 2\n2 3\n3 3\n3\n1 1\n1 3\n3 3\n4\n1 2\n2 3\n3 4\n4 5\n","output":"2\n3\n3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::compress::{compress, Compressed};
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let lr = input.read_size_pair_vec(n).dec();

    let (l, r) = lr.detuple();
    let r = r.inc();
    let Compressed { arrs: [l, r], .. } = compress([&l, &r]);
    let m = r[Back(0)];
    let mut at = n;
    let u = Vec::with_gen_back(m, |i, _| {
        while l[at - 1] > i {
            at -= 1;
        }
        at
    });
    let c1 = r[0];
    let c2 = u[0];
    let ways = [r, u];
    let end = [m, n];
    let mut mem = Memoization2d::new(2, n.max(m) + 1, |mem, t, pos| -> usize {
        if pos == end[t] {
            1
        } else {
            let p1 = ways[t].upper_bound(&pos);
            let res = 1 + mem
                .call(t, ways[t][p1])
                .max(mem.call(1 - t, ways[1 - t][pos]));
            res
        }
    });
    out.print_line(mem.call(0, c1).max(mem.call(1, c2)));
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
