//{"name":"A Night at Buckingham Palace","group":"Kattis","url":"https://open.kattis.com/problems/anightatbuckinghampalace","interactive":false,"timeLimit":1000,"tests":[{"input":"(11,LL) (7,LLL) (8,R) (5,) (4,L) (13,RL) (2,LLR) (1,RRR) (4,RR) () (3,L) (4,R) ()\n","output":"5 4 8 11 13 4 7 2 1\nincomplete\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::str_scan;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut map = FxHashMap::default();
    loop {
        let s = input.read_str();
        if s.len() == 2 {
            break;
        }
        str_scan!(&s, "(@,@", val: i32, key: Str);
        map.insert(Str::from(&key[..key.len() - 1]), val);
    }

    let mut total = 0;
    let mut levels = DefaultTreeMap::new(Vec::new());
    let mut rec = RecursiveFunction::new(|rec, key: Str| {
        if let Some(&val) = map.get(&key) {
            levels[key.len()].push(val);
            total += 1;
            let mut t = key.clone();
            t.push(b'L');
            rec.call(t);
            let mut t = key.clone();
            t.push(b'R');
            rec.call(t);
        }
    });
    rec.call(Str::new());
    if total == map.len() {
        out.print_line_iter(levels.into_iter().flat_map(|(_, v)| v));
    } else {
        out.print_line("incomplete");
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
