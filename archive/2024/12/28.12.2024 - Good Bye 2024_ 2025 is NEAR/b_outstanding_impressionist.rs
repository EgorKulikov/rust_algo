//{"name":"B. Outstanding Impressionist","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n1 1\n1 1\n4\n1 3\n1 3\n1 3\n1 3\n6\n3 6\n2 2\n1 2\n1 1\n3 4\n2 2\n7\n3 4\n4 4\n4 4\n1 3\n2 5\n1 4\n2 2\n3\n4 5\n4 4\n5 5\n","output":"00\n1111\n100110\n1001111\n011\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOutstandingImpressionist"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let lr = input.read_size_pair_vec(n).dec();

    let mut qty = vec![0; 2 * n];
    let mut is_single = vec![0; 2 * n];
    for (l, r) in lr.copy_iter() {
        if l == r {
            qty[l] += 1;
            is_single[l] = 1;
        }
    }
    let p = is_single.partial_sums();
    let mut ans = Str::new();
    for (l, r) in lr {
        if l == r {
            if qty[l] == 1 {
                ans.push(b'1');
            } else {
                ans.push(b'0');
            }
        } else {
            if p[r + 1] - p[l] == r + 1 - l {
                ans.push(b'0');
            } else {
                ans.push(b'1');
            }
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
