//{"name":"F. We Were Both Children","group":"Codeforces - Codeforces Round 886 (Div. 4)","url":"https://codeforces.com/contest/1850/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n5\n1 2 3 4 5\n3\n2 2 2\n6\n3 1 3 4 9 10\n9\n1 3 2 4 2 3 7 8 5\n1\n10\n8\n7 11 6 8 12 4 4 8\n10\n9 11 9 12 1 7 2 5 8 10\n","output":"3\n3\n3\n5\n0\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FWeWereBothChildren"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut map = DefaultHashMap::<_, i32>::new();
    for a in a {
        map[a] += 1;
    }
    let mut qty = vec![0; n + 1];
    for (k, v) in map {
        for i in (k..=n).step_by(k) {
            qty[i] += v;
        }
    }
    out_line!(qty.into_iter().max());
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
