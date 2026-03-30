//{"name":"Falling Multiset","group":"Eolymp - Basecamp - Weekend Practice #21 ","url":"https://eolymp.com/en/compete/idp1eoul596rpbbmp0mfbbo94g/problem/5","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n4 3\n1 2\n3 3\n2 1\n","output":"12\n38\n83\n94\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::prime_fft::PrimeFFT;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_utils::factorials;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let wv = input.read_size_pair_vec(n);

    type Mod = ModIntF;
    let mut fft = PrimeFFT::new();
    let buben = 5000;
    let f = factorials::<Mod>(n + 1);
    let mut a = vec![Mod::new(0); 2 * n];
    let mut bonus = vec![Mod::new(0); n];
    for (i, (w, v)) in wv.copy_enumerate() {
        a[Back(i + w)] += f[w] * v;
        if i + w <= n {
            bonus[i + w - 1] += f[w] * v;
        }
    }
    for i in 0..n - 1 {
        let a = bonus[i];
        bonus[i + 1] += a;
    }
    let mut ans = bonus;
    let mut b = vec![Mod::new(0); buben];
    let mut rem = n;
    let mut at = 0;
    loop {
        for (i, (w, v)) in wv.copy_enumerate() {
            if w > at && w <= at + buben {
                a[Back(i + w)] -= f[w] * v;
                let mut cur = Mod::from(v);
                let mut cw = w;
                let mut x = i;
                while cw - 1 > at && x < n {
                    cur *= cw;
                    cw -= 1;
                    ans[x] += cur;
                    x += 1;
                }
                rem -= 1;
            }
        }
        if rem == 0 {
            break;
        }
        for i in 0..buben {
            b[i] = Mod::new(1) / f[at + i + 1];
        }
        let res = fft.multiply(&a, &b);
        for i in 0..n {
            ans[Back(i)] += res[n - at - 2 + i];
        }
        at += buben;
    }
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
