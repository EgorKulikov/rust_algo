//{"name":"F. Fighter","group":"Yandex - SNWS-2022, Round 2","url":"https://contest.yandex.ru/snws2022/contest/23958/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n+ 111\n- 101\n+ 001\n- 111\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFighter"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let _m = input.read_usize();

    let mut new = 0;
    let mut ans = 0;
    for _ in 0..n {
        let t = input.read_char();
        let mask = u64::from_str_radix(&input.read_string(), 2).unwrap();
        if t == '+' {
            new |= mask;
        } else {
            if (new & mask) != mask {
                ans += 1;
            }
            new &= !mask;
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
    let test_type = TestType::Single;
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
