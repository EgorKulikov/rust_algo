//{"name":"problem_g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = BitSet;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut ans = 0;
    let mut vals = DefaultHashMap::new(0i64);
    for i in a {
        if !data[i] {
            vals.clear();
            continue;
        }
        let mut n_vals = DefaultHashMap::new(0i64);
        ans += i as i64;
        n_vals[i as i64] += 1;
        for (k, v) in vals {
            let g = gcd(k, i as i64);
            ans += g * v;
            n_vals[g] += v;
        }
        vals = n_vals;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = BitSet::new(10_000_001);
    let d = divisor_table(10_000_001);
    for i in 1..=10_000_000 {
        let mut good = true;
        let mut x = i;
        while x != 1 {
            let d = d[x];
            let mut q = 0;
            while x % d == 0 {
                x /= d;
                q += 1;
            }
            if q == 1 {
                good = false;
                break;
            }
        }
        pre_calc.change(i, good);
    }

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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
