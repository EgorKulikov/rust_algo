//{"name":"E. Карточная игра","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"1 4\n","output":"2\n"},{"input":"2 2\n","output":"2\n"},{"input":"3 6\n","output":"1690\n"},{"input":"5 4\n","output":"568\n"},{"input":"500 500\n","output":"84693741\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKartochnayaIgra"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    type Mod = ModIntF;
    let mut mem_delta = Memoization2d::new(m + 1, m + 2, |mem, step, delta| {
        if step + delta > m {
            Mod::zero()
        } else if step == m {
            Mod::one()
        } else {
            let mut res = mem.call(step + 1, delta + 1);
            if delta > 0 {
                res += mem.call(step + 1, delta - 1);
            }
            res
        }
    });
    let mut delta_res = Vec::with_capacity(m + 1);
    for i in 0..=m {
        delta_res.push(mem_delta.call(0, i));
    }
    let mut mem = Memoization2d::new(n, m + 1, |mem, step, delta| {
        if step == n - 1 {
            if delta == 0 {
                Mod::one()
            } else {
                Mod::zero()
            }
        } else {
            let mut res = Mod::zero();
            for i in 0..=delta {
                res += delta_res[i] * mem.call(step + 1, delta - i);
            }
            res
        }
    });
    let mut ans = Mod::zero();
    for i in 0..=m {
        ans += mem.call(0, i) * delta_res[i];
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
