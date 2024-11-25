//{"name":"Archaeological Arithmetics","group":"TLX - TLX Regular Open Contest #41","url":"https://tlx.toki.id/contests/troc-41/problems/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4 6\n","output":"YA\n-3 0 3 6\n"},{"input":"3 10\n","output":"TIDAK\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ArchaeologicalArithmetics"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::iter::repeat;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let m = input.read_long();

    if m % n == 0 {
        out.print_line("YA");
        out.print_line_iter(repeat(m / n).take(n as usize));
    } else if n % 2 == 0 && m % (n / 2) == 0 {
        out.print_line("YA");
        let mut ans = Vec::with_capacity(n as usize);
        let x = (m / (n / 2) - n + 1) / 2;
        for i in 0..n {
            ans.push(x + i);
        }
        out.print_line(ans);
    } else {
        out.print_line("TIDAK");
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
