//{"name":"C - Cost to Flip","group":"AtCoder - AtCoder Regular Contest 194 (Div. 2)","url":"https://atcoder.jp/contests/arc194/tasks/arc194_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n0 1 1 1\n1 0 1 0\n4 6 2 9\n","output":"16\n"},{"input":"5\n1 1 1 1 1\n1 1 1 1 1\n1 1 1 1 1\n","output":"0\n"},{"input":"20\n1 1 1 1 0 0 1 1 0 0 0 1 0 1 0 1 1 0 1 0\n0 0 0 1 1 1 0 1 1 0 0 0 0 0 0 1 0 1 0 0\n52 73 97 72 54 15 79 67 13 55 65 22 36 90 84 46 1 2 27 8\n","output":"2867\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let c = input.read_long_vec(n);

    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| c[i]);
    let a = Vec::with_gen(n, |i| a[order[i]]);
    let b = Vec::with_gen(n, |i| b[order[i]]);
    let c = Vec::with_gen(n, |i| c[order[i]]);
    let pa = a.partial_sums();
    let pb = b.partial_sums();
    let tail = Vec::with_gen_back(n + 1, |i, v| {
        if i == n {
            0
        } else {
            let mut res = v[i + 1];
            if a[i] == 1 {
                res += (pa[n] - pa[i + 1]) * c[i];
            }
            if b[i] == 1 {
                res += (pb[n] - pb[i]) * c[i];
            }
            res
        }
    });
    let mut ops = 0;
    let mut per_op = 0;
    let mut per_prefix = 0;
    let mut per_suffix = 0;
    let mut preffix_ops = 0;
    let mut suffix_ops = 0;
    let mut ans = tail[0];
    let mut base = 0;
    for i in 0..n {
        if a[i] == 1 {
            if b[i] == 1 {
                per_op += c[i];
            } else {
                per_prefix += c[i];
                preffix_ops += 1;
                base -= c[i] * preffix_ops;
                ops += 1;
            }
        } else if b[i] == 1 {
            per_suffix += c[i];
            base -= c[i] * suffix_ops;
            suffix_ops += 1;
            ops += 1;
        }
        ans.minim(
            base + tail[i + 1]
                + (ops + pa[n] - pa[i + 1] + pb[n] - pb[i + 1]) * per_op
                + per_prefix * (preffix_ops + pa[n] - pa[i + 1])
                + per_suffix * (suffix_ops + pb[n] - pb[i + 1]),
        );
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
