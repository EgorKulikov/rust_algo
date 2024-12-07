//{"name":"E. Three Strings","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/E","interactive":false,"timeLimit":2500,"tests":[{"input":"7\na\nb\ncb\nab\ncd\nacbd\nab\nba\naabb\nxxx\nyyy\nxyxyxy\na\nbcd\ndecf\ncodes\nhorse\ncodeforces\negg\nannie\negaegaeg\n","output":"1\n0\n2\n0\n3\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EThreeStrings"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_str();
    let b = input.read_str();
    let c = input.read_str();

    let mut mem = Memoization2d::new(a.len() + 1, b.len() + 1, |mem, i, j| {
        if i == a.len() && j == b.len() {
            return 0;
        }
        let mut res = usize::MAX;
        if i != a.len() {
            res.minim(mem.call(i + 1, j) + (c[i + j] != a[i]) as usize);
        }
        if j != b.len() {
            res.minim(mem.call(i, j + 1) + (c[i + j] != b[j]) as usize);
        }
        res
    });
    out.print_line(mem.call(0, 0));
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
