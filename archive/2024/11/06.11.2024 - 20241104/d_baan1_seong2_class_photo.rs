//{"name":"D - Baan1 Seong2 (Class Photo)","group":"LightOJ","url":"https://lightoj.com/contest/20241104/arena/problem/6346","interactive":false,"timeLimit":1000,"tests":[{"input":"7 3 6\n189 186 186 185 184 173 166\n1 6 2 4 7 5\n","output":"166 184 186 189 173 185\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBaan1Seong2ClassPhoto"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::VecDeque;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(q).dec();

    let mut order = VecDeque::new();
    for i in 0..n {
        if i % (2 * k) < k {
            order.push_front(a[i]);
        } else {
            order.push_back(a[i]);
        }
    }
    let ans = b.into_iter().map(|i| order[i]).collect::<Vec<_>>();
    if q > 0 {
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
