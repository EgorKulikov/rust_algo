//{"name":"E - Multiple Bonus","group":"AtCoder - AtCoder Weekday Contest 0032 Beta","url":"https://atcoder.jp/contests/awc0032/tasks/awc0032_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 6\n1 2 3 4 5\n2 3\n1 2 10\n2 5\n1 1 1\n2 1\n2 4\n","output":"6\n35\n2\n34\n"},{"input":"4 7\n5 1 4 2\n1 4 7\n2 4\n1 3 2\n2 2\n1 1 5\n2 3\n2 1\n","output":"19\n6\n27\n10\n"},{"input":"12 10\n3 1 4 1 5 9 2 6 5 3 5 8\n2 6\n1 3 4\n2 12\n1 5 7\n2 10\n1 2 1\n1 12 100\n2 12\n2 1\n2 11\n","output":"23\n68\n65\n188\n3\n75\n"},{"input":"20 14\n10 20 30 40 50 60 70 80 90 100 110 120 130 140 150 160 170 180 190 200\n2 20\n1 4 25\n1 6 10\n2 12\n1 1 3\n2 5\n1 10 100\n2 20\n1 7 8\n1 20 50\n2 19\n2 20\n1 3 6\n2 18\n","output":"2100\n875\n190\n2515\n2203\n2581\n2046\n"},{"input":"1 4\n1000000000\n2 1\n1 1 1000000000\n2 1\n2 1\n","output":"1000000000\n2000000000\n2000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_long_vec(n);

    let limit = n.lower_sqrt();
    let mut add = vec![0; limit];
    let mut ft = FenwickTree::from(s.as_slice());

    for _ in 0..q {
        let op = input.read_int();
        match op {
            1 => {
                let k = input.read_size();
                let v = input.read_long();
                if k <= limit {
                    add[k - 1] += v;
                } else {
                    for i in (k - 1..n).step_by(k) {
                        ft.add(i, v);
                    }
                }
            }
            2 => {
                let x = input.read_size();
                let mut ans = ft.get(..x);
                for i in 0..x.min(limit) {
                    ans += add[i] * ((x / (i + 1)) as i64);
                }
                out.print_line(ans);
            }
            _ => unreachable!(),
        }
    }
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
