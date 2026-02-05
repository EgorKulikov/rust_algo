//{"name":"E - Climbing Silver","group":"AtCoder - Denso Create Programming Contest 2026（AtCoder Beginner Contest 443）","url":"https://atcoder.jp/contests/abc443/tasks/abc443_e","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n5 3\n.###.\n..#..\n#.#.#\n#...#\n##..#\n2 2\n##\n..\n4 1\n####\n####\n####\n.###\n3 3\n...\n...\n...\n10 3\n##.##.##.#\n.####..#..\n...#.#..#.\n.#.#.#.#..\n...####...\n#.#.##....\n.##...#...\n#.##.....#\n#....###.#\n.#..#.#...\n","output":"10111\n11\n1000\n111\n0011010010\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let sc = input.read_size() - 1;
    let mut s = input.read_char_table(n, n);

    let mut first = Vec::with_gen(n, |i| {
        for j in (0..n).rev() {
            if s[(j, i)] == b'#' {
                return j;
            }
        }
        n
    });
    let mut can = BitSet::new(n);
    can.set(sc);
    let mut next = BitSet::new(n);
    for i in (1..n).rev() {
        for j in 0..n {
            if can[j] && first[j] <= i {
                for k in 0..=first[j] {
                    s[(k, j)] = b'.';
                }
                first[j] = n;
            }
        }
        next.fill(false);
        for j in 0..n {
            for k in j.saturating_sub(1)..(j + 2).min(n) {
                if can[k] && (s[(i - 1, j)] == b'.' || i - 1 == first[j]) {
                    next.set(j);
                    break;
                }
            }
        }
        swap(&mut can, &mut next);
    }
    for i in 0..n {
        out.print(can[i] as i32);
    }
    out.print_line(());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
