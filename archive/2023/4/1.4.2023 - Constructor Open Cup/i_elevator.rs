//{"name":"I. Elevator","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/I","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 5 7\n0 1 2 3\n4 5 9\n0 1 2 3\n3 2 10\n1 1 1\n2 3 3\n0 5\n4 4 10\n3 1 1 1\n","output":"3\n5\n3\n0\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IElevator"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let c = input.read_size();
    let m = input.read_size() / 2;
    let a = input.read_size_vec(n);

    let mut ans = 0;
    for i in 0..=n {
        let mut a = a.clone();
        let mut spent = 0;
        let mut cur = 0;
        loop {
            let mut rem = c;
            for j in (0..i).rev() {
                if rem == c && a[j] > 0 {
                    let full = a[j] / c;
                    spent += full * (j + 1);
                    cur += full * c;
                    a[j] %= c;
                }
                if rem == c && a[j] > 0 {
                    spent += j + 1;
                }
                let cc = rem.min(a[j]);
                rem -= cc;
                cur += cc;
                a[j] -= cc;
            }
            if rem == c {
                break;
            }
        }
        if spent > m {
            break;
        }
        let mut m = m - spent;
        loop {
            let mut rem = c;
            for j in i..n {
                if rem == c && a[j] > 0 {
                    let full = (a[j] / c).min(m / (j + 1));
                    m -= full * (j + 1);
                    cur += full * c;
                    a[j] -= c * full;
                }
                if m < j + 1 {
                    break;
                }
                let cc = rem.min(a[j]);
                rem -= cc;
                cur += cc;
                a[j] -= cc;
                if rem == 0 {
                    m -= j + 1;
                    break;
                }
            }
            if rem == c {
                break;
            }
        }
        ans.maxim(cur);
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
