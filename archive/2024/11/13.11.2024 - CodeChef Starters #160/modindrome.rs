//{"name":"Modindrome","group":"CodeChef - START160A","url":"https://www.codechef.com/START160A/problems/MODINDROME","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 3\n3 2\n2 6\n4 5\n69 420\n","output":"5\n0\n48\n799\n465199511\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Modindrome"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    type Mod = ModIntF;
    let mut ans = Mod::zero();
    for x in 1..m {
        let small = m / x;
        let q_small = x - m % x;
        let big = small + 1;
        let q_big = m % x;
        let num_pairs = Mod::from_index(small) * Mod::from_index(small) * Mod::from_index(q_small)
            + Mod::from_index(big) * Mod::from_index(big) * Mod::from_index(q_big);
        let mut cur = num_pairs.power(n / 2);
        if n % 2 == 1 {
            cur *= Mod::from_index(m);
        }
        ans += cur;
    }
    let p = Mod::from_index(m).power((n + 1) / 2);
    ans -= p * Mod::from_index(m);
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
