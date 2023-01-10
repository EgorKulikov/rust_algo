//{"name":"C. Бинарные строки это весело","group":"Codeforces - Codeforces Round #838 (Div. 2)","url":"https://codeforces.com/contest/1762/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n1\n1\n0\n2\n11\n3\n010\n9\n101101111\n37\n1011011111011010000011011111111011111\n","output":"1\n1\n3\n3\n21\n365\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBinarnieStrokiEtoVeselo"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let s = input.read_vec::<char>(n);

    type Mod = ModIntF;
    let mut ans = Mod::one();
    let mut add = Mod::one();
    for (i, j) in s.consecutive_iter() {
        if i != j {
            add = Mod::one();
        } else {
            add *= Mod::new(2);
        }
        ans += add;
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
