//{"name":"G - Lightweight Knapsack","group":"AtCoder - JPRS Programming Contest 2026#1 (AtCoder Beginner Contest 442)","url":"https://atcoder.jp/contests/abc442/tasks/abc442_g","interactive":false,"timeLimit":4000,"tests":[{"input":"4 7\n3 5 5\n1 2 4\n2 7 1\n2 1 2\n","output":"16\n"},{"input":"2 1\n3 442 442\n2 442 442\n","output":"0\n"},{"input":"15 913575467\n1 60505998 818008580\n2 121011861 138996221\n3 181517958 501899080\n1 60506027 840594328\n3 181517875 350034067\n1 60505924 155374934\n3 181517816 910748511\n1 60506042 545531545\n3 181517877 797829355\n3 181517837 164163676\n1 60505894 353195922\n1 60505912 954291757\n1 60506022 160449218\n3 181517873 404011431\n1 60506043 782177068\n","output":"55276836358648682\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut c = input.read_size();
    let wvk = input.read_vec::<(usize, usize, usize)>(n);

    let (w, v, mut k) = wvk.detuple();
    let ok = k.clone();
    let order = (0..n)
        .collect::<Vec<_>>()
        .sorted_by_key(|&i| -Rational::new(v[i] as i64, w[i] as i64));
    let mut ans = 0;
    const BUBEN: usize = 50;
    for j in 0..n {
        let i = order[j];
        let q = k[i].min(c.saturating_sub(BUBEN) / w[i]);
        ans += v[i] * q;
        k[i] -= q;
        c -= w[i] * q;
        if k[i] != 0 {
            let mut need = vec![2; 4];
            for l in (0..j).rev() {
                let l = order[l];
                if need[w[l]] > 0 {
                    let cur = ok[l].min(need[w[l]]);
                    ans -= v[l] * cur;
                    c += w[l] * cur;
                    k[l] = cur;
                    need[w[l]] -= cur;
                }
            }
            break;
        }
    }
    if c >= BUBEN + 20 {
        out.print_line(ans);
        return;
    }
    let mut res = vec![0; c + 1];
    for i in 0..n {
        if k[i] == 0 {
            continue;
        }
        for j in (1..=c).rev() {
            let q = (j / w[i]).min(k[i]);
            for x in 1..=q {
                let cand = res[j - x * w[i]] + x * v[i];
                res[j].maxim(cand);
            }
        }
    }
    ans += res[c];
    out.print_line(ans);
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
