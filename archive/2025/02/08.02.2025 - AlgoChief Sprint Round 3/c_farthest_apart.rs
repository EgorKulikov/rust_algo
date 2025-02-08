//{"name":"C. Farthest Apart","group":"Codeforces - AlgoChief Sprint Round 3","url":"https://codeforces.com/gym/105705/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n2 9\n1 8\n3 7\n3\n10 60\n11 58\n12 59\n13 59\n","output":"5\n47\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x1 = input.read_int();
    let x2 = input.read_int();
    let pairs = input.read_int_pair_vec(n).sorted();

    let min_tail = Vec::with_gen_back(
        n + 1,
        |i, v| {
            if i == n {
                x2
            } else {
                pairs[i].1.min(v[i + 1])
            }
        },
    );
    let mut max_r = 0;
    let mut ans = None;
    for i in 0..n {
        max_r.maxim(pairs[i].1);
        let r = x2.min(max_r).min(min_tail[i + 1]);
        ans.maxim(r - pairs[i].0.max(x1));
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
