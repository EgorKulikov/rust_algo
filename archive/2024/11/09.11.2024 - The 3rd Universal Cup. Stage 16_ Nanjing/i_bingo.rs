//{"name":"I. Bingo","group":"Universal Cup - The 3rd Universal Cup. Stage 16: Nanjing","url":"https://contest.ucup.ac/contest/1828/problem/9572","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 2\n1 3 2 4\n3 1\n10 10 10\n1 3\n20 10 30\n3 4\n1 1 4 5 1 4 1 9 1 9 8 10\n","output":"56\n60\n60\n855346687\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IBingo"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::prime_fft::PrimeFFT;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n * m).sorted();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n * m + 1);
    let mut v = vec![Mod::zero(); n * m + 1];
    for i in 0..=n {
        for j in 0..=m {
            if i + j == 0 {
                continue;
            }
            v[i * m + j * n - i * j] += if (i + j) % 2 == 0 {
                Mod::new(-1)
            } else {
                Mod::one()
            } * c.c(n, i)
                * c.c(m, j);
        }
    }
    let mut u = vec![Mod::zero(); n * m + 1];
    for i in 0..=n * m {
        v[i] *= c.fact(n * m - i);
        u[i] = c.inv_fact(i);
    }
    let res = PrimeFFT::new().multiply(&v, &u);
    let mut so_far = Mod::zero();
    let mut ans = Mod::zero();
    for i in 1..=n * m {
        if i != n * m && a[i] == a[i - 1] {
            continue;
        }
        let cur = res[i] * c.fact(i);
        ans += (cur - so_far) * Mod::new(a[i - 1]);
        so_far = cur;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
