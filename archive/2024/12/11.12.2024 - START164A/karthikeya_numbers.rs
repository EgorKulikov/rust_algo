//{"name":"Karthikeya Numbers","group":"CodeChef - START164A","url":"https://www.codechef.com/START164A/problems/SPC2025Q8","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n27 3\n4 2\n4 5\n","output":"7\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KarthikeyaNumbers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::number_iterator::iterate_with_base;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let x = input.read_long();

    if x == 1 || x % 2 == 0 {
        out.print_line(0);
        return;
    }
    let mut ans = 0;
    for (mut prefix, len, _) in iterate_with_base(1, n, x) {
        let mut ok = true;
        while prefix != 0 {
            let d = prefix % x;
            prefix /= x;
            if d != 0 && d != x - 1 {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 2i64.power(len);
        }
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
