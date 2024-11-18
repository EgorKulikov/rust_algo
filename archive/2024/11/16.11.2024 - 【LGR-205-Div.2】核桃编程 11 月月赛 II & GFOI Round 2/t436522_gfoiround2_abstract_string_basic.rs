//{"name":"T436522 「GFOI Round 2」Abstract String Basic","group":"Luogu","url":"https://www.luogu.com.cn/problem/T436522?contestId=210735","interactive":false,"timeLimit":1000,"tests":[{"input":"9\ncbbccxxxx\n","output":"godfather\n"},{"input":"26\nabcdefghijklmnopqrstuvwxyz\n","output":"abcdefghijklmnopqrstuvwxyz\n"},{"input":"28\naabcdefghijklmnopqrstuvwxyzz\n","output":"cybcdefghijklmnopqrstuvwxycy\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T436522GFOIRound2AbstractStringBasic"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut qty = [0; 26];
    for c in s.iter() {
        qty[(c - b'a') as usize] += 1;
    }
    let mut ans = Str::with_capacity(n);
    for c in s.iter() {
        let c = (c - b'a') as usize;
        qty[c] -= 1;
        let mut best = None;
        for i in 0..26 {
            best.minim((qty[i], i as u8 + b'a'));
        }
        ans.push(best.unwrap().1);
        qty[c] += 1;
    }
    out.print_line(ans);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
