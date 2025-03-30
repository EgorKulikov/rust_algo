//{"name":"P2 - Pokemon Battles","group":"DMOJ - Dr. Anne Anderson Coding Contest 1","url":"https://dmoj.ca/problem/daacc1p2","interactive":false,"timeLimit":1000,"tests":[{"input":"FIRE\n5\n3 FIRE\n2 WATER\n6 GRASS\n3 GRASS\n4 FIRE\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    fn type_id(s: Str) -> i32 {
        match s.as_slice() {
            b"FIRE" => 0,
            b"WATER" => 1,
            b"GRASS" => 2,
            _ => panic!(),
        }
    }

    let e = type_id(input.read_str());
    let n = input.read_size();
    let mut ans = 0;
    for _ in 0..n {
        let damage = input.read_int();
        let t = type_id(input.read_str());
        let real_damage = if t == e {
            damage
        } else if (t + 1) % 3 == e {
            damage / 2
        } else {
            damage * 2
        };
        ans.maxim(real_damage);
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
