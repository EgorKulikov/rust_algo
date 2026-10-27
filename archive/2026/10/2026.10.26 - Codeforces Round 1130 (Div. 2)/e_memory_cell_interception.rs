use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut n = input.read_size();
    let mut m = input.read_size();
    let mut a = input.read_str();
    let mut b = input.read_str();

    for _ in 0..2 {
        let mut at = Vec::new();
        let mut len = 0;
        let mut q = 0;
        let mut id = vec![0; n];
        for i in 0..n {
            if a[i] == b'R' {
                if at.len() == len {
                    at.push(Vec::new());
                }
                at[len].push(i);
                id[i] = q;
                q += 1;
                len += 1;
            } else {
                len = 0;
            }
        }
        let mut ans = vec![0; q];
        let mut set = BitSet::new(n + m);
        let mut ft = FenwickTree::new(n + m);
        let mut grow = Vec::new();
        for i in 0..m {
            if b[i] == b'W' {
                set.set(i + 1);
                ft.add(i + 1, 1);
                grow.push(i + 2);
            }
        }
        for k in at.indices() {
            for j in at[k].copy_iter() {
                if j == k {
                    ans[id[j]] = 1 + ft.get(..);
                } else {
                    ans[id[j]] = m + 1 + ft.get(m + 1..);
                }
            }
            let mut next_grow = Vec::new();
            for i in grow {
                if i < n + m && !set[i] {
                    set.set(i);
                    ft.add(i, 1);
                    next_grow.push(i + 1);
                }
            }
            grow = next_grow;
        }
        out.print_line(ans);
        swap(&mut a, &mut b);
        swap(&mut n, &mut m);
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
