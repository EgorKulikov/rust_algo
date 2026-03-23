//{"name":"B - Range Mex Sum","group":"AtCoder - AtCoder Regular Contest++ 216","url":"https://atcoder.jp/contests/arc216/tasks/arc216_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n0 -1 -1\n1 1\n2 2\n1 2\n1 3\n","output":"2\n0\n3\n6\n"},{"input":"5 3\n-1 2 -1 -1 1\n1 4\n3 5\n1 5\n","output":"6\n8\n30\n"},{"input":"15 1\n-1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1\n1 15\n","output":"612227903\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::inverses;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_utils::{factorials, PartialSums};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_int_vec(n);

    let mut ws = vec![0; n];
    let mut pos = vec![n; n];
    for i in 0..n {
        if a[i] == -1 {
            ws[i] = 1;
        } else {
            pos[a[i] as usize] = i;
        }
    }
    let ww = ws.partial_sums();
    type Mod = ModIntF;
    let f = factorials::<Mod>(ww[n] + 1);
    let inv = inverses::<u32, Mod>(ww[n] + 1);

    let mut val = Arr2d::new(ww[n] + 1, ww[n] + 1, Mod::new(0));

    for i in 0..=ww[n] {
        let mut cur = f[ww[n]];
        let mut next = ww[n];
        for j in (1..=i).rev() {
            val[(i, j)] = cur;
            cur *= j;
            cur *= inv[next];
            next -= 1;
        }
        val[(i, 0)] = cur;
    }

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let mut nw = ww[r] - ww[l];
        let base = nw;
        let mut ans = 0i64;
        // let mut cur = f[ww[n]];
        // let mut at = ww[n];
        for i in 0..n {
            if pos[i] != n {
                if pos[i] < l || pos[i] >= r {
                    break;
                }
            } else {
                if nw == 0 {
                    break;
                }
                nw -= 1;
            }
            ans += val[(base, nw)].val() as i64;
        }
        out.print_line(Mod::from(ans));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
