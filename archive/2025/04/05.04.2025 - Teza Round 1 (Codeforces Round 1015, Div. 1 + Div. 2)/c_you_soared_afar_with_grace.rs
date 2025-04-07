//{"name":"C. You Soared Afar With Grace","group":"Codeforces - Teza Round 1 (Codeforces Round 1015, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2084/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1 2\n1 2\n2\n1 2\n2 1\n4\n1 3 2 4\n2 4 1 3\n5\n2 5 1 3 4\n3 5 4 2 1\n5\n3 1 2 4 5\n1 2 3 4 5\n","output":"-1\n0\n1\n1 2\n2\n1 2\n1 3\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();
    let b = input.read_size_vec(n).dec();

    let mut id = vec![n; n];
    let mut has = FxHashMap::default();
    let mut mid = if n % 2 == 0 { None } else { Some(n / 2) };
    let mut x = 0;
    for i in 0..n {
        if a[i] == b[i] {
            if let Some(m) = mid {
                id[i] = m;
                mid = None;
            } else {
                out.print_line(-1);
                return;
            }
        } else {
            if let Some(&pos) = has.get(&((b[i], a[i]))) {
                id[pos] = x;
                id[i] = n - x - 1;
                x += 1;
            } else {
                has.insert((a[i], b[i]), i);
            }
        }
    }
    if id.copy_count(n) != 0 {
        out.print_line(-1);
        return;
    }
    let mut inv = id.inv();
    let mut ans = Vec::new();
    for i in 0..n {
        let j = inv[i];
        assert!(j >= i);
        if j > i {
            ans.push((i + 1, j + 1));
            id.swap(i, j);
            inv.swap(id[j], i);
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
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
