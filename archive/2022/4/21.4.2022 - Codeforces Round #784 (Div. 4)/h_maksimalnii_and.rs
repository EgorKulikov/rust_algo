//{"name":"H. Максимальный AND","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 2\n2 1 1\n7 0\n4 6 6 28 6 6 12\n1 30\n0\n4 4\n3 1 3 1\n","output":"2\n4\n2147483646\n1073741825\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HMaksimalniiAND"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut k = input.read_usize();
    let a = input.read_unsigned_vec(n);

    let mut ans = 0;
    for i in (0..=30).rev() {
        let mut need = 0;
        for &a in &a {
            if !a.is_set(i) {
                need += 1;
            }
        }
        if need <= k {
            ans.set_bit(i);
            k -= need;
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
