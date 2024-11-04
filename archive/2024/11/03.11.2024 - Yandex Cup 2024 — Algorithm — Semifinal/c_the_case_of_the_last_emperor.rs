//{"name":"C. The Case of the Last Emperor","group":"Yandex - Yandex Cup 2024 — Algorithm — Semifinal","url":"https://contest.yandex.com/contest/70295/problems/C/","interactive":false,"timeLimit":4000,"tests":[{"input":"3 3 3\n1 2\n2 3\n1 3\n1 3\n2 4\n3 5\n","output":"8\n16\n32\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTheCaseOfTheLastEmperor"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    type Mod = ModInt7;
    let mut matrix = Matrix::zero(n, n);
    for (u, v) in edges {
        matrix[(u, v)] += Mod::one();
        matrix[(v, u)] += Mod::one();
    }
    let mut pw = Vec::with_capacity(25);
    for _ in 0..25 {
        pw.push(matrix.clone());
        matrix = matrix.mult(&matrix);
    }
    for _ in 0..q {
        let v = input.read_size() - 1;
        let k = input.read_size();

        let mut res = Matrix::zero(1, n);
        res[(0, v)] = Mod::one();
        for i in 0..25 {
            if k.is_set(i) {
                res = res.mult(&pw[i]);
            }
        }
        out.print_line(res.iter().fold(Mod::zero(), |acc, x| acc + *x));
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
