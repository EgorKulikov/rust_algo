//{"name":"C. Шифр","group":"Codeforces - Codeforces Round #806 (Div. 4)","url":"https://codeforces.com/contest/1703/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n9 3 1\n3 DDD\n4 UDUU\n2 DU\n2\n0 9\n9 DDDDDDDDD\n9 UUUUUUUUU\n5\n0 5 9 8 3\n10 UUUUUUUUUU\n3 UUD\n8 UUDUUDDD\n10 UUDUUDUDDU\n4 UUUU\n","output":"2 1 1\n9 0\n0 4 9 6 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CShifr"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_int_vec(n);

    for a in &mut a {
        let _len = input.read_usize();
        let s: Str = input.read();
        for c in s {
            if c == b'U' {
                *a -= 1;
            } else {
                *a += 1;
            }
        }
        *a %= 10;
        if *a < 0 {
            *a += 10;
        }
    }
    out_line!(a);
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
