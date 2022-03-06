//{"name":"C. Тайлер и строки","group":"Codeforces - Codeforces Round #775 (Div. 1, по задачам Открытой олимпиады школьников по программированию)","url":"https://codeforces.com/contest/1648/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n1 2 2\n2 1 2 1\n","output":"2\n"},{"input":"4 4\n1 2 3 4\n4 3 2 1\n","output":"23\n"},{"input":"4 3\n1 1 1 2\n1 1 2\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTailerIStroki"}}}

use algo_lib::collections::vec_ext::{IncDec, Qty};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{
    Callable3, Callable5, RecursiveFunction3, RecursiveFunction5,
};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::inverse_factorials;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::factorials;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let s = input.read_usize_vec(n).dec_by_one();
    let t = input.read_usize_vec(m).dec_by_one();

    type Mod = ModIntF;
    let mut q = s.qty();
    let b = q.len();
    let mut one_more = vec![Mod::zero(); 4 * b];
    let mut current = vec![Mod::zero(); 4 * b];
    let inv_f = inverse_factorials::<Mod>(n + 1);
    let f = factorials::<Mod>(n);
    let mut init = RecursiveFunction3::new(|f, root, from, to| {
        if from + 1 == to {
            current[root] = inv_f[q[from]];
            if q[from] == 0 {
                one_more[root] = Mod::zero();
            } else {
                one_more[root] = inv_f[q[from] - 1];
            }
            return;
        }
        let mid = (from + to) >> 1;
        f.call(2 * root + 1, from, mid);
        f.call(2 * root + 2, mid, to);
        current[root] = current[2 * root + 1] * current[2 * root + 2];
        one_more[root] = current[2 * root + 1] * one_more[2 * root + 2]
            + current[2 * root + 2] * one_more[2 * root + 1];
    });
    init.call(0, 0, b);
    let mut ans = Mod::zero();
    let mut len = n;
    for i in t {
        if len == 0 {
            ans += Mod::one();
            break;
        }
        len -= 1;
        let mut query = RecursiveFunction5::new(|f, root, from, to, left, right| {
            if to <= left || right <= from {
                (Mod::zero(), Mod::one())
            } else if from >= left && to <= right {
                (one_more[root], current[root])
            } else {
                let mid = (from + to) >> 1;
                let (oml, cl) = f.call(2 * root + 1, from, mid, left, right);
                let (omr, cr) = f.call(2 * root + 2, mid, to, left, right);
                (oml * cr + omr * cl, cl * cr)
            }
        });
        ans += query.call(0, 0, b, 0, i).0 * query.call(0, 0, b, i, b).1 * f[len];
        if q[i] == 0 {
            out_line!(ans);
            return;
        }
        let mut update = RecursiveFunction3::new(|f, root, from, to| {
            if from + 1 == to {
                q[from] -= 1;
                current[root] = inv_f[q[from]];
                if q[from] == 0 {
                    one_more[root] = Mod::zero();
                } else {
                    one_more[root] = inv_f[q[from] - 1];
                }
                return;
            }
            let mid = (from + to) >> 1;
            if i < mid {
                f.call(2 * root + 1, from, mid);
            } else {
                f.call(2 * root + 2, mid, to);
            }
            current[root] = current[2 * root + 1] * current[2 * root + 2];
            one_more[root] = current[2 * root + 1] * one_more[2 * root + 2]
                + current[2 * root + 2] * one_more[2 * root + 1];
        });
        update.call(0, 0, b);
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
