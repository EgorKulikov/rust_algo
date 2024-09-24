//{"name":"E. Fun is Counting","group":"Codeforces - Theforces Round #34 (ABC-Forces)","url":"https://codeforces.com/gym/105350/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2\n1 1\n3\n2 2 2\n3\n2 1 2\n4\n1 2 3 2\n4\n3 3 2 2\n6\n3 3 2 2 3 3\n","output":"3\n1\n6\n0\n12\n60\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFunIsCounting"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).sorted();

    if a[0] == n - 1 {
        if n == 2 {
            out.print_line(3);
        } else {
            out.print_line(1);
        }
        return;
    }
    if a[0] + 1 < a[n - 1] {
        out.print_line(0);
        return;
    }
    let single = a.iter().count_eq(&&(a[n - 1] - 1));
    if single >= a[n - 1] {
        out.print_line(0);
        return;
    }
    let mult = a[n - 1] - single;
    if single + 2 * mult > n {
        out.print_line(0);
        return;
    }
    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + 1);
    let mut ans = c.c(n, single) * c.c(n - single, mult);
    if n > single + 2 * mult {
        ans *= c.comb_with_rep(mult, n - single - 2 * mult);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
