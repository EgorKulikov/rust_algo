//{"name":"C. Сбалансированные кучки камней","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"https://codeforces.com/contest/1623/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1 2 10 100\n4\n100 100 100 1\n5\n5 1 1 1 8\n6\n1 2 3 4 5 6\n","output":"7\n1\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSbalansirovannieKuchkiKamnei"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let h = input.read_long_vec(n);

    let mut l = 1;
    let mut r = 1000000000;
    let mut hc = vec![0; n];
    while l < r {
        let mid = (l + r + 1) >> 1;
        hc.copy_from_slice(&h);
        let mut good = true;
        for i in (2..n).rev() {
            if hc[i] < mid {
                good = false;
                break;
            }
            let delta = (hc[i] - mid).min(h[i]) / 3;
            hc[i - 2] += 2 * delta;
            hc[i - 1] += delta;
        }
        good &= hc[0] >= mid && hc[1] >= mid;
        if good {
            l = mid;
        } else {
            r = mid - 1;
        }
    }
    out_line!(l);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
