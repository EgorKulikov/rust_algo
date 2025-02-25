//{"name":"P5 - Get It Twisted, They Will Divide Us","group":"DMOJ - Dilhan's Computing Contest 1","url":"https://dmoj.ca/problem/dcc1p5","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1\n011\n101\n110\n","output":"IMPOSSIBLE\n"},{"input":"6 2\n010001\n101000\n010100\n001010\n000101\n100010\n","output":"POSSIBLE\n010002\n102000\n020100\n001020\n000201\n200010\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::md_arr::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut a = input.read_char_table(n, n);

    if k == 1 {
        for i in 0..n {
            for j in 0..i {
                if a[i][j] == b'0' {
                    for k in 0..n {
                        if a[i][k] == b'1' {
                            a[i][k] = b'2';
                            a[k][i] = b'2';
                        }
                    }
                    out.print_line("POSSIBLE");
                    out.print_table(&a);
                    return;
                }
            }
        }
        out.print_line("IMPOSSIBLE");
    } else {
        if n == 2 {
            out.print_line("IMPOSSIBLE");
            return;
        }
        let bs = Vec::with_gen(n, |i| {
            let mut res = BitSet::new(n);
            for j in 0..n {
                if a[(i, j)] == b'1' {
                    res.set(j);
                }
            }
            res
        });
        let tail = ((64 - n % 64) % 64) as u32;
        for i in 0..n {
            for j in 0..i {
                let mut free = 0;
                let mut intersect = a[(i, j)] == b'1';
                for (&a, &b) in bs[i].raw_iter().zip(bs[j].raw_iter()) {
                    intersect |= (a & b) != 0;
                    free += (!(a | b)).count_ones();
                }
                if a[(i, j)] == b'0' {
                    free -= 2;
                }
                free -= tail;
                if free + (!intersect) as u32 >= 2 {
                    for k in 0..n {
                        if i != k && a[(i, k)] == b'1' {
                            a[(i, k)] = b'2';
                            a[(k, i)] = b'2';
                        }
                        if i != j && a[(j, k)] == b'1' {
                            a[(j, k)] = b'2';
                            a[(k, j)] = b'2';
                        }
                    }
                    out.print_line("POSSIBLE");
                    out.print_table(&a);
                    return;
                }
            }
        }
        out.print_line("IMPOSSIBLE");
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
