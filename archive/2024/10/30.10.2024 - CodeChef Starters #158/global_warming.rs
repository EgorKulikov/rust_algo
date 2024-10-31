//{"name":"Global Warming","group":"CodeChef - START158A","url":"https://www.codechef.com/START158A/problems/GLOBALWARMIN","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3 1\n3 2\n10 3\n54 23\n199900 131333\n2 1\n","output":"1\n2\n150\n715004866\n54312164\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GlobalWarming"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let twos = n - 1 - k;
    type Mod = ModIntF;
    let mut ans = Mod::zero();
    let c = Combinations::<Mod>::new(n);
    for end in 0..=1.min(twos) {
        for i in 0..=(twos - end) / 2 {
            let j = twos - end - 2 * i;
            if 2 * (i + j) > n - 2 - end {
                continue;
            }
            ans += Mod::from_index(n - 1 - i - j - end)
                * c.c(n - 2 - i - j - end, i + j)
                * c.c(i + j, i);
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
