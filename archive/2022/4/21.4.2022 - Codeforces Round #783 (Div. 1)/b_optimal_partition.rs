//{"name":"B. Optimal Partition","group":"Codeforces - Codeforces Round #783 (Div. 1)","url":"https://codeforces.com/contest/1667/problem/B","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n3\n1 2 -3\n4\n0 -2 3 -4\n5\n-1 -2 3 -1 -1\n6\n-1 2 -3 4 -5 6\n7\n1 -1 -1 1 -1 -1 1\n","output":"1\n2\n1\n6\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOptimalPartition"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::{compress, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long_vec(n);

    let s = a.as_slice().partial_sums();
    if s[n] > 0 {
        out_line!(n);
        return;
    }
    let (v, (s,)) = compress!(s);
    let len = v.len();

    const INF: isize = isize::MIN / 2;

    let mut val = vec![INF; 4 * len];

    let mut ans = 0isize;
    for i in 0..n {
        let mut update = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            val[root].maxim(ans - i.into_isize());
            if left + 1 == right {
                return;
            }
            let mid = (left + right) >> 1;
            if s[i] < mid {
                f.call(2 * root + 1, left, mid);
            } else {
                f.call(2 * root + 2, mid, right);
            }
        });
        update.call(0, 0, len);
        if a[i] < 0 {
            ans -= 1;
        } else if a[i] > 0 {
            ans += 1;
        }
        let mut query =
            RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| -> isize {
                if left >= s[i + 1] {
                    return INF;
                }
                if right <= s[i + 1] {
                    return val[root];
                }
                let mid = (left + right) >> 1;
                f.call(2 * root + 1, left, mid)
                    .max(f.call(2 * root + 2, mid, right))
            });
        ans.maxim(query.call(0, 0, len) + (i + 1).into_isize());
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
    let test_type = TestType::MultiNumber;
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
