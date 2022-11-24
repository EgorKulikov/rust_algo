//{"name":"Palindrome by Splitting","group":"CodeChef - START66A","url":"https://www.codechef.com/START66A/problems-old/SPLITPAL","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n3 7 6 4\n5\n1 4 5 4 1\n5\n1 2 3 4 5\n","output":"2\n0\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PalindromeBySplitting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_int_vec(n);

    let mut i = 0;
    let mut j = n - 1;
    let mut ans = 0;
    while i < j {
        if a[i] == a[j] {
            i += 1;
            j -= 1;
        } else if a[i] < a[j] {
            a[j] -= a[i];
            i += 1;
            ans += 1;
        } else {
            a[i] -= a[j];
            j -= 1;
            ans += 1;
        }
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
