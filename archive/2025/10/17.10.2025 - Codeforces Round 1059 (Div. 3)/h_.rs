//{"name":"H. Красивая задача","group":"Codeforces - Codeforces Round 1059 (Div. 3)","url":"https://codeforces.com/contest/2162/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 2\n1 1 3 4\n1 2\n2 4\n3 2\n1 1 3\n1 2\n2 3\n3 1\n1 1 1\n1 3\n9 3\n4 5 9 1 1 1 2 2 3\n1 6\n3 7\n7 9\n","output":"1011\n101\n111\n100100001\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::{powers, UpperDiv};
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n).sorted();
    let lr = input.read_size_pair_vec(m);

    let pos = Vec::with_gen(n + 1, |i| {
        let mut res = i;
        for (l, r) in lr.copy_iter() {
            if i >= l && i < r {
                res.minim(l - 1);
            }
        }
        res
    });
    let pow = powers(3u64, 40);
    let mut len = 0;
    let id = Vec::with_gen(n + 1, |i| {
        let was = len;
        len += (n + 1 - i) * (i + 1);
        was
    });
    let mut res = vec![0u64; len.upper_div(40)];
    let mut calc = RecursiveFunction3::new(|rec, p, i, j| {
        let at = id[i + j] + i * (n + 1 - (i + j)) + n - p;
        let index = at / 40;
        let val = res[index] / pow[at % 40] % 3;
        if val != 0 {
            return val == 2;
        }
        let mut ret = i == 0 && j == 0;
        if !ret && i > 0 && rec.call(p - 1, i - 1, j) {
            ret = true;
        }
        if !ret && pos[p] != p && i + j <= pos[p] && rec.call(pos[p], j, i) {
            ret = true;
        }
        if !ret && pos[p] == p && j > 0 && rec.call(p - 1, j - 1, i) {
            ret = true;
        }
        res[index] += pow[at % 40] * if ret { 2 } else { 1 };
        ret
    });
    let mut ans = Str::new();
    for i in 1..=n {
        let less = a.copy_filter(|x| *x < i).count();
        let greater = a.copy_filter(|x| *x > i).count();
        if calc.call(n, less, greater) || calc.call(n, greater, less) {
            ans.push(b'1');
        } else {
            ans.push(b'0');
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
