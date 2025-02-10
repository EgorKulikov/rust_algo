//{"name":"Firefly","group":"Kattis","url":"https://open.kattis.com/problems/firefly","interactive":false,"timeLimit":1000,"tests":[{"input":"6 7\n1\n5\n3\n3\n5\n1\n","output":"2 3\n"},{"input":"14 5\n1\n3\n4\n2\n2\n4\n3\n4\n3\n3\n3\n2\n3\n3\n","output":"7 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_size();
    let obstacles = input.read_vec::<usize>(n);

    let mut ft = FenwickTree::new(h);
    for i in 0..n {
        if i % 2 == 0 {
            ft.add(0, 1);
            ft.add(obstacles[i], -1);
        } else {
            ft.add(h - obstacles[i], 1);
        }
    }
    let mut min = None;
    let mut qty = 0;
    for i in 0..h {
        let cur = ft.get(..=i);
        if min.minim(cur) {
            qty = 1;
        } else if min == Some(cur) {
            qty += 1;
        }
    }
    out.print_line((min, qty));
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
