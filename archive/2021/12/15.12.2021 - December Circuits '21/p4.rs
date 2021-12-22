//{"name":"Task","group":"HackerEarth - December Circuits '21","url":"https://www.hackerearth.com/challenges/competitive/december-circuits-21-2/algorithm/beautiful-numbers-4-20d7065b/","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n1 100\n","output":"9\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::number_ext::NumDigs;
use algo_lib::numbers::number_iterator::iterate;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize, sums: &Vec<Vec<u32>>) {
    let l: i32 = input.read();
    let r: i32 = input.read();

    let ans = solve_impl(sums, l, r);
    out_line!(ans);
}

fn solve_impl(sums: &Vec<Vec<u32>>, l: i32, r: i32) -> u32 {
    let mut ans = 0u32;
    for (mut prefix, rem, _) in iterate(l, r) {
        let total_digs = rem + prefix.num_digs();
        if total_digs % 2 != 0 {
            continue;
        }
        let mut left_digs = (total_digs - rem).min(total_digs / 2);
        let mut right_digs = if rem < total_digs / 2 {
            total_digs / 2 - rem
        } else {
            0
        };
        let mut left_sum = 0;
        let mut right_sum = 0;
        for _ in 0..right_digs {
            right_sum += prefix % 10;
            prefix /= 10;
        }
        for _ in 0..left_digs {
            left_sum += prefix % 10;
            prefix /= 10;
        }
        left_digs = total_digs / 2 - left_digs;
        right_digs = total_digs / 2 - right_digs;
        let delta = left_sum - right_sum;
        if delta >= 0 {
            let delta = delta as usize;
            for i in delta..sums[right_digs].len().min(sums[left_digs].len() + delta) {
                ans += sums[left_digs][i - delta] * sums[right_digs][i];
            }
        } else {
            let delta = (-delta) as usize;
            for i in delta..(sums[right_digs].len() + delta).min(sums[left_digs].len()) {
                ans += sums[left_digs][i] * sums[right_digs][i - delta];
            }
        }
    }
    ans
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut sums = vec![vec![1]];
    while sums.len() <= 6 {
        let last = sums.last().unwrap();
        let mut next = vec![0u32; last.len() + 9];
        for (i, val) in last.iter().enumerate() {
            for j in 0..10 {
                next[i + j] += *val;
            }
        }
        sums.push(next);
    }

    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1, &sums);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

#[test]
fn stress() {
    let mut sums = vec![vec![1]];
    while sums.len() <= 6 {
        let last = sums.last().unwrap();
        let mut next = vec![0u32; last.len() + 9];
        for (i, val) in last.iter().enumerate() {
            for j in 0..10 {
                next[i + j] += *val;
            }
        }
        sums.push(next);
    }

    let mut ans = 0u32;
    for i in 1..100000 {
        if i.num_digs() % 2 == 0 {
            let half = i.num_digs() / 2;
            let mut ls = 0;
            let mut j = i;
            for _ in 0..half {
                ls += j % 10;
                j /= 10;
            }
            let mut rs = 0;
            for _ in 0..half {
                rs += j % 10;
                j /= 10;
            }
            if ls == rs {
                ans += 1;
            }
        }
        let actual = solve_impl(&sums, 1, i);
        assert_eq!(actual, ans);
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
