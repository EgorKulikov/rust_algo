//{"name":"Cool Checker","group":"CodeChef - START173A","url":"https://www.codechef.com/START173A/problems/COOLCHECK","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n2 4\n1 3\n1 2 3 4\n2 2\n7 8\n7 8\n1 1\n5\n6\n","output":"-1\n2\n7 8\n1\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(m);

    if a.copy_count(a[0]) == n {
        if b.contains(&a[0]) {
            out.print_line(-1);
        } else {
            out.print_line(1);
            out.print_line(a[0]);
        }
        return;
    }
    if n > 20 {
        for i in 1..n {
            if a[i] != a[0] {
                for j in 1..=n - 1 {
                    if a[0] % j as i64 != a[i] % j as i64 {
                        let mut ans = vec![a[0]];
                        for k in 1.. {
                            if k != i {
                                ans.push(a[k]);
                                if ans.len() == j {
                                    break;
                                }
                            }
                        }
                        if ans.copy_sum() % j as i64 == 0 {
                            ans.clear();
                            let mut target = j - 1;
                            for k in 1.. {
                                ans.push(a[k]);
                                if k == i {
                                    target += 1;
                                }
                                if ans.len() == target {
                                    break;
                                }
                            }
                            if ans.len() == j - 1 {
                                ans.push(a[i]);
                            }
                            assert!(ans.copy_sum() % j as i64 != 0);
                        }
                        out.print_line(ans.len());
                        out.print_line(ans);
                        return;
                    }
                }
                unreachable!();
            }
        }
    }
    let set = b.into_iter().collect::<FxHashSet<_>>();
    for i in usize::iter_all(n).skip(1) {
        let mut sum = 0;
        let mut d = 0;
        for j in 0..n {
            if i.is_set(j) {
                sum += a[j];
                d += 1;
            }
        }
        if sum % d != 0 || !set.contains(&(sum / d)) {
            out.print_line(d);
            let mut ans = Vec::new();
            for j in 0..n {
                if i.is_set(j) {
                    ans.push(a[j]);
                }
            }
            out.print_line(ans);
            return;
        }
    }
    out.print_line(-1);
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
