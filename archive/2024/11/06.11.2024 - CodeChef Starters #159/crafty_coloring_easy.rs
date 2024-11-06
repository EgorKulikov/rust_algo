//{"name":"Crafty Coloring (Easy)","group":"CodeChef - START159A","url":"https://www.codechef.com/START159A/problems/CRCOLEZ","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 2 2\nAB\n3 2 1\nA\n3 4 2\nBA\n","output":"Bob\nAlice\nBob\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CraftyColoringEasy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Ordering;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    let delta = n.max(m) - n.min(m);
    let mut a = 0;
    let mut b = 0;
    for i in 0..n.min(m) {
        match s[i % k] {
            b'A' => a += delta + 1 + 2 * i,
            b'B' => b += delta + 1 + 2 * i,
            _ => unreachable!(),
        }
    }
    out.print_line(match a.cmp(&b) {
        Ordering::Less => "Bob",
        Ordering::Equal => "Draw",
        Ordering::Greater => "Alice",
    });
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
