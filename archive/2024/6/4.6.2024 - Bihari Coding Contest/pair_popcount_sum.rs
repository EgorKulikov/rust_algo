//{"name":"Pair Popcount Sum","group":"HackerRank - Bihari Coding Contest","url":"https://www.hackerrank.com/contests/bihari-coding-contest/challenges/pair-popcount-sum","interactive":false,"timeLimit":4000,"tests":[{"input":"10\n12 14 19 23 17 9 3 5 13 29\n","output":"124\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PairPopcountSum"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    fn qty(a: &[i64], from: i64, to: i64) -> usize {
        let mut f = 0;
        let mut t = a.len();
        let mut res = 0;
        loop {
            while f < t && a[f] + a[t - 1] < from {
                f += 1;
            }
            while f < t && a[f] + a[t - 1] >= to {
                t -= 1;
            }
            if f == t {
                break res;
            }
            let first = a.lower_bound(&(from - a[f])).max(f + 1);
            res += t - first;
            f += 1;
        }
    }
    let mut ans = 0;
    let mut b = vec![0; n];
    for i in 0..=30 {
        let ab = i64::all_bits(i + 1);
        for j in 0..n {
            b[j] = a[j] & ab;
        }
        b.sort();
        let cur = qty(&b, 1 << i, 1 << (i + 1)) + qty(&b, 3 * (1 << i), 1 << (i + 2));
        ans += cur;
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
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
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
