//{"name":"Glass Bridge (Easy)","group":"CodeChef - START159A","url":"https://www.codechef.com/START159A/problems/SQUIDGAME2EZ","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2\n1 1\n2 3\n","output":"500000004 750000006\n500000004\n0 250000002 500000004\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GlassBridgeEasy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::num_utils::powers;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    type Mod = ModInt7;
    let c = Combinations::<Mod>::new(n + m + 1);
    let p = powers(Mod::new(2).inv().unwrap(), n + m + 1);
    let mut ans = vec![Mod::zero(); m];
    let mut total = Mod::zero();
    for i in n - 1..m {
        total += c.c(i, n - 1) * p[i + 1];
        ans[i] = total;
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
