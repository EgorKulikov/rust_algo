//{"name":"MEXtreme Divisibility","group":"CodeChef - START157A","url":"https://www.codechef.com/START157A/problems/ONEDIF","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 1\n1 1 1 1\n5 5\n1 3 4 2 2\n4 5\n2 1 5 2\n7 20\n15 20 13 2 17 20 12\n","output":"2\n6\n4\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MEXtremeDivisibility"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::lcm;
use algo_lib::numbers::primes::factorize::all_divisors;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);

    let divisors = all_divisors::<usize>(m + 1, false);
    #[derive(Clone)]
    enum Time {
        Zero,
        Once(usize),
        Twice,
    }

    impl Time {
        fn add(&mut self, id: usize) {
            match self {
                Time::Zero => {
                    *self = Time::Once(id);
                }
                Time::Once(_) => {
                    *self = Time::Twice;
                }
                Time::Twice => {}
            }
        }
    }
    let mut times = vec![Time::Zero; m + 1];
    for i in 0..n {
        for &j in &divisors[a[i]] {
            times[j].add(i);
        }
    }
    let mut to_add = 1;
    let mut repl = vec![1; n];
    let mut all = BTreeSet::new();
    for i in 0..n {
        all.insert((1, i));
    }
    for i in 1..=m {
        let mut check = false;
        match times[i] {
            Time::Zero => {
                to_add = lcm(to_add, i);
                check = true;
            }
            Time::Once(id) => {
                all.remove(&(repl[id], id));
                repl[id] = lcm(repl[id], i);
                all.insert((repl[id], id));
                check = true;
            }
            Time::Twice => {}
        }
        if check {
            if to_add > m {
                out.print_line(i);
                return;
            }
            let min = all.iter().next().unwrap().0;
            if lcm(to_add, min) <= m {
                continue;
            }
            let mut found = false;
            for i in 0..n {
                if lcm(repl[i], to_add) <= m {
                    found = true;
                    break;
                }
            }
            if !found {
                out.print_line(i);
                return;
            }
        }
    }
    out.print_line(m + 1);
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
