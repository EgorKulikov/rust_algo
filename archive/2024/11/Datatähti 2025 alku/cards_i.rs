//{"name":"Cards I","group":"CSES - Datatähti 2025 alku","url":"https://cses.fi/524/task/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4 1 2\n2 0 1\n3 0 0\n2 1 1\n4 4 1\n","output":"YES\n1 4 3 2\n2 1 3 4\nNO\nYES\n1 2 3\n1 2 3\nYES\n1 2\n2 1\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CardsI"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size();
    let b = input.read_size();

    if a + b > n || a + b > 0 && a.min(b) == 0 {
        out.print_line(false);
        return;
    }
    let mut pa = Vec::with_capacity(n);
    let mut pb = Vec::with_capacity(n);
    for i in 1..=b {
        pa.push(i);
        pb.push(i + a);
    }
    for i in b + 1..=a + b {
        pa.push(i);
        pb.push(i - b);
    }
    for i in a + b + 1..=n {
        pa.push(i);
        pb.push(i);
    }
    out.print_line(true);
    out.print_line(pa);
    out.print_line(pb);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
