//{"name":"MEX of XOR","group":"CodeChef - START162A","url":"https://www.codechef.com/START162A/problems/MEXXOR","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 1 2\n9 25 16\n11 99 55\n99 8244 353\n69 69420 420\n","output":"0\n10\n48\n256\n384\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MEXOfXOR"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_iterator::iterate_with_base;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_int();
    let r = input.read_int();
    let x = input.read_int();

    let mut segs = Vec::new();
    for (_, len, prefix) in iterate_with_base(l, r, 2) {
        segs.push((prefix ^ ((x >> len) << len), 1 << len));
    }
    segs.sort();
    let mut cur = 0;
    for (i, len) in segs {
        if i != cur {
            break;
        }
        cur += len;
    }
    out.print_line(cur);
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
