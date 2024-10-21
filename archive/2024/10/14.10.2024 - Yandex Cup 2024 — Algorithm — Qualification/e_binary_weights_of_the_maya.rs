//{"name":"E. Binary Weights of the Maya","group":"Yandex - Yandex Cup 2024 — Algorithm — Qualification","url":"https://contest.yandex.com/contest/69390/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3\n0 1 2\n","output":"4\n"},{"input":"20\n4\n1 3 5 6\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBinaryWeightsOfTheMaya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(m);

    let mut res = vec![None; (n + 1).min(1_000_000)];
    type Mod = ModInt7;
    let mut rec = RecursiveFunction::new(|f, x| {
        if x < res.len() {
            if let Some(ans) = res[x] {
                return ans;
            }
        }
        if x == 0 {
            return Mod::one();
        }
        let mut ans = Mod::zero();
        for &i in &a {
            if x >= i && x % 2 == i % 2 {
                ans += f.call((x - i) / 2);
            }
        }
        if x < res.len() {
            res[x] = Some(ans);
        }
        ans
    });
    out.print_line(rec.call(n));
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
