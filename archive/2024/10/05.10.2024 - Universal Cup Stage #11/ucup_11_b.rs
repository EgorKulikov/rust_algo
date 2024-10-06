//{"name":"ucup_11_b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup_11_b"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n).sorted().do_with(|v| v.reverse());

    type Mod = ModIntF;
    let mut ans = vec![Mod::zero(); n + 1];
    let mut qty = vec![Mod::zero(); m + 1];
    qty[0] = Mod::one();
    let c = Combinations::<Mod>::new(n);
    for i in 0..n {
        let mut cur = Mod::zero();
        for j in (0..=m).rev() {
            let cand = qty[j];
            if j + a[i] >= m {
                cur += cand;
            }
            qty[(j + a[i]).min(m)] += cand;
        }
        // eprintln!("qty = {:?}", qty);
        for j in 0..n - i {
            ans[j] += cur * c.c(n - i - 1, j);
        }
        // eprintln!("ans = {:?}", ans);
    }
    out.print_per_line(&ans);
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
