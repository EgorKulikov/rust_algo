//{"name":"F - Simple Operations on Sequence","group":"AtCoder - M-SOLUTIONS Programming Contest 2021(AtCoder Beginner Contest 232)","url":"https://atcoder.jp/contests/abc232/tasks/abc232_f","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3 5\n4 2 5 2\n6 4 2 1\n","output":"16\n"},{"input":"5 12345 6789\n1 2 3 4 5\n1 2 3 4 5\n","output":"0\n"},{"input":"18 20719114 5117250357733867\n10511029 36397527 63027379 44706927 47672230 79861204 57882493 42931589 51053644 52300688 43971370 26515475 62139996 41282303 34022578 12523039 6696497 64922712\n14720753 4621362 25269832 91410838 86751784 32741849 6602693 60719353 28911226 88280613 18745325 80675202 34289776 37849132 99280042 73760634 43897718 40659077\n","output":"13104119429316474\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSimpleOperationsOnSequence"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let x: i64 = input.read();
    let y: i64 = input.read();
    let a: Vec<i64> = input.read_vec(n);
    let b: Vec<i64> = input.read_vec(n);

    let mut ans = vec![std::i64::MAX; 1 << n];
    ans[0] = 0;
    for i in 0usize..(1 << n) {
        let mut swaps = 0i64;
        let k = i.count_ones() as usize;
        let cur = ans[i];
        for j in 0..n {
            if i.is_set(j) {
                continue;
            }
            ans[i + (1 << j)].minim(cur + swaps * y + (a[k] - b[j]).abs() * x);
            swaps += 1;
        }
    }
    out_line!(*ans.last().unwrap());
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
