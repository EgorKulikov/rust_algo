//{"name":"1-D Frogger (Easy)","group":"Kattis","url":"https://open.kattis.com/problems/1dfroggereasy","interactive":false,"timeLimit":1000,"tests":[{"input":"6 4 42\n-9 1 42 -2 -3 -3\n","output":"magic\n2\n"},{"input":"8 2 13\n7 5 4 2 13 -2 -3 6\n","output":"cycle\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut s = input.read_int() - 1;
    let m = input.read_int();
    let a = input.read_int_vec(n);

    let mut visited = BitSet::new(n);
    for i in 0.. {
        if s < 0 {
            out.print_line("left");
            out.print_line(i);
            return;
        }
        if s >= n as i32 {
            out.print_line("right");
            out.print_line(i);
            return;
        }
        if visited[s as usize] {
            out.print_line("cycle");
            out.print_line(i);
            return;
        }
        if a[s as usize] == m {
            out.print_line("magic");
            out.print_line(i);
            return;
        }
        visited.set(s as usize);
        s += a[s as usize];
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
