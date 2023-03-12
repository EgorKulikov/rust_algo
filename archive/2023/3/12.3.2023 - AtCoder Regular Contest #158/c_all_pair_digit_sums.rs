//{"name":"C - All Pair Digit Sums","group":"AtCoder - AtCoder Regular Contest 158","url":"https://atcoder.jp/contests/arc158/tasks/arc158_c","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n53 28\n","output":"36\n"},{"input":"1\n999999999999999\n","output":"135\n"},{"input":"5\n123 456 789 101 112\n","output":"321\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAllPairDigitSums"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::powers;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut ans = 0;
    let mut table = Arr2d::new(15, 15, Vec::new());
    let t = powers(10i64, 16);
    for i in a {
        let mut x = i;
        let mut sd = 0;
        while x > 0 {
            sd += x % 10;
            x /= 10;
        }
        ans += 2 * sd * n.into_i64();
        for j in 0..15 {
            for k in j..15 {
                let cur = i % t[k + 1] / t[j];
                table[(j, k)].push(cur);
            }
        }
    }
    for j in 0..15 {
        for k in j..15 {
            table[(j, k)].sort();
            let l1 = t[k + 1 - j];
            let c = &table[(j, k)];
            let mut front = c.len();
            let mut back = c.len();
            for &i in c {
                let d = i % 10;
                let l2 = d + l1;
                while front > 0 && c[front - 1] + i >= l1 {
                    front -= 1;
                }
                while back > 0 && c[back - 1] + i >= l2 {
                    back -= 1;
                }
                ans -= (back - front).into_i64() * 9;
            }
        }
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
