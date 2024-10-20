//{"name":"B. Topological Sort","group":"Universal Cup - The 3rd Universal Cup. Stage 13: Sendai","url":"https://contest.ucup.ac/contest/1812/problem/9477","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 3 2\n","output":"4\n"},{"input":"5\n1 2 3 4 5\n","output":"1024\n"},{"input":"6\n4 2 1 5 6 3\n","output":"4096\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTopologicalSort"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::num_utils::powers;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    type Mod = ModIntF;
    let mut maxes = Vec::new();
    let mut ans = Mod::one();
    let twos = powers(Mod::new(2), n + 1);
    for i in 0..n {
        while let Some(&last) = maxes.last() {
            if p[last] > p[i] {
                ans *= twos[i] - twos[last];
                break;
            }
            maxes.pop();
        }
        if maxes.is_empty() {
            ans *= twos[i];
        }
        maxes.push(i);
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
