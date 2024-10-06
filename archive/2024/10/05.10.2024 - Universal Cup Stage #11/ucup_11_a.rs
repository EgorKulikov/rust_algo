//{"name":"ucup_11_a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup_11_a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    type Mod = ModIntF;
    let mut matrix = Matrix::zero(49, 49);
    for i in 0..7 {
        for j in 0..7 {
            matrix[(i * 7 + j, i * 7 + j)] = Mod::new(50);
            if i < 6 {
                matrix[(i * 7 + j, (i + 1) * 7 + j)] = Mod::one();
            } else {
                matrix[(i * 7 + j, i * 7 + j)] += Mod::one();
            }
            if j < 6 {
                matrix[(i * 7 + j, i * 7 + j + 1)] = Mod::one();
            } else {
                matrix[(i * 7 + j, i * 7 + j)] += Mod::one();
            }
        }
    }
    let mut powers = Vec::with_capacity(30);
    let mut cur = matrix.clone();
    for _ in 0..30 {
        powers.push(cur.clone());
        cur = cur.mult(&cur);
    }

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_int();
        let mut res = Matrix::row(&[Mod::zero(); 49]);
        res[(0, 0)] = Mod::one();
        for i in 0..30 {
            if n.is_set(i) {
                res = res.mult(&powers[i]);
            }
        }
        out.print_line(res[(0, 48)]);
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
