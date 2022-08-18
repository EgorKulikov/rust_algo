//{"name":"D. Роророробот","group":"Codeforces - Educational Codeforces Round 132 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1709/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"11 10\n9 0 0 10 3 4 8 11 10 8\n6\n1 2 1 3 1\n1 2 1 3 2\n4 3 4 5 2\n5 3 11 5 3\n5 3 11 5 2\n11 9 9 10 1\n","output":"YES\nNO\nNO\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRorororobot"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_usize_vec(m);

    let mut max = vec![0; 4 * m];
    let mut init = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
        if left + 1 == right {
            max[root] = a[left];
            return;
        }
        let mid = (left + right) >> 1;
        f.call(2 * root + 1, left, mid);
        f.call(2 * root + 2, mid, right);
        max[root] = max[2 * root + 1].max(max[2 * root + 2]);
    });
    init.call(0, 0, m);

    let q = input.read_usize();
    for _ in 0..q {
        let xs = input.read_usize();
        let ys = input.read_usize() - 1;
        let xf = input.read_usize();
        let yf = input.read_usize() - 1;
        let k = input.read_usize();

        let x_delta = if xf > xs { xf - xs } else { xs - xf };
        if x_delta % k != 0 {
            out_line!(false);
            continue;
        }
        let y_delta = if yf > ys { yf - ys } else { ys - yf };
        if y_delta % k != 0 {
            out_line!(false);
            continue;
        }
        let h = n - (n - xs) % k;
        let from = ys.min(yf);
        let to = ys.max(yf) + 1;
        let mut query = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            if to <= left || right <= from {
                return 0;
            }
            if from <= left && right <= to {
                return max[root];
            }
            let mid = (left + right) >> 1;
            f.call(2 * root + 1, left, mid)
                .max(f.call(2 * root + 2, mid, right))
        });
        out_line!(query.call(0, 0, m) < h);
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
