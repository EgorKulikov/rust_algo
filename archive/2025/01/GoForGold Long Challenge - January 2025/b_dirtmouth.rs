//{"name":"B. Dirtmouth","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6 3\naacabc\n","output":"5\n"},{"input":"6 4\naaaaaa\n","output":"0\n"},{"input":"15 4\nabacabadabacaba\n","output":"16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let m = input.read_size();
    let s = input.read_str();

    let mut qty = Arr2d::new(m, m, 0usize);
    for (a, b) in s.consecutive_iter_copy() {
        let a = a as usize - b'a' as usize;
        let b = b as usize - b'a' as usize;
        qty[(a, b)] += 1;
        qty[(b, a)] += 1;
    }
    let ans = Vec::with_gen_prefix(1 << m, |i, ans| {
        let mut q = 0;
        for j in 0..m {
            if i.is_set(j) {
                for k in 0..m {
                    if !i.is_set(k) {
                        q += qty[(j, k)];
                    }
                }
            }
        }
        if i == 0 {
            q
        } else {
            let mut min = usize::MAX;
            for j in 0..m {
                if i.is_set(j) {
                    min.minim(ans[i.without_bit(j)]);
                }
            }
            min + q
        }
    });
    out.print_line(ans[Back(0)]);
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

//START MAIN
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
//END MAIN
