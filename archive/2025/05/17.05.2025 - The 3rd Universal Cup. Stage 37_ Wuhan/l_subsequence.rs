//{"name":"L. Subsequence","group":"Universal Cup - The 3rd Universal Cup. Stage 37: Wuhan","url":"https://contest.ucup.ac/contest/2025/problem/10747","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n7\n3 5 9 8 2 11 5\n7\n7 9 2 4 17 10 15\n1\n100\n2\n100 100\n","output":"5\n4\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n).sorted();

    let mut first = DefaultHashMap::new(n);
    let mut last = DefaultHashMap::new(0);
    for i in 0..n {
        first[a[i]].minim(i);
        last[a[i]].maxim(i);
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            if a[i] % 2 != a[j] % 2 {
                continue;
            }
            let mid = (a[i] + a[j]) / 2;
            if first[mid] == n {
                continue;
            }
            let best = (i + j) / 2;
            if best < first[mid] {
                let mut tail = j - first[mid];
                let mut head = first[mid] - i;
                head.minim(tail);
                tail.minim(head + 1);
                ans.maxim(head + tail + 1);
            } else if best > last[mid] {
                let mut tail = j - last[mid];
                let mut head = last[mid] - i;
                head.minim(tail);
                tail.minim(head + 1);
                ans.maxim(head + tail + 1);
            } else {
                ans.maxim(j - i + 1);
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
