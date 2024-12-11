//{"name":"P11362 [NOIP2024] 遗失的赋值（民间数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11362?contestId=217331","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 1 2\n1 1\n2 2 2\n1 1\n2 2\n2 2 2\n1 1\n1 2\n","output":"4\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11362NOIP2024"}}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let v = input.read_int();
    let restrictions = input.read_size_pair_vec(m).dec().sorted();

    type Mod = ModInt7;
    let mut ans = Mod::new(v).power(2 * restrictions[0].0 + 2 * (n - 1 - restrictions[Back(0)].0));
    for (&(a, x_a), &(b, x_b)) in restrictions.consecutive_iter() {
        if a == b {
            if x_a != x_b {
                out.print_line("0");
                return;
            }
            continue;
        }
        ans *= Mod::new(v).power(2 * (b - a)) - Mod::new(v).power(b - a - 1) * Mod::new(v - 1);
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
