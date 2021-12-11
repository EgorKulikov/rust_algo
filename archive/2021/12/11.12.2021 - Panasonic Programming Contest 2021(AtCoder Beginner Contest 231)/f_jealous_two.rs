//{"name":"F - Jealous Two","group":"AtCoder - Panasonic Programming Contest 2021(AtCoder Beginner Contest 231)","url":"https://atcoder.jp/contests/abc231/tasks/abc231_f","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n50 100 150\n1 3 2\n","output":"4\n"},{"input":"3\n123456789 123456 123\n987 987654 987654321\n","output":"6\n"},{"input":"10\n3 1 4 1 5 9 2 6 5 3\n2 7 1 8 2 8 1 8 2 8\n","output":"37\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FJealousTwo"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{compress, out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let a: Vec<u32> = input.read_vec(n);
    let b: Vec<u32> = input.read_vec(n);

    let (_, (a,)) = compress!(a);
    let (_, (b,)) = compress!(b);
    let mut gifts = a.into_iter().zip(b.into_iter()).collect_vec();
    gifts.sort();
    let mut ft: FenwickTree<u64> = FenwickTree::new(n);
    let mut ans: u64 = 0;
    for (i, (a, b)) in gifts.iter().enumerate() {
        if i == 0 || gifts[i - 1].0 != *a {
            let mut j = i;
            while j < n && gifts[j].0 == *a {
                ft.add(gifts[j].1, 1);
                j += 1;
            }
        }
        ans += ft.get(*b, n);
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
