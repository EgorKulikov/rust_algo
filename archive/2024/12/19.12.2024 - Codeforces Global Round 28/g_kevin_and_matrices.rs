//{"name":"G. Kevin and Matrices","group":"Codeforces - Codeforces Global Round 28","url":"https://codeforces.com/contest/2048/problem/G","interactive":false,"timeLimit":6000,"tests":[{"input":"3\n2 2 2\n2 3 4\n11 45 14\n","output":"14\n2824\n883799966\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GKevinAndMatrices"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let v = input.read_size();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + 1);
    let mut ans = Mod::zero();
    for x in 1..=v {
        for a in 1..=n {
            let pp1 = Mod::from_index(v - x + 1).power(n - a);
            let bpp2 = Mod::from_index(x).power(a * m);
            let pp2 = Mod::from_index(x).power(a).inv().unwrap();
            let bpp3 = Mod::from_index(v).power((n - a) * m);
            let pp3 = Mod::from_index(v).power(n - a).inv().unwrap();
            let mult = bpp2 * bpp3 * c.c(n, a) * if a % 2 == 0 { Mod::one() } else { -Mod::one() };
            let by = -pp1 * pp2 * pp3;
            let cur = (by + Mod::one()).power(m) - Mod::one();
            ans += cur * mult;
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
