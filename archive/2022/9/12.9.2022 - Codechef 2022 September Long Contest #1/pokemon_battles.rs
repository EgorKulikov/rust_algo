//{"name":"Pokemon Battles","group":"CodeChef - SEP221A","url":"https://www.codechef.com/SEP221A/problems/PBATTLE","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1 2\n3 4\n2\n1 2\n2 1\n3\n1 2 3\n1 4 2\n5\n2 3 5 4 1\n4 2 1 5 6\n","output":"1\n2\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PokemonBattles"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let mut t = a.into_iter().zip(b.into_iter()).collect_vec();
    t.sort();
    t.reverse();
    let mut ans = 0;
    let mut max = 0;
    for (_, b) in t {
        if b > max {
            ans += 1;
            max = b;
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
