//{"name":"Yet another Beautiful String","group":"SeriousOJ - Happy New Year 2025","url":"https://judge.eluminatis-of-lu.com/contest/676ffd92569fb90008aac7da/1158","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n10\ncccacbaaac\n10\nacbbccaaca\n5\nbacba\n2\nab\n","output":"1\n1\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"YetAnotherBeautifulString"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::sieve::primes;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let p: Vec<usize> = primes(2000);

    let mut reps = vec![Vec::new(); 2001];
    for i in p.copy_iter() {
        reps[i].push((i, 0, 0));
        for j in p.copy_iter() {
            if i + j > 2000 {
                break;
            }
            reps[i + j].push((i, j, 0));
            for k in p.copy_iter() {
                if i + j + k > 2000 {
                    break;
                }
                reps[i + j + k].push((i, j, k));
                if j == k {
                    break;
                }
            }
            if i == j {
                break;
            }
        }
    }
    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let s = input.read_str();

        let mut q = Vec::with_capacity(3);
        for c in b'a'..=b'c' {
            q.push(s.copy_count(c));
        }
        q.sort_by_key(|x| Reverse(*x));
        let mut ans = None;
        for (i, j, k) in reps[n].copy_iter() {
            ans.minim((i.abs_diff(q[0]) + j.abs_diff(q[1]) + k.abs_diff(q[2])) / 2);
        }
        out.print_line(ans);
    }
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
