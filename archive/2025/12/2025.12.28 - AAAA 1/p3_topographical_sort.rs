//{"name":"P3 - Topographical Sort","group":"DMOJ - AAAA 1","url":"https://dmoj.ca/problem/aaaa1p3","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n6\n2 1 3 0 4 3\n","output":"2 3 2 1 0 0\n"},{"input":"2\n5\n1 3 3 2 1\n","output":"3 1 4 1 5\n"},{"input":"2\n5\n1 2 3 4 5\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::default_map::by_index;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_int();
    let n = input.read_size();

    if t == 1 {
        let h = input.read_size_vec(n);
        let mut enabled = BitSet::new(n);
        let mut islands = 1;
        let p = by_index(&h);
        let mut ans = Vec::with_capacity(n);
        for i in 0..n {
            for j in p[i].copy_iter() {
                islands += 1;
                if j == 0 || enabled[j - 1] {
                    islands -= 1;
                }
                if j == n - 1 || enabled[j + 1] {
                    islands -= 1;
                }
                enabled.set(j);
            }
            ans.push(islands);
        }
        out.print_line(ans);
    } else {
        let mut c = input.read_size_vec(n);
        while c.last() == Some(&0) {
            c.pop();
        }
        if c.contains(&0) {
            out.print_line(-1);
            return;
        }
        c.push(0);
        let mut was = 1;
        let mut ans = vec![n];
        let mut islands = VecDeque::new();
        islands.push_back(0);
        for i in c.indices() {
            if c[i] > was {
                for _ in 0..c[i] - was {
                    ans.push(i);
                    islands.push_back(ans.len());
                    ans.push(n);
                }
            } else {
                for _ in 0..was - c[i] {
                    let cur = islands.pop_front().unwrap();
                    ans[cur] = i;
                }
            }
            was = c[i];
            if ans.len() > n {
                out.print_line(-1);
                return;
            }
        }
        ans.resize(n, 0);
        out.print_line(ans);
    }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
