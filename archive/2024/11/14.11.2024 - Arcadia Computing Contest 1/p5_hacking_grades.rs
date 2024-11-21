//{"name":"P5 - Hacking Grades","group":"DMOJ - Arcadia Computing Contest 1","url":"https://dmoj.ca/problem/ahscc1p5","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 2 3\n2 5 3\n","output":"72.2222222\n"},{"input":"5 100\n2 5 9 9 3\n10 50 22 51 9\n","output":"59.7345122\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5HackingGrades"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::rational::Rational;
use algo_lib::numbers::real::{IntoReal, Real};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut a = input.read_vec::<i128>(n);
    let mut b = input.read_vec(n);

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((
            Rational::new(a[i] + 1, b[i] + 1) - Rational::new(a[i], b[i]),
            i,
        ));
    }
    for _ in 0..k {
        let (_, id) = heap.pop().unwrap();
        a[id] += 1;
        b[id] += 1;
        heap.push((
            Rational::new(a[id] + 1, b[id] + 1) - Rational::new(a[id], b[id]),
            id,
        ));
    }
    let mut ans = Real::zero();
    for (a, b) in a.into_iter().zip(b) {
        ans += a.into_real() / b;
    }
    ans /= n;
    ans *= 100;
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
