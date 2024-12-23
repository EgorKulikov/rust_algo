//{"name":"Inflation","group":"Kattis","url":"https://open.kattis.com/problems/inflation2","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n2 1 1 2 5\n6\nINFLATION 1\nSET 3 2\nSET 5 2\nINFLATION 4\nSET 6 1\nSET 10 1\n","output":"16\n14\n14\n34\n14\n5\n"},{"input":"3\n1 4 1\n5\nSET 1 1\nSET 3 4\nINFLATION 2\nSET 3 1\nSET 6 4\n","output":"6\n6\n12\n8\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Inflation"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_long_vec(n);
    let q = input.read_size();

    let mut added = 0;
    let mut sum = p.copy_sum();
    let mut qty = DefaultHashMap::<_, i64>::new();
    for x in p {
        qty[x] += 1;
    }

    for _ in 0..q {
        let command = input.read_str();
        match command.as_slice() {
            b"INFLATION" => {
                let x = input.read_long();
                added += x;
                sum += x * n as i64;
            }
            b"SET" => {
                let x = input.read_long();
                let y = input.read_long();
                let q = qty[x - added];
                qty[x - added] = 0;
                qty[y - added] += q;
                sum += (y - x) * q;
            }
            _ => unreachable!(),
        }
        out.print_line(sum);
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
