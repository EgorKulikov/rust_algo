//{"name":"A Rational Sequence 2","group":"Kattis","url":"https://open.kattis.com/problems/rationalsequence2","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1/1\n2 1/3\n3 5/2\n4 2178309/1346269\n","output":"1 1\n2 4\n3 11\n4 1431655765\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let id = input.read_size();
    scan!(input, "@/@", num: i64, den: i64);

    let mut num = num;
    let mut den = den;
    let mut dir = Vec::new();
    while num > 1 || den > 1 {
        if num > den {
            dir.push(Direction::Right);
            num -= den;
        } else {
            dir.push(Direction::Left);
            den -= num;
        }
    }
    let mut ans: usize = 1;
    let mut size = 1;
    for d in dir {
        match d {
            Direction::Left => {}
            Direction::Right => {
                ans += size;
            }
        }
        ans += size;
        size *= 2;
    }
    out.print_line((id, ans));
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
