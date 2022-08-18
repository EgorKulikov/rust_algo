//{"name":"E. Обмен и максимальный отрезок","group":"Codeforces - Educational Codeforces Round 133 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1716/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n-3 5 -3 2 8 -20 6 -1\n3\n1\n0\n1\n","output":"18\n8\n13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EObmenIMaksimalniiOtrezok"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long_vec(1 << n);

    #[derive(Copy, Clone)]
    struct Node {
        sum: i64,
        prefix: i64,
        suffix: i64,
        all: i64,
    }
    let mut rec = RecursiveFunction2::new(|f, from: usize, to: usize| -> Vec<Node> {
        if from + 1 == to {
            return vec![Node {
                sum: a[from].max(0),
                prefix: a[from].max(0),
                suffix: a[from].max(0),
                all: a[from],
            }];
        }

        let mid = (from + to) >> 1;
        let left = f.call(from, mid);
        let right = f.call(mid, to);

        let mut res = Vec::with_capacity(to - from);
        for (&l, &r) in left.iter().zip(right.iter()) {
            res.push(Node {
                sum: l.sum.max(r.sum).max(l.suffix + r.prefix),
                prefix: l.prefix.max(l.all + r.prefix),
                suffix: r.suffix.max(r.all + l.suffix),
                all: l.all + r.all,
            });
        }
        for (&r, &l) in left.iter().zip(right.iter()) {
            res.push(Node {
                sum: l.sum.max(r.sum).max(l.suffix + r.prefix),
                prefix: l.prefix.max(l.all + r.prefix),
                suffix: r.suffix.max(r.all + l.suffix),
                all: l.all + r.all,
            });
        }
        res
    });
    let res = rec.call(0, 1 << n);
    let mut state = 0;
    let q = input.read_usize();
    for _ in 0..q {
        let k = input.read_usize();
        state.flip_bit(k);
        out_line!(res[state].sum);
    }
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
