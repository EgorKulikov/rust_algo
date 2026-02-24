//{"name":"L. Triangles","group":"Universal Cup - GP of St. Petersburg","url":"https://contest.ucup.ac/contest/3384/problem/17172","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n6 5\n3 6\n5 3\n2 4\n1 1\n4 2\n","output":"32\n5 2 3\n4 6 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::next_permutation::NextPermutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pts = input.read_size_pair_vec(n).dec();

    let third = n / 3;
    let mut t = Arr2d::new(3, 3, Vec::new());
    for i in 0..n {
        let (x, y) = pts[i];
        t[(x / third, y / third)].push(i + 1);
    }
    out.print_line(2 * third * third * 4);
    let mut p = (0..3).collect::<Vec<_>>();
    let mut ans = Vec::new();
    loop {
        loop {
            let mut good = true;
            for i in 0..3 {
                if t[(i, p[i])].is_empty() {
                    good = false;
                    break;
                }
            }
            if !good {
                break;
            }
            let mut cur = Vec::new();
            for i in 0..3 {
                cur.push(t[(i, p[i])].pop().unwrap());
            }
            ans.push(cur);
        }
        if !p.next_permutation() {
            break;
        }
    }
    assert_eq!(ans.len(), third);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
