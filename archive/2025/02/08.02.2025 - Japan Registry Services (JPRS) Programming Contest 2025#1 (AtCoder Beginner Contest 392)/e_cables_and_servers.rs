//{"name":"E - Cables and Servers","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2025#1 (AtCoder Beginner Contest 392)","url":"https://atcoder.jp/contests/abc392/tasks/abc392_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n1 1\n1 2\n2 1\n3 4\n4 4\n","output":"1\n1 1 3\n"},{"input":"4 3\n3 4\n4 1\n1 2\n","output":"0\n"},{"input":"5 4\n3 3\n3 3\n3 3\n3 3\n","output":"4\n1 3 5\n2 3 4\n3 3 2\n4 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let mut dsu = DSU::new(n);
    let mut unused = Vec::new();
    for (i, (u, v)) in edges.copy_enumerate() {
        if !dsu.union(u, v) {
            unused.push(i);
        }
    }
    let mut ans = Vec::new();
    for i in 0..n {
        while dsu.find(i) != dsu.find(0) {
            let id = unused.pop().unwrap();
            let (u, _) = edges[id];
            if dsu.find(u) == dsu.find(0) {
                ans.push((id + 1, u + 1, i + 1));
                dsu.union(i, u);
            } else {
                ans.push((id + 1, u + 1, 1));
                dsu.union(u, 0);
            }
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
