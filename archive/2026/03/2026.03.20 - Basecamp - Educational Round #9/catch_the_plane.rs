//{"name":"Catch the Plane","group":"Eolymp - Basecamp - Educational Round #9","url":"https://eolymp.com/en/compete/1fck3aasuh3ev201dcp2lm8868/problem/10","interactive":false,"timeLimit":3000,"tests":[{"input":"8 4\n1000\n0 1 0 900 0.2\n0 2 100 500 1.0\n2 1 500 700 1.0\n2 1 501 701 0.1\n0 3 200 400 0.5\n3 1 500 800 0.1\n3 0 550 650 0.9\n0 1 700 900 0.1\n","output":"0.3124\n"},{"input":"4 2\n2\n0 1 0 1 0.5\n0 1 0 1 0.5\n0 1 1 2 0.4\n0 1 1 2 0.2\n","output":"0.7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::real::Real;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let n = input.read_size();
    let _k = input.read_long();
    let buses = input
        .read_vec::<(i32, i32, i64, i64, Real)>(m)
        .sorted_by_key(|k| -k.2);

    enum EventType {
        Arrival,
        Departure,
        Update,
    }
    struct Event {
        t: EventType,
        id: usize,
    }
    let mut events = Vec::with_capacity(m * 3);
    for i in 0..m {
        events.push(Event {
            t: EventType::Arrival,
            id: i,
        });
        events.push(Event {
            t: EventType::Departure,
            id: i,
        });
        events.push(Event {
            t: EventType::Update,
            id: i,
        });
    }
    let key = |e: &Event| match e.t {
        EventType::Arrival => (-buses[e.id].3, 1),
        EventType::Departure => (-buses[e.id].2, 2),
        EventType::Update => (-buses[e.id].2, 3),
    };
    events.sort_by_key(key);
    let mut ans = vec![Real::zero(); n];
    ans[1] = Real::one();
    let mut pp = vec![Real::zero(); m];
    for event in events {
        let id = event.id;
        match event.t {
            EventType::Arrival => {
                pp[id] = ans[buses[id].1 as usize];
            }
            EventType::Departure => {
                pp[id] =
                    pp[id] * buses[id].4 + ans[buses[id].0 as usize] * (Real::one() - buses[id].4);
            }
            EventType::Update => {
                ans[buses[id].0 as usize].maxim(pp[id]);
            }
        }
    }
    out.set_precision(6);
    out.print_line(ans[0]);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
