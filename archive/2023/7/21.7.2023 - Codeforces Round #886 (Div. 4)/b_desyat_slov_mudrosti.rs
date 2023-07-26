//{"name":"B. Десять слов мудрости","group":"Codeforces - Codeforces Round 886 (Div. 4)","url":"https://codeforces.com/contest/1850/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n7 2\n12 5\n9 3\n9 4\n10 1\n3\n1 2\n3 4\n5 6\n1\n1 43\n","output":"4\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDesyatSlovMudrosti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let ab = input.read_int_pair_vec(n);

    out_line!(
        ab.into_iter()
            .enumerate()
            .filter_map(|(i, (a, b))| if a <= 10 { Some((b, i + 1)) } else { None })
            .max()
            .unwrap()
            .1
    );
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
