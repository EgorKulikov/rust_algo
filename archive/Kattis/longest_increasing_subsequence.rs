//{"name":"Longest Increasing Subsequence","group":"Kattis","url":"https://open.kattis.com/problems/longincsubseq","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n1 2 3 4 5 6 7 8 9 10\n10\n1 1 1 1 1 1 1 1 1 1\n10\n5 19 5 81 50 28 29 1 83 23\n","output":"10\n0 1 2 3 4 5 6 7 8 9\n1\n7\n5\n0 1 5 6 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LongestIncreasingSubsequence"}}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut val = Vec::new();
    let mut pos = Vec::new();
    let mut prev = Vec::with_capacity(n);
    for i in 0..n {
        let p = val.lower_bound(&a[i]);
        if p == 0 {
            prev.push(None);
        } else {
            prev.push(Some(pos[p - 1]));
        }
        if p == val.len() {
            val.push(a[i]);
            pos.push(i);
        } else {
            val[p] = a[i];
            pos[p] = i;
        }
    }
    let mut ans = Vec::with_capacity(val.len());
    let mut cur = pos[Back(0)];
    while let Some(p) = prev[cur] {
        ans.push(cur);
        cur = p;
    }
    ans.push(cur);
    ans.reverse();
    out.print_line(ans.len());
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
