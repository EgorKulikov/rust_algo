//{"name":"E - Max × Sum","group":"AtCoder - AtCoder Beginner Contest 376","url":"https://atcoder.jp/contests/abc376/tasks/abc376_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 2\n3 7 6\n9 2 4\n5 3\n6 4 1 5 9\n8 6 5 1 7\n10 6\n61 95 61 57 69 49 46 47 14 43\n39 79 48 92 90 76 30 16 30 94\n","output":"42\n60\n14579\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMaxSum"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let mut ans = None;
    let mut sum = 0;
    let mut heap = BinaryHeap::new();
    let order = (0..n).collect_vec().do_with(|v| {
        v.sort_by_key(|&i| a[i]);
    });
    for i in order {
        sum += b[i];
        heap.push(b[i]);
        if heap.len() > k {
            sum -= heap.pop().unwrap();
        }
        if heap.len() == k {
            ans.minim(a[i] * sum);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
