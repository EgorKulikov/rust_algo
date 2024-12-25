//{"name":"Divisors Array (Hard Version)","group":"CodeChef - START166A","url":"https://www.codechef.com/START166A/problems/LOLBSGNJ6PK8","interactive":false,"timeLimit":3000,"tests":[{"input":"3 3\n1 2 3\n","output":"4 6 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DivisorsArrayHardVersion"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);

    let div_table = divisor_table::<usize>(m.max(a.copy_max()) + 1);
    let mut base = vec![0; div_table.len()];
    for mut i in 1..=m {
        while i != 1 {
            let cur = div_table[i];
            let mut cnt = 0usize;
            while i % cur == 0 {
                i /= cur;
                cnt += 1;
            }
            base[cur] += cnt;
        }
    }
    type Mod = ModInt7;
    let mut base_val = Mod::one();
    for i in base.copy_iter() {
        base_val *= Mod::from_index(i + 1);
    }

    let mut ans = Vec::with_capacity(n);
    for mut i in a {
        let mut cur_ans = base_val;
        while i != 1 {
            let cur = div_table[i];
            let mut cnt = 0;
            while i % cur == 0 {
                i /= cur;
                cnt += 1;
            }
            cur_ans /= Mod::from_index(base[cur] + 1);
            cur_ans *= Mod::from_index(base[cur] + cnt + 1);
        }
        ans.push(cur_ans);
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
