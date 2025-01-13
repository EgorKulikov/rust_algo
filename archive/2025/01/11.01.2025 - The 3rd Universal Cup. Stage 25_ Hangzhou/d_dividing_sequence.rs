//{"name":"D. Dividing Sequence","group":"Universal Cup - The 3rd Universal Cup. Stage 25: Hangzhou","url":"https://contest.ucup.ac/contest/1893/problem/9729","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n3 1 2 3 2\n3\n1 1 2\n3\n3 3 3\n5\n1 3 1 3 1\n5\n2 2 1 3 3\n","output":"1\n3\n3\n1 1 2\n2\n3 3\n3\n1 3 1\n4\n2 1 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDividingSequence"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = Vec::new();
    let mut vars = vec![(0, 0)];
    let mut len = n;
    for i in 0..n {
        let mut next_vars = Vec::new();
        for (p, q) in vars {
            if p != q {
                if ans[q] == a[i] {
                    next_vars.push((p, q + 1));
                } else if ans[q] > a[i] {
                    len.minim(p);
                }
            }
            if p >= ans.len() {
                ans.push(a[i]);
                next_vars.push((p + 1, q));
                if len > p {
                    len = n;
                }
                break;
            }
            if ans[p] == a[i] {
                next_vars.push((p + 1, q));
            } else if ans[p] > a[i] {
                ans[p] = a[i];
                next_vars.push((p + 1, q));
                ans.truncate(p + 1);
                if len > p {
                    len = n;
                }
                break;
            }
        }
        next_vars.sort_unstable();
        next_vars.dedup();
        vars = next_vars;
    }
    if let Some((cand, _)) = vars.get(0) {
        len.minim(*cand);
    }
    ans.truncate(len);
    out.print_line(len);
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
