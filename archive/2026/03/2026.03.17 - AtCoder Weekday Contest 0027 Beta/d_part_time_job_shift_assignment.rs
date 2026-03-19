//{"name":"D - Part-Time Job Shift Assignment","group":"AtCoder - AtCoder Weekday Contest 0027 Beta","url":"https://atcoder.jp/contests/awc0027/tasks/awc0027_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n3 100\n5 200\n2 150\n4 80\n1 50\n4 3 2\n","output":"330\n"},{"input":"4 3\n5 100\n5 200\n5 300\n5 400\n3 4 2\n","output":"-1\n"},{"input":"8 5\n10 500\n7 300\n15 800\n5 200\n12 600\n3 100\n8 450\n20 1000\n8 12 5 15 10\n","output":"2550\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let hs = input.read_long_pair_vec(n);
    let p = input.read_long_vec(m);

    #[derive(Eq, PartialEq)]
    enum Event {
        Job(i64, i64),
        Worker(i64),
    }

    impl Event {
        fn skill(&self) -> i64 {
            match self {
                Event::Job(h, _) => *h,
                Event::Worker(p) => *p,
            }
        }
    }

    impl PartialOrd for Event {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Event {
        fn cmp(&self, other: &Self) -> Ordering {
            let res = self.skill().cmp(&other.skill());
            if res != Ordering::Equal {
                return res;
            }
            match (self, other) {
                (Event::Job(_, _), Event::Worker(_)) => Ordering::Less,
                (Event::Worker(_), Event::Job(_, _)) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        }
    }

    let mut events = Vec::new();
    for i in 0..n {
        events.push(Event::Job(hs[i].0, hs[i].1));
    }
    for i in 0..m {
        events.push(Event::Worker(p[i]));
    }
    events.sort();
    let mut available = BinaryHeap::new();
    let mut ans = 0;
    for event in events {
        match event {
            Event::Job(_, c) => {
                available.push(c);
            }
            Event::Worker(_) => {
                if let Some(c) = available.pop() {
                    ans += c;
                } else {
                    out.print_line(-1);
                    return;
                }
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
