//{"name":"Money Matters","group":"Kattis","url":"https://open.kattis.com/problems/moneymatters","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\n100\n-75\n-25\n-42\n42\n0 1\n1 2\n3 4\n","output":"POSSIBLE\n"},{"input":"4 2\n15\n20\n-10\n-25\n0 2\n1 3\n","output":"IMPOSSIBLE\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MoneyMatters"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let o = input.read_int_vec(n);
    let edges = input.read_size_pair_vec(m);

    let mut dsu = DSU::new(n);
    for (a, b) in edges {
        dsu.union(a, b);
    }
    let mut qty = vec![0; n];
    for i in 0..n {
        qty[dsu.find(i)] += o[i];
    }
    out.set_bool_output(BoolOutput::Custom("POSSIBLE", "IMPOSSIBLE"));
    out.print_line(qty.iter().all(|&x| x == 0));
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
