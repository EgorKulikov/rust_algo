//{"name":"uc9_g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc9_g"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::primes::sieve::primes;

type PreCalc = Vec<i64>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, p: &mut PreCalc) {
    let n = input.read_long();

    let mut nc = n;
    let mut pd = Vec::new();
    for &i in p.iter() {
        if i * i > nc {
            break;
        }
        if nc % i == 0 {
            let mut q = 0;
            while nc % i == 0 {
                nc /= i;
                q += 1;
            }
            pd.push((i, q));
        }
    }
    if nc > 1 {
        pd.push((nc, 1));
    }
    let mut ans = 1i64;
    for &(_, e) in pd.iter() {
        ans *= 2 * e + 1;
    }
    // let mut rec = RecursiveFunction2::new(|f, mut d: i64, step: usize| {
    //     if step == pd.len() {
    //         dd.push(d);
    //     } else {
    //         let (p, e) = pd[step];
    //         for i in 0..=e {
    //             f.call(d, step + 1);
    //             if i < e {
    //                 d *= p;
    //             }
    //         }
    //     }
    // });
    // rec.call(1, 0);
    // dd.sort();
    // let mut set = HashSet::new();
    // for i in dd.indices() {
    //     for j in 0..=i {
    //         if n / dd[i] < dd[j] {
    //             break;
    //         }
    //         if (n / dd[i]) % dd[j] == 0 {
    //             set.insert(Rational::new(dd[j], dd[i]));
    //         }
    //     }
    // }
    out.print_line((ans + 1) / 2);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = primes(100_000);

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
