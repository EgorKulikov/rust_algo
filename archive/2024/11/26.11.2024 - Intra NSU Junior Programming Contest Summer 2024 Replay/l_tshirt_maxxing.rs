//{"name":"L - T-shirt-Maxxing","group":"LightOJ","url":"https://lightoj.com/contest/injpc-2024-replay/arena/problem/6463","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2\n1 5\n8 10\n3 6\n2 5\n6 9\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LTShirtMaxxing"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let events = input
        .read_size_pair_vec(n)
        .do_with(|v| v.sort_unstable_by_key(|x| x.1));

    let mut heap = BTreeSet::new();
    for i in 0..m {
        heap.insert((0, i));
    }
    let mut ans = 0;
    for (s, e) in events {
        if let Some(&(t, id)) = heap.prev(&(s, m)) {
            heap.remove(&(t, id));
            heap.insert((e, id));
            ans += 1;
        }
    }
    out.print_line(ans);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
