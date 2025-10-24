//{"name":"G. Navigator","group":"KEP.uz","url":"https://kep.uz/competitions/contests/contest/452/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"6 6\n1 2 5 8 10 15\n101101\n101010\nNEXT\nSET_CHECK 1\nNEXT\nSET_HIDE 1\nPREV\nNEAREST\n","output":"2\n5\n1\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let id = input.read_size_vec(n);
    let a = input.read_str();
    let b = input.read_str();

    let mut cur = Arr2d::new(3, 3, 0);
    let mut next = Arr3d::new(n, 3, 3, 0);
    for i in (0..n).rev() {
        let ca = (a[i] - b'0') as usize;
        let cb = (b[i] - b'0') as usize;
        for j in [ca, 2] {
            for k in [cb, 2] {
                cur[(j, k)] = i;
            }
        }
    }
    for i in (0..n).rev() {
        for j in 0..3 {
            for k in 0..3 {
                next[(i, j, k)] = cur[(j, k)];
            }
        }
        let ca = (a[i] - b'0') as usize;
        let cb = (b[i] - b'0') as usize;
        for j in [ca, 2] {
            for k in [cb, 2] {
                cur[(j, k)] = i;
            }
        }
    }
    let mut prev = Arr3d::new(n, 3, 3, 0);
    for i in 0..n {
        let ca = (a[i] - b'0') as usize;
        let cb = (b[i] - b'0') as usize;
        for j in [ca, 2] {
            for k in [cb, 2] {
                cur[(j, k)] = i;
            }
        }
    }
    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                prev[(i, j, k)] = cur[(j, k)];
            }
        }
        let ca = (a[i] - b'0') as usize;
        let cb = (b[i] - b'0') as usize;
        for j in [ca, 2] {
            for k in [cb, 2] {
                cur[(j, k)] = i;
            }
        }
    }

    let mut ca = 2;
    let mut cb = 2;
    let mut at = 0;
    for _ in 0..q {
        let command = input.read_str();
        match command.as_slice() {
            b"NEXT" => {
                at = next[(at, ca, cb)];
                out.print_line(id[at]);
            }
            b"PREV" => {
                at = prev[(at, ca, cb)];
                out.print_line(id[at]);
            }
            b"NEAREST" => {
                let na = next[(at, ca, cb)];
                let dist_n = if na >= at { na - at } else { n + na - at };
                let pa = prev[(at, ca, cb)];
                let dist_p = if pa <= at { at - pa } else { n + at - pa };
                if dist_n < dist_p || dist_n == dist_p && na < pa {
                    at = na;
                } else {
                    at = pa;
                }
                out.print_line(id[at]);
            }
            b"SET_CHECK" => {
                ca = ((input.read_int() + 3) % 3) as usize;
            }
            b"SET_HIDE" => {
                cb = ((input.read_int() + 3) % 3) as usize;
            }
            _ => unreachable!(),
        }
    }
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
