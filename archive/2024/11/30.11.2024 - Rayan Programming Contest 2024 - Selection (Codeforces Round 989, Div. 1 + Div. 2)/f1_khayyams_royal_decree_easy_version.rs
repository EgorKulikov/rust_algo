//{"name":"F1. Khayyam's Royal Decree (Easy Version)","group":"Codeforces - Rayan Programming Contest 2024 - Selection (Codeforces Round 989, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2034/problem/F1","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4 0\n1 1 1\n1 0\n3 3 2\n1 1\n2 2\n3 3 2\n2 1\n1 2\n10 4 5\n1 0\n8 0\n6 4\n0 2\n7 4\n","output":"10\n499122180\n798595498\n149736666\n414854846\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1KhayyamsRoyalDecreeEasyVersion"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let mut special = input.read_size_pair_vec(k);

    type Mod = ModIntF;
    let mult = if special.contains(&(0, 0)) {
        Mod::new(2)
    } else {
        Mod::new(1)
    };
    special.push((0, 0));
    special.push((n, m));
    special.sort();
    special.dedup();
    let c = Combinations::<Mod>::new(n + m + 1);
    let mut q = vec![Mod::zero(); special.len()];
    let mut sum = vec![Mod::zero(); special.len()];
    let mut direct = Arr2d::new(special.len(), special.len(), Mod::zero());
    q[Back(0)] = Mod::one();
    for i in special.indices().rev() {
        let (r, b) = special[i];
        for j in i + 1..special.len() {
            let (r1, b1) = special[j];
            if r1 < r || b1 < b {
                continue;
            }
            direct[(i, j)] = c.c(r1 - r + b1 - b, r1 - r);
            for k in i + 1..j {
                let (r2, b2) = special[k];
                if r <= r2 && r2 <= r1 && b <= b2 && b2 <= b1 {
                    let sub = direct[(i, k)] * c.c(r1 - r2 + b1 - b2, r1 - r2);
                    direct[(i, j)] -= sub;
                }
            }
            let add = q[j] * direct[(i, j)];
            q[i] += add;
            let add = sum[j] * Mod::new(2) * direct[(i, j)]
                + Mod::from_index((r1 - r) * 2 + b1 - b) * q[j] * direct[(i, j)];
            sum[i] += add;
        }
    }
    out.print_line(sum[0] * mult / q[0]);
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
