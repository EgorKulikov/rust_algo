//{"name":"B2: Final Product (Chapter 2)","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-1/problems/B2","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n3 1 7\n2 10 15\n2 1000 21\n50 50000 3628800\n","output":"Case #1: 3\nCase #2: 12\nCase #3: 16\nCase #4: 229471373\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"final_product_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"final_product_chapter__output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::primes::factorize::Factorize;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    drop(input);

    type Mod = ModInt7;
    let pd = b.prime_divisors();
    let mut ans = Mod::zero();
    let ways = |x: usize| -> Mod {
        let mut res = Mod::one();
        for i in 1..=x {
            res *= n as usize + i - 1;
            res /= i;
        }
        res
    };
    let mut rec = RecursiveFunction3::new(|rec, step: usize, prod: u64, cur: Mod| {
        if step == pd.len() {
            assert!(prod <= a as u64);
            ans += cur;
            return;
        }
        let (p, cnt) = pd[step];
        let mut new_prod = prod;
        for i in 0..=cnt {
            rec.call(step + 1, new_prod, cur * ways(i) * ways(cnt - i));
            new_prod = new_prod.saturating_mul(p);
            if new_prod > a as u64 {
                break;
            }
        }
    });
    rec.call(0, 1, Mod::one());

    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    output.flush();
    is_exhausted
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let paths = std::fs::read_dir(".").unwrap();
    let mut result = None;
    let mut last_accessed = None;
    let re = regex::Regex::new("final_product_chapter_.*_.*input[.]txt").unwrap();
    for path in paths {
        let path = path.unwrap();
        let cur_accessed = path.metadata().unwrap().accessed().unwrap();
        let path = path.path();
        let cur_name = path.file_name().unwrap().to_str().unwrap();
        if re.is_match(cur_name) {
            if last_accessed.is_none() || cur_accessed > last_accessed.unwrap() {
                result = Some(cur_name.to_string());
                last_accessed = Some(cur_accessed);
            }
        }
    }
    let in_file = std::fs::File::open(result.unwrap()).unwrap();
    let input = algo_lib::io::input::Input::file(in_file);
    let out_file = std::fs::File::create("final_product_chapter__output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
