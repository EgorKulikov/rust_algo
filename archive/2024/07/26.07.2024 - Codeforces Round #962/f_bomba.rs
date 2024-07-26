//{"name":"F. Бомба","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4\n5 6 7\n2 3 4\n5 9\n32 52 68 64 14\n18 14 53 24 8\n5 1000\n1 2 3 4 5\n5 4 3 2 1\n1 1000000\n1000000\n1\n10 6\n3 3 5 10 6 8 6 8 7 7\n6 1 7 4 1 1 8 9 3 1\n","output":"21\n349\n27\n500000500000\n47\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBomba"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let for_lower = |x: usize| -> (usize, usize) {
        let mut qty = 0;
        let mut sum = 0;
        let mut xs = 0;
        for i in 0..n {
            if a[i] < x {
                continue;
            }
            let cur_qty = (a[i] - x + b[i] - 1) / b[i];
            qty += cur_qty;
            if a[i] % b[i] == x % b[i] {
                xs += 1;
            }
            if cur_qty > 0 {
                sum += (2 * a[i] - (cur_qty - 1) * b[i]) * cur_qty / 2;
            }
        }
        if qty > k || qty + xs < k {
            (qty, sum)
        } else {
            (qty, sum + (k - qty) * x)
        }
    };
    let (qty, sum) = for_lower(0);
    if qty <= k {
        out.print_line(sum);
        return;
    }
    let mut left = 0;
    let mut right = 1_000_000_000;
    while left < right {
        let mid = (left + right) / 2;
        let (qty, _) = for_lower(mid);
        if qty > k {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    out.print_line(for_lower(left).1);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
