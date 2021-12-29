//{"name":"C. Репрезентативные края","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"https://codeforces.com/contest/1616/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4\n1 2 3 4\n4\n1 1 2 2\n2\n0 -1\n6\n3 -2 4 -1 -4 0\n1\n-100\n","output":"0\n2\n0\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CReprezentativnieKraya"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    if n == 1 {
        out_line!(0);
        return;
    }
    let mut ans = n - 2;
    for i in 0..n {
        for j in i + 1..n {
            let base = Rational::new(a[i], 1);
            let step = Rational::new(a[j] - a[i], (j - i).into_i32());
            let mut cur = 2;
            for k in j + 1..n {
                if Rational::new(a[k], 1) == base + step * Rational::new((k - i).into_i32(), 1) {
                    cur += 1;
                }
            }
            ans.minim(n - cur);
        }
    }
    out_line!(ans);
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
