//{"name":"F. Infinite Loop","group":"Universal Cup - The 3rd Universal Cup. Stage 22: Zhengzhou","url":"https://contest.ucup.ac/contest/1873/problem/9773","interactive":false,"timeLimit":1000,"tests":[{"input":"2 5 6\n1 1\n4 3\n1 1\n1 2\n2 1\n2 2\n3 1\n3 2\n","output":"1 1\n2 1\n2 2\n3 1\n3 2\n4 1\n"},{"input":"3 10 5\n2 4\n3 1\n10 7\n2 2\n7 1\n4 3\n5 2\n28 3\n","output":"3 1\n8 10\n6 2\n6 7\n34 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FInfiniteLoop"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let tasks = input.read_size_pair_vec(n);

    let mut cur_time = 0;
    let mut day_1 = Vec::new();
    let mut sum_b = 0;
    for (a, b) in tasks.copy_iter() {
        let end = cur_time.max(a) + b;
        day_1.push(end);
        cur_time = end;
        sum_b += b;
    }
    let mut day_2 = Vec::new();
    for (a, b) in tasks.copy_iter() {
        let end = cur_time.max(a + k) + b;
        day_2.push(end);
        cur_time = end;
    }

    let mut print = |mut x: usize| {
        x -= 1;
        if x % k == 0 {
            out.print_line((x / k, k));
        } else {
            out.print_line((x / k + 1, x % k));
        }
    };

    for _ in 0..q {
        let x = input.read_size();
        let y = input.read_size() - 1;

        if x == 1 {
            print(day_1[y]);
        } else if sum_b >= k {
            print(day_2[y] + sum_b * (x - 2));
        } else {
            print(day_2[y] + k * (x - 2));
        }
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
