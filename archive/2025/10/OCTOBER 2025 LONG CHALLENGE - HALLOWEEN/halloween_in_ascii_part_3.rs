//{"name":"halloween_in_ascii_part_3","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let mut cols = Vec::new();
    loop {
        let s = input.read_line();
        if s.as_slice() == b"=====" {
            break;
        }
        cols.push(s);
    }
    let mut rows = Vec::new();
    loop {
        let s = input.read_line();
        if s.as_slice() == b"=====" {
            break;
        }
        rows.push(s);
    }

    let h = rows.len();
    let w = rows[0].len();
    let mut row_sets = vec![((0..h).collect::<Vec<_>>(), (0..h).collect::<Vec<_>>())];
    let mut col_sets = vec![((0..w).collect::<Vec<_>>(), (0..w).collect::<Vec<_>>())];
    while col_sets.len() < w {
        let mut new_row_sets = Vec::new();
        for (set, indices) in row_sets {
            let mut by_cert_r = DefaultHashMap::new(Vec::new());
            let mut by_cert_c = DefaultHashMap::new(Vec::new());
            for i in set {
                let mut cert = Vec::new();
                for j in col_sets.indices() {
                    for k in col_sets[j].1.copy_iter() {
                        cert.push((j, rows[i][k]));
                    }
                }
                cert.sort();
                by_cert_r[cert].push(i);
            }
            for i in indices {
                let mut cert = Vec::new();
                for j in col_sets.indices() {
                    for k in col_sets[j].0.copy_iter() {
                        cert.push((j, cols[i + 1][k]));
                    }
                }
                cert.sort();
                by_cert_c[cert].push(i);
            }
            let cert_r = by_cert_r.into_iter().collect::<Vec<_>>().sorted();
            let cert_c = by_cert_c.into_iter().collect::<Vec<_>>().sorted();
            new_row_sets.extend(cert_r.iter_map(|(_, v)| v).zip(cert_c.iter_map(|(_, v)| v)));
        }
        row_sets = new_row_sets;
        let mut new_col_sets = Vec::new();
        for (set, indices) in col_sets {
            let mut by_cert_r = DefaultHashMap::new(Vec::new());
            let mut by_cert_c = DefaultHashMap::new(Vec::new());
            for i in set {
                let mut cert = Vec::new();
                for j in row_sets.indices() {
                    for k in row_sets[j].1.copy_iter() {
                        cert.push((j, cols[k + 1][i]));
                    }
                }
                cert.sort();
                by_cert_c[cert].push(i);
            }
            for i in indices {
                let mut cert = Vec::new();
                for j in row_sets.indices() {
                    for k in row_sets[j].0.copy_iter() {
                        cert.push((j, rows[k][i]));
                    }
                }
                cert.sort();
                by_cert_r[cert].push(i);
            }
            let cert_c = by_cert_c.into_iter().collect::<Vec<_>>().sorted();
            let cert_r = by_cert_r.into_iter().collect::<Vec<_>>().sorted();
            new_col_sets.extend(cert_c.iter_map(|(_, v)| v).zip(cert_r.iter_map(|(_, v)| v)));
        }
        col_sets = new_col_sets;
    }
    col_sets.sort_by_key(|x| x.1[0]);
    let mut s = Str::new();
    for i in 0..w {
        s.push(cols[0][col_sets[i].0[0]]);
    }
    let pos = s.iter().position(|&c| c == b'>').unwrap();
    let to = s[pos..].iter().position(|&c| c == b'<').unwrap() + pos;
    let ans = Str::from(&s[pos + 1..to]);
    writeln!(out, "Case #{}: {}", test_case, ans).unwrap();
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
