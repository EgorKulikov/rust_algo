//{"name":"Ladice","group":"Kattis","url":"https://open.kattis.com/problems/ladice","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\n1 2\n1 3\n1 2\n1 3\n1 2\n","output":"LADICA\nLADICA\nLADICA\nSMECE\nSMECE\n"},{"input":"9 10\n1 2\n3 4\n5 6\n7 8\n9 10\n2 3\n1 5\n8 2\n7 9\n","output":"LADICA\nLADICA\nLADICA\nLADICA\nLADICA\nLADICA\nLADICA\nLADICA\nLADICA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Ladice"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size();
    let ab = input.read_size_pair_vec(n).dec();

    let mut dsu = DSU::new(l);
    let mut qty = vec![0; l];
    for (a, b) in ab {
        let a = dsu.find(a);
        let b = dsu.find(b);
        if dsu.union(a, b) {
            qty[a] += qty[b];
        }
        if qty[a] == dsu.size(a) {
            out.print_line("SMECE");
        } else {
            qty[a] += 1;
            out.print_line("LADICA");
        }
    }
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
