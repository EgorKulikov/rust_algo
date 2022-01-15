//{"name":"A. Анализ закупочных стратегий","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/A/","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n9 3\n-2 -1 0 1 3 5 7 9 11\n1 3\n3 4\n3 3\n9 9\n3 9\n3 9\n","output":"0\n5.5\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAnalizZakupochnikhStrategii"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let a = input.read_long_vec(n);

    for _ in 0..q {
        let f1 = input.read_usize() - 1;
        let t1 = input.read_usize();
        let f2 = input.read_usize() - 1;
        let t2 = input.read_usize();
        let total_len = t1 - f1 + t2 - f2;
        let mut ans = 0;
        for at in [(total_len - 1) / 2, total_len / 2] {
            let mut left = 0;
            let mut right = n;
            while left < right {
                let mid = (left + right + 1) >> 1;
                let mut cur = 0;
                if mid > f1 {
                    cur += (mid - f1).min(t1 - f1);
                }
                if mid > f2 {
                    cur += (mid - f2).min(t2 - f2);
                }
                if cur > at {
                    right = mid - 1;
                } else {
                    left = mid;
                }
            }
            ans += a[left];
        }
        out_line!((ans as f64) / 2.);
    }
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
