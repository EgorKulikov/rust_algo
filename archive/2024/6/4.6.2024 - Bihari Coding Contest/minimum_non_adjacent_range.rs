//{"name":"Minimum Non-Adjacent Range","group":"HackerRank - Bihari Coding Contest","url":"https://www.hackerrank.com/contests/bihari-coding-contest/challenges/minimum-non-adjacent-range","interactive":false,"timeLimit":4000,"tests":[{"input":"7 4\n0 2 4 5 9 11 3\n","output":"9\n"},{"input":"7 3\n0 2 4 5 9 11 3\n","output":"3\n"},{"input":"7 5\n0 2 4 5 9 11 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinimumNonAdjacentRange"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);

    if 2 * k - 1 > n {
        out.print_line(-1);
        return;
    }
    let mut left = 0;
    let mut right = a.iter().max().unwrap() - a.iter().min().unwrap();
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| a[i]);
    while left < right {
        let mid = (left + right) / 2;
        let mut segs = BTreeSet::new();
        let mut cur = 0;
        let mut to = 0;
        let mut found = false;
        for i in 0..n {
            let j = order[i];
            while to < n && a[order[to]] - a[j] <= mid {
                let k = order[to];
                to += 1;
                let mut start = k;
                let mut end = k;
                if let Some(&(a, b)) = segs.range(..(k, k)).next_back() {
                    if b + 1 == k {
                        segs.remove(&(a, b));
                        cur -= (b - a + 2) / 2;
                        start = a;
                    }
                }
                if let Some(&(a, b)) = segs.range((k, k)..).next() {
                    if a == k + 1 {
                        segs.remove(&(a, b));
                        cur -= (b - a + 2) / 2;
                        end = b;
                    }
                }
                segs.insert((start, end));
                cur += (end - start + 2) / 2;
            }
            if cur >= k {
                found = true;
                break;
            }
            let &(a, b) = segs.range(..(j, n)).next_back().unwrap();
            segs.remove(&(a, b));
            cur -= (b - a + 2) / 2;
            if a < j {
                segs.insert((a, j - 1));
                cur += (j - a + 1) / 2;
            }
            if b > j {
                segs.insert((j + 1, b));
                cur += (b - j + 1) / 2;
            }
        }
        if found {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out.print_line(left);
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
