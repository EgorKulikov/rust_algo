//{"name":"C1. Хорошие подмассивы (простая версия)","group":"Codeforces - Codeforces Round #825 (Div. 2)","url":"https://codeforces.com/contest/1736/problem/C1","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n1 2 3\n3\n1 1 1\n4\n2 1 4 3\n","output":"6\n3\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C1KhoroshiePodmassiviProstayaVersiya"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n).dec_by_one();

    let mut limits = VecDeque::new();
    let mut ans = 0i64;
    let mut end = n.into_i32();
    for (i, a) in a.into_iter().enumerate().rev() {
        let i = i.into_i32();
        while let Some(&(j, b)) = limits.back() {
            if a - i > b - j {
                break;
            }
            limits.pop_back();
        }
        limits.push_back((i, a));
        if let Some(&(j, b)) = limits.front() {
            if b - (j - i) < 0 {
                end = j;
                limits.pop_front();
            }
        }
        ans += (end - i).into_i64();
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
