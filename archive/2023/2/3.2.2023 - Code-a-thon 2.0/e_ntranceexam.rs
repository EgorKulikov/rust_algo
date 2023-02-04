//{"name":"ENTRANCE EXAM","group":"CodeChef - CDAT2023","url":"https://www.codechef.com/CDAT2023/problems/ENT_EXAM","interactive":false,"timeLimit":3000,"tests":[{"input":"3\nzwaxrpd 3\naba 1\naabb 3\n","output":"5\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENTRANCEEXAM"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();
    let k = input.read_size();

    let mut qty = vec![0; 256];
    let mut more = 0;
    let mut ans = 0;
    for (i, c) in s.iter().enumerate() {
        let c = c.into_usize();
        if qty[c] == 1 {
            more += 1;
        }
        qty[c] += 1;
        if i >= k {
            let c = s[i - k].into_usize();
            qty[c] -= 1;
            if qty[c] == 1 {
                more -= 1;
            }
        }
        if i >= k - 1 && more == 0 {
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
