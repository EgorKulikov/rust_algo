//{"name":"C. Dolce Vita","group":"Codeforces - Educational Codeforces Round 127 (Rated for Div. 2)","url":"https://codeforces.com/contest/1671/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 7\n2 1 2\n5 9\n10 20 30 40 50\n1 1\n1\n2 1000\n1 1\n","output":"11\n0\n1\n1500\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDolceVita"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_long();
    let mut a = input.read_long_vec(n);

    a.sort_unstable();
    let s = a.as_slice().partial_sums();
    let mut ans = 0;
    let mut day = 0;
    for (i, s) in s.into_iter().enumerate().skip(1).rev() {
        let i = i.into_i64();
        if x < s {
            continue;
        }
        let cur = (x - s) / i + 1;
        ans += (cur - day) * i;
        day = cur;
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
