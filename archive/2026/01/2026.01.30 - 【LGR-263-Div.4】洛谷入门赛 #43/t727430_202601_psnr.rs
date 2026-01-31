//{"name":"T727430 [语言月赛 202601] 峰值信噪比 / PSNR","group":"Luogu","url":"https://www.luogu.com.cn/problem/T727430?contestId=304401","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n178 239 196\n169 175 32\n248 205 45\n76 89 228\n217 65 83\n222 108 153\n242 17 68\n183 141 49\n5 54 90\n130 194 229\n82 23 243\n16 87 23\n154 185 192\n181 228 2\n215 194 29\n84 176 6\n83 210 28\n106 169 63\n","output":"7.67666099\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::md_arr::arr3d::Arr3dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::numbers::number_ext::Square;
use algo_lib::numbers::real::{IntoReal, Real};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let it = input.read_3d_table::<i64>(3, h, w);
    let kt = input.read_3d_table::<i64>(3, h, w);

    let d = Arr2d::with_gen(h, w, |i, j| {
        let mut res = 0;
        for k in 0..3 {
            res += (it[(k, i, j)] - kt[(k, i, j)]).square();
        }
        res
    });
    let mut s = Arr2d::new(h + 1, w + 1, 0);
    for (i, j) in d.indices() {
        s[(i + 1, j + 1)] = s[(i, j + 1)] + s[(i + 1, j)] - s[(i, j)] + d[(i, j)];
    }
    let mut min_m = Real(f64::INFINITY);
    for (i, j) in d.indices() {
        for k in i + w.upper_div(2)..=h {
            for l in j + h.upper_div(2)..=w {
                let sum = s[(k, l)] + s[(i, j)] - s[(k, j)] - s[(i, l)];
                min_m.minim(sum.into_real() / (3 * (k - i) * (l - j)));
            }
        }
    }
    out.print_line((Real(255.) / min_m.sqrt()).log10().into_real() * 20);
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
