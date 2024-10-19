//{"name":"E. MEXimize the Score","group":"Codeforces - Codeforces Round 979 (Div. 2)","url":"https://codeforces.com/contest/2030/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 0 1\n4\n0 0 1 1\n5\n0 0 1 2 2\n4\n1 1 1 1\n","output":"11\n26\n53\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMEXimizeTheScore"}}}

use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let q = a.qty_bound(n);
    type Mod = ModIntF;
    let mut total = Mod::one();
    let c = Combinations::<Mod>::new(n + 1);
    let mut last = vec![Mod::zero(); n + 1];
    let mut ans = Mod::zero();
    let mut other = Mod::new(2).power(n);
    for i in 0..n {
        let mut cur = Vec::new();
        let mut rem_cur = Mod::new(2).power(q[i]);
        other /= rem_cur;
        for j in 0..last.len().min(q[i] + 1) {
            let mut x = total * c.c(q[i], j);
            rem_cur -= c.c(q[i], j);
            x += last[j] * rem_cur;
            total -= last[j];
            ans += Mod::from_index(j) * x * other;
            cur.push(x);
        }
        total = cur.iter().fold(Mod::zero(), |acc, x| acc + *x);
        last = cur;
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
