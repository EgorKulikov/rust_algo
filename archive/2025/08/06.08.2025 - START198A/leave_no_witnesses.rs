//{"name":"Leave no Witnesses","group":"CodeChef - START198A","url":"https://www.codechef.com/START198A/problems/DISPART","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n010\n4\n0100\n5\n01011\n","output":"1\n-1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    if n.is_power_of_two() {
        out.print_line(-1);
        return;
    }
    let mut ans = 0;
    for i in 0.. {
        let x = (1 << i) - 1;
        if x >= n {
            break;
        }
        if s[x] != b'1' {
            ans += 1;
        }
    }
    let mut cur = n;
    loop {
        if s[cur - 1] != b'0' {
            ans += 1;
        }
        let ones = cur.trailing_ones();
        if cur.count_ones() == ones {
            break;
        }
        cur -= 1 << ones;
    }
    out.print_line(ans);
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
