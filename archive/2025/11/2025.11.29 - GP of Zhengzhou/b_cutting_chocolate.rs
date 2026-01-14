//{"name":"B. Cutting Chocolate","group":"Universal Cup - GP of Zhengzhou","url":"https://contest.ucup.ac/contest/2661/problem/15302","interactive":false,"timeLimit":1000,"tests":[{"input":"4 4 1\n1 1 0\n4\n1 2 1\n2 4 1\n3 1 1\n4 3 1\n","output":"1\n"},{"input":"3 3 3\n1 1 1\n3\n1 1 1\n2 2 2\n3 3 3\n","output":"0\n"},{"input":"9 9 9\n1 1 1\n8\n1 1 2\n2 2 8\n1 9 1\n2 8 9\n8 2 1\n9 2 9\n9 8 1\n8 9 7\n","output":"180\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _dim = input.read_size_vec(3);
    let cuts = input.read_size_vec(3);
    let n = input.read_size();
    let xyz = input.read_size_table(n, 3);

    let parts = cuts.copy_map(|c| c + 1).product::<usize>();
    if n % parts != 0 {
        out.print_line(0);
        return;
    }
    type Mod = ModInt7;
    let mut ans = Mod::one();
    let mut pos = Vec::new();
    for i in 0..3 {
        let vals = xyz.col(i).copied().collect::<Vec<_>>().sorted();
        let part = n / (cuts[i] + 1);
        let mut cur_pos = vec![0];
        for i in 0..cuts[i] {
            let end = vals[(i + 1) * part - 1];
            let start = vals[(i + 1) * part];
            ans *= start - end;
            cur_pos.push(start);
        }
        pos.push(cur_pos);
    }
    let mut qty = Arr3d::new(cuts[0] + 1, cuts[1] + 1, cuts[2] + 1, 0);
    for i in 0..n {
        let x = pos[0].upper_bound(&xyz[i][0]) - 1;
        let y = pos[1].upper_bound(&xyz[i][1]) - 1;
        let z = pos[2].upper_bound(&xyz[i][2]) - 1;
        qty[(x, y, z)] += 1;
    }
    if qty.iter_all(|v| v == n / parts) {
        out.print_line(ans);
    } else {
        out.print_line(0);
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
