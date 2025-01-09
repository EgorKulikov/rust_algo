//{"name":"Chef Loves Maths","group":"CodeChef - START168","url":"https://www.codechef.com/problems/FERM_SQUARE?tab=solution","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3\n1 1 1\n3\n1 2 3\n3\n1 2 2\n","output":"6\n14\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ChefLovesMaths"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_size();

    let dt = divisor_table(10_000_001);
    let mut r = Random::new();
    let val = Vec::gen(10_000_001, |_, _| r.gen::<u64>());
    type Mod = ModInt7;

    for _ in 0..t {
        let n = input.read_size();
        let a = input.read_size_vec(n);

        let mut ans = Mod::from_index(n) * Mod::from_index(n + 1) / Mod::new(2) * Mod::new(3);
        let mut all = 0;
        let mut all_qty = DefaultHashMap::new(0);
        all_qty[0] = 1;
        let mut bad = 0;
        let mut bad_qty = DefaultHashMap::new(0);
        bad_qty[0] = 1;
        let mut two = 0;
        let mut eight = 1;
        let mut te = Arr2d::new(2, 8, 0);
        te[(0, 1)] = 1;

        for mut i in a {
            let mut ic = i;
            while ic % 2 == 0 {
                two ^= 1;
                ic /= 2;
            }
            eight = eight * ic % 8;
            ans += Mod::from_index(te[(two, 8 - eight)]);
            te[(two, eight)] += 1;
            while i != 1 {
                let d = dt[i];
                all ^= val[d];
                if d % 4 == 3 {
                    bad ^= val[d];
                }
                i /= d;
            }
            ans -= Mod::from_index(all_qty[all]);
            ans -= Mod::from_index(bad_qty[bad]);
            all_qty[all] += 1;
            bad_qty[bad] += 1;
        }
        out.print_line(ans);
    }
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
