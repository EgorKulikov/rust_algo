//{"name":"Interval Less Query","group":"Eolymp - Basecamp - Educational Round #9","url":"https://eolymp.com/en/compete/1fck3aasuh3ev201dcp2lm8868/problem/8","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n1 3 2 4 3 10 5 5\n4\n1 8 5\n1 4 3\n5 8 9\n2 6 4\n","output":"5\n2\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Ordering;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let q = input.read_size();
    let queries = input.read_vec::<(usize, usize, i32)>(q).dec();

    #[derive(Eq, PartialEq)]
    enum Event {
        Add(usize, i32),
        Query(usize, usize, i32, usize),
    }

    impl Event {
        fn value(&self) -> i32 {
            match self {
                Event::Add(_, x) => *x,
                Event::Query(_, _, x, _) => *x,
            }
        }
    }

    impl PartialOrd for Event {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Event {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let v = self.value();
            let ov = other.value();
            if v != ov {
                return v.cmp(&ov);
            }
            match (self, other) {
                (&Event::Add(..), &Event::Query(..)) => Ordering::Greater,
                (&Event::Query(..), &Event::Add(..)) => Ordering::Less,
                _ => Ordering::Equal,
            }
        }
    }

    let mut events = Vec::with_capacity(n + q);
    for i in 0..n {
        events.push(Event::Add(i, a[i]));
    }
    for i in 0..q {
        let (l, r, x) = queries[i];
        events.push(Event::Query(l, r, x, i));
    }
    events.sort();
    let mut ans = vec![0; q];
    let mut ft = FenwickTree::new(n);
    for event in events {
        match event {
            Event::Add(i, _) => {
                ft.add(i, 1);
            }
            Event::Query(l, r, _, i) => {
                ans[i] = ft.get(l..=r);
            }
        }
    }
    out.print_per_line(&ans);
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
