//{"name":"K. Random Mex","group":"Universal Cup - The 3rd Universal Cup. Stage 13: Sendai","url":"https://contest.ucup.ac/contest/1812/problem/9486","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 2\n1 1\n20 23\n8000 8000\n","output":"374341634\n1\n111675632\n994279778\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KRandomMex"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value::Value;
use algo_lib::numbers::mod_int::{ModIntF, ValF};
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::number_ext::Power;
use std::collections::HashMap;
use std::mem::transmute;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_size();
    let queries = input.read_size_pair_vec(t);

    type Mod = ModIntF;
    let cc = Combinations::<Mod>::new(8001);
    let mut a = Arr2d::new(8001, 8001, Mod::zero());
    for m in 1..=8000 {
        a[(m, 0)] = Mod::from_index(m);
        let im = Mod::from_index(m).inv().unwrap();
        for q in 1..m {
            a[(m, q)] = Mod::from_index(m - q) * im * (Mod::one() + a[(m - 1, q)]);
        }
    }
    for m in 1..=8000 {
        for q in 0..=m {
            a[(m, q)] *= cc.c(m, q);
        }
    }
    let mut b = Arr2d::new(8001, 8001, Mod::zero());
    b[(0, 0)] = Mod::one();
    for n in 1..=8000 {
        for p in 1..=n {
            b[(n, p)] = Mod::from_index(p) * (b[(n - 1, p - 1)] + b[(n - 1, p)]);
        }
    }
    for n in 0..=8000 {
        b[n].reverse();
    }
    let mut cache = HashMap::<(usize, usize), Mod>::new();
    let a: Arr2d<i32> = unsafe { transmute(a) };
    let b: Arr2d<i32> = unsafe { transmute(b) };

    fn multiply(a: &[i32], b: &[i32]) -> i128 {
        let mut res = 0;
        for i in a.indices() {
            res += ((a[i] as i64) * (b[i] as i64)) as i128;
        }
        res
    }

    for (n, m) in queries {
        if let Some(val) = cache.get(&(n, m)) {
            out.print_line(*val);
            continue;
        }
        let aa = &a[m];
        let bb = &b[n][8000 - m..];
        let res = multiply(&aa[m.max(n) - n..m], &bb[m.max(n) - n..m]);
        let mut res = Mod::new((res % ValF::val() as i128) as i32);
        res /= Mod::from_index(m).power(n);
        out.print_line(res);
        cache.insert((n, m), res);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
