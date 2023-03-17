//{"name":"B. Вакцинация","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 3 5 3\n1 2 3 10 11 18\n6 4 0 0\n3 3 3 3 3 4\n9 10 2 2\n0 1 2 3 4 5 6 7 8\n3 10 3 6\n10 20 30\n5 5 4 4\n0 2 4 6 8\n","output":"2\n3\n2\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVaktsinatsiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_size();
    let d = input.read_size();
    let w = input.read_size();
    let t = input.read_size_vec(n);

    let mut time = t[0];
    let mut id = 0;
    let mut ans = 1;
    for (i, t) in t.into_iter().enumerate() {
        if t > time + d + w || i >= id + k {
            ans += 1;
            time = t;
            id = i;
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
