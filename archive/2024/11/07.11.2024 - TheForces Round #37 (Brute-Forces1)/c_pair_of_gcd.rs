//{"name":"C. Pair of GCD","group":"Codeforces - TheForces Round #37 (Brute-Forces1)","url":"https://codeforces.com/gym/105491/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n3\n2 3 2\n4\n4 3 2 1\n6\n4 4 3 3 2 2\n9\n3 6 9 2 3 6 9 3 6\n","output":"3\n2\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPairOfGCD"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::HashSet;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut all = HashSet::new();
    all.insert(0);
    let mut but_one = HashSet::new();
    let mut but_two = HashSet::new();
    for i in a {
        let mut new_all = HashSet::new();
        let mut new_but_one = HashSet::new();
        let mut new_but_two = HashSet::new();

        for j in but_two {
            new_but_two.insert(gcd(i, j));
        }
        for j in but_one {
            new_but_one.insert(gcd(i, j));
            new_but_two.insert(j);
        }
        for j in all {
            new_all.insert(gcd(i, j));
            new_but_one.insert(j);
        }
        all = new_all;
        but_one = new_but_one;
        but_two = new_but_two;
    }
    out.print_line(but_two.into_iter().max());
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
