//{"name":"D2. Minimum with Left Shift (Hard Version)","group":"Codeforces - TheForces Round #39 (1000-Forces)","url":"https://codeforces.com/gym/105672/problem/D2","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n1 2 3 4\n3\n2 2\n1 1 5\n2 3\n3\n1 -1 1\n3\n2 2\n1 3 -1000000000\n2 1000000000\n","output":"22\n32\n0\n-1999999998\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);

    let mut sum = a.copy_sum();
    let mut base = a
        .copy_enumerate()
        .map(|(i, x)| x * (i + 1) as i64)
        .sum::<i64>();
    let mut ft = FenwickTree::from(a.as_slice());

    let q = input.read_size();
    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let p = input.read_size() - 1;
                let x = input.read_long();
                let delta = x - a[p];
                sum += delta;
                base += delta * (p + 1) as i64;
                a[p] = x;
                ft.add(p, delta);
            }
            2 => {
                let k = input.read_long() % n as i64;
                out.print_line(base + ft.get(0..k as usize) * n as i64 - sum * k);
            }
            _ => unreachable!(),
        }
    }
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
//END MAIN
