//{"name":"F. Ленивые числа","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"8\n2 2\n4 2\n6 4\n33 2\n532 13\n780011804570805480 3788\n366364720306464627 4702032149561577\n293940402103595405 2\n","output":"2\n2\n1\n3\n1\n3789\n1\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLenivieChisla"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_i128();
    let k = input.read_i128();

    let mut from = 1;
    let mut ans = 0;
    while from <= n {
        let to = (from * k - 1).min(n);
        let mut left = from;
        let mut right = to + 1;
        let mut next_left = from;
        let mut next_right = to + 1;
        while left < right {
            let cur = (left + right) / 2;
            let mut pos = 0;
            let mut copy = cur;
            let mut ten = from;
            while copy > 0 {
                pos += copy - ten + 1;
                copy /= k;
                ten /= k;
            }
            if cur > from {
                let mut f = from * k;
                let mut t = cur;
                while f <= n {
                    t *= k;
                    pos += t.min(n + 1) - f;
                    f *= k;
                }
            }
            if pos < cur {
                if next_left == left && next_right == right {
                    next_left = cur + 1;
                }
                left = cur + 1;
            } else {
                if next_left == left && next_right == right {
                    if pos == cur {
                        next_left = cur + 1;
                    } else {
                        next_right = cur;
                    }
                }
                right = cur;
            }
        }
        let start = left;

        let mut left = next_left;
        let mut right = next_right;
        while left < right {
            let cur = (left + right) / 2;
            let mut pos = 0;
            let mut copy = cur;
            let mut ten = from;
            while copy > 0 {
                pos += copy - ten + 1;
                copy /= k;
                ten /= k;
            }
            if cur > from {
                let mut f = from * k;
                let mut t = cur;
                while f <= n {
                    t *= k;
                    pos += t.min(n + 1) - f;
                    f *= k;
                }
            }
            if pos <= cur {
                left = cur + 1;
            } else {
                right = cur;
            }
        }
        ans += right - start;
        from = to + 1;
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
