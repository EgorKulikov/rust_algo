//{"name":"E. Matrix Transformation","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n12\n13\n2 2\n10 10\n42 42\n21 21\n21 21\n2 2\n74 10\n42 106\n21 85\n85 21\n2 4\n1 2 3 4\n5 6 7 8\n3 2 3 4\n1 0 1 0\n","output":"Yes\nYes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMatrixTransformation"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_table(n, m);
    let b = input.read_int_table(n, m);

    for i in 0..30 {
        let mut need_row = BitSet::new(n);
        let mut need_col = BitSet::new(m);
        let mut num_row = vec![0; n];
        let mut num_col = vec![0; m];
        for j in 0..n {
            for k in 0..m {
                if b[(j, k)].is_set(i) {
                    num_row[j] += 1;
                    if !a[(j, k)].is_set(i) {
                        need_col.set(k);
                    }
                } else {
                    num_col[k] += 1;
                    if a[(j, k)].is_set(i) {
                        need_row.set(j);
                    }
                }
            }
        }
        let mut row_queue = Vec::new();
        let mut col_queue = Vec::new();
        for i in 0..n {
            if num_row[i] == 0 {
                row_queue.push(i);
            }
        }
        for i in 0..m {
            if num_col[i] == 0 {
                col_queue.push(i);
            }
        }
        let mut done_row = BitSet::new(n);
        let mut done_col = BitSet::new(m);
        loop {
            let mut updated = false;
            if let Some(row) = row_queue.pop() {
                updated = true;
                for j in 0..m {
                    if !b[(row, j)].is_set(i) {
                        num_col[j] -= 1;
                        if num_col[j] == 0 {
                            col_queue.push(j);
                        }
                    }
                }
                done_row.set(row);
            }
            if let Some(col) = col_queue.pop() {
                updated = true;
                for j in 0..n {
                    if b[(j, col)].is_set(i) {
                        num_row[j] -= 1;
                        if num_row[j] == 0 {
                            row_queue.push(j);
                        }
                    }
                }
                done_col.set(col);
            }
            if !updated {
                break;
            }
        }
        for j in 0..n {
            if need_row[j] && !done_row[j] {
                out.print_line(false);
                return;
            }
        }
        for j in 0..m {
            if need_col[j] && !done_col[j] {
                out.print_line(false);
                return;
            }
        }
    }
    out.print_line(true);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
