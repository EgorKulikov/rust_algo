//{"name":"Palindrome In Making","group":"CodeChef - START66A","url":"https://www.codechef.com/START66A/problems-old/MAKEPAL3","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6\n2 6 4 3 4 1\n2\n1 10\n3\n1 10 1\n","output":"2\n9\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PalindromeInMaking"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_long_vec(n);

    for i in 0..n / 2 {
        let j = n - 1 - i;
        let m = a[i].min(a[j]);
        a[i] -= m;
        a[j] -= m;
    }
    if n % 2 == 1 {
        a[n / 2] = 0;
    }
    let mut ans = 0;
    let mut cur = 0;
    for i in a {
        if i > cur {
            ans += i - cur;
        }
        cur = i;
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
