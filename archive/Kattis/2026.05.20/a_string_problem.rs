//{"name":"A String Problem","group":"Kattis","url":"https://open.kattis.com/problems/stringproblem","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 5\n4 9\n6 3\n2 7\n0 8\n","output":"3\n4 8 9\n0 5 8\n1 9 5\n"},{"input":"5\n0 1\n3 2\n4 5\n6 7\n9 8\n","output":"4\n1 3 9\n4 9 3\n2 5 7\n3 7 5\n"},{"input":"4\n1 4\n6 3\n5 2\n7 0\n","output":"2\n0 4 6\n1 6 4\n"},{"input":"6\n3 9\n7 5\n10 2\n0 6\n1 11\n8 4\n","output":"6\n3 6 1\n4 1 2\n2 2 3\n0 3 4\n5 4 5\n1 5 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::mem::swap;

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let strings = input.read_size_pair_vec(n);

    let mut id = vec![0; 2 * n];
    for i in 0..n {
        id[strings[i].0] = i;
        id[strings[i].1] = i;
    }
    let mut qty = vec![0; 2 * n];
    for i in 0..n {
        let (a, b) = strings[i];
        let pos = (a + b) % (2 * n);
        if pos % 2 != 0 {
            qty[pos] += 1;
        }
    }
    let mut pos = qty.max_position();
    if pos % 2 == 0 {
        pos = 1;
    }
    let mut ans = Vec::new();
    let mut fixed = BitSet::new(2 * n);
    let mut done = BitSet::new(n);
    for mut i in 0..n {
        if done[i] {
            continue;
        }
        let start = i;
        loop {
            let (mut a, mut b) = strings[i];
            if (a + b) % (2 * n) == pos {
                break;
            }
            if fixed[b] {
                swap(&mut a, &mut b);
            }
            assert!(!fixed[b]);
            let target = (pos + 2 * n - b) % (2 * n);
            assert!(!fixed[target]);
            fixed.set(target);
            ans.push((i, a, target));
            done.set(i);
            i = id[target];
            if i == start {
                break;
            }
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
