//{"name":"Calculating Dart Scores","group":"Kattis","url":"https://open.kattis.com/problems/calculatingdartscores","interactive":false,"timeLimit":1000,"tests":[{"input":"180\n","output":"triple 20\ntriple 20\ntriple 20\n"},{"input":"96\n","output":"triple 19\ndouble 15\nsingle 9\n"},{"input":"27\n","output":"triple 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CalculatingDartScores"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();

    fn solve(n: i32) -> Option<String> {
        if n <= 20 {
            return Some(format!("single {}", n));
        }
        if n <= 40 && n % 2 == 0 {
            return Some(format!("double {}", n / 2));
        }
        if n <= 60 && n % 3 == 0 {
            return Some(format!("triple {}", n / 3));
        }
        None
    }

    for i in 1..=60.min(n) {
        let s1 = solve(i);
        if s1.is_none() {
            continue;
        }
        if i == n {
            out.print_line(s1);
            return;
        }
        for j in 1..=i.min(n - i) {
            let s2 = solve(j);
            if s2.is_none() {
                continue;
            }
            if i + j == n {
                out.print_line(s1);
                out.print_line(s2);
                return;
            }
            for k in 1..=j.min(n - i - j) {
                let s3 = solve(k);
                if s3.is_none() {
                    continue;
                }
                if i + j + k == n {
                    out.print_line(s1);
                    out.print_line(s2);
                    out.print_line(s3);
                    return;
                }
            }
        }
    }
    out.print_line("impossible");
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
