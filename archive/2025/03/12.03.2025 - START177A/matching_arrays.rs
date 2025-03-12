//{"name":"Matching Arrays","group":"CodeChef - START177A","url":"https://www.codechef.com/START177A/problems/MATCHARR","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 2\n1 1 2\n1 2\n4 2\n1 2 1 2\n1 2\n3 3\n3 2 1\n3 2 1\n2 2\n1 1\n2 2\n4 1\n1 1 1 1\n1\n","output":"1\n4\n1\n0\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n).dec();
    let b = input.read_size_vec(m).dec();

    let next = Vec::with_gen_back(n + 1, |i, v: &Vec<Vec<usize>>| {
        if i == n {
            vec![n; m]
        } else {
            let mut res = v[i + 1].clone();
            res[a[i]] = i;
            res
        }
    });
    let next_sp = Vec::with_gen_back(n + 1, |i, v: &Vec<Vec<usize>>| {
        if i == n {
            vec![n; m]
        } else {
            let mut res = v[i + 1].clone();
            if i == n - 1 || a[i + 1] != a[i] {
                res[a[i]] = i;
            }
            res
        }
    });
    let mut ans = 0;
    for i in 0..n {
        if a[i] != b[0] {
            continue;
        }
        let mut pos = i;
        let mut at = 0;
        while at < m {
            if b[at] == b[0] {
                if pos < n && a[pos] == b[0] {
                    pos += 1;
                    at += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if pos < n && a[pos] == b[0] {
            if at == m {
                ans += 1;
            }
            continue;
        }
        for j in at..m {
            if j + 1 == m || b[j + 1] == b[j] {
                pos = next[pos][b[j]] + 1;
            } else {
                pos = next_sp[pos][b[j]] + 1;
            }
            if pos == n + 1 {
                break;
            }
        }
        ans += n + 1 - pos;
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
