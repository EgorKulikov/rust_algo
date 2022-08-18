//{"name":"D. Перемещение фишки","group":"Codeforces - Educational Codeforces Round 133 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1716/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8 1\n","output":"1 1 2 2 3 4 5 6\n"},{"input":"10 2\n","output":"0 1 0 1 1 1 1 2 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPeremeshchenieFishki"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut k = input.read_usize();

    type Mod = ModIntF;
    let mut ans = vec![Mod::zero(); n + 1];
    let mut shift = 0;
    let mut cur = vec![Mod::zero(); n + 1];
    cur[0] = Mod::one();
    loop {
        shift += k;
        if shift > n {
            break;
        }
        for i in k..=(n - shift) {
            let add = cur[i - k];
            cur[i] += add;
        }
        for i in shift..=n {
            ans[i] += cur[i - shift];
        }
        k += 1;
    }
    out_line!(ans.into_iter().skip(1).collect_vec());
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
