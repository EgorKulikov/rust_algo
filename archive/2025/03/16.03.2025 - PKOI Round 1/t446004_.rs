//{"name":"T446004 不知道","group":"Luogu","url":"https://www.luogu.com.cn/problem/T446004?contestId=183510","interactive":false,"timeLimit":500,"tests":[{"input":"2 0\n","output":"0 1\n"},{"input":"4 1\n3\n","output":"0 748683265 748683265 499122177\n"},{"input":"8 3\n3 6 8\n","output":"0 291154603 291154603 582309206 166374059 166374059 748683265 748683265\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let t = input.read_size_vec(k).dec();

    if k == 0 {
        let mut ans = vec![0; n];
        ans[n - 1] = 1;
        out.print_line(ans);
        return;
    }
    type Mod = ModIntF;
    let mut fav = BitSet::new(n);
    let mut p = vec![0; n];
    fav.set(0);
    p[0] = 1;
    for t in t.copy_iter() {
        fav.set(t);
        p[t] = 1;
    }
    let p = p.partial_sums();
    let mut inv = Mod::one();
    for i in 0..n - 1 {
        inv /= Mod::new_signed(p[n - i]);
    }
    let mut x = Arr2d::new(n, n, Mod::zero());
    for i in 0..n {
        x[(0, i)] = Mod::one();
    }
    for i in 1..n {
        for j in i..n {
            x[(i, j)] =
                x[(i - 1, j - 1)] * Mod::new_signed(p[j + 1] - p[j - i + 1]) + x[(i, j - 1)];
        }
    }
    let mut mult = inv;
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        ans.push(mult * x[(n - i - 1, n - 1)]);
        mult *= Mod::new_signed(p[i + 1]);
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
