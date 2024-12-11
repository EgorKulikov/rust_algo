//{"name":"I. Items","group":"Universal Cup - The 3rd Universal Cup. Stage 20: Kunming","url":"https://contest.ucup.ac/contest/1871/problem/9870","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5 25\n0 0 0 0 5\n5 11\n4 4 4 5 5\n5 0\n1 2 3 4 5\n5 25\n0 1 2 3 4\n","output":"Yes\nNo\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IItems"}}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::prime_fft::PrimeFFT;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let w = input.read_size_vec(n).sorted().do_with(|w| w.dedup());

    type Mod = ModIntF;
    let mut v = vec![Mod::zero(); 4 * n + 1];
    for i in w {
        v[i + n * 2] = Mod::one();
    }
    let mut fft = PrimeFFT::new();
    let d = m / n;
    let mut op = |a: &[Mod], b: &[Mod]| {
        let res = fft.multiply(a, b);
        let mut res = res[d + 2 * n..].to_vec();
        res.truncate(4 * n + 1);
        for i in 0..=4 * n {
            if res[i] != Mod::zero() {
                res[i] = Mod::one();
            }
        }
        res
    };

    let mut powers = Vec::new();
    powers.push(v.clone());
    let mut id = 1;
    while id * 2 <= n {
        powers.push(op(&powers[Back(0)], &powers[Back(0)]));
        id *= 2;
    }
    let mut res = vec![Mod::zero(); 4 * n + 1];
    res[2 * n] = Mod::one();
    for i in powers.indices() {
        if n.is_set(i) {
            res = op(&res, &powers[i]);
        }
    }
    out.print_line(res[2 * n + m % n] != Mod::zero());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
