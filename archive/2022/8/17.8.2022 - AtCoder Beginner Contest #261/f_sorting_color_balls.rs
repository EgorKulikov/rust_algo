//{"name":"F - Sorting Color Balls","group":"AtCoder - AtCoder Beginner Contest 261","url":"https://atcoder.jp/contests/abc261/tasks/abc261_f","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n1 5 2 2 1\n3 2 1 2 1\n","output":"6\n"},{"input":"3\n1 1 1\n3 2 1\n","output":"0\n"},{"input":"3\n3 1 2\n1 1 2\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSortingColorBalls"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let c = input.read_usize_vec(n).dec_by_one();
    let x = input.read_usize_vec(n).dec_by_one();

    let mut tree: Vec<(_, DefaultMap<_, usize>)> = vec![(0, DefaultMap::new()); 4 * n];
    let mut ans = 0;
    for i in 0..n {
        let from = x[i] + 1;
        let to = n;
        let mut query = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            if to <= left || right <= from {
                return 0;
            }
            if from <= left && right <= to {
                return tree[root].0 - tree[root].1[c[i]];
            }
            let mid = (left + right) >> 1;
            f.call(2 * root + 1, left, mid) + f.call(2 * root + 2, mid, right)
        });
        ans += query.call(0, 0, n);
        let at = x[i];
        let mut update = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            tree[root].0 += 1;
            tree[root].1[c[i]] += 1;
            if left + 1 == right {
                return;
            }
            let mid = (left + right) >> 1;
            if at < mid {
                f.call(2 * root + 1, left, mid);
            } else {
                f.call(2 * root + 2, mid, right);
            }
        });
        update.call(0, 0, n);
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
