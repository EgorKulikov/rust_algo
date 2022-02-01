//{"name":"C. Убийство монстра","group":"Codeforces - Educational Codeforces Round 122 (Rated for Div. 2)","url":"https://codeforces.com/contest/1633/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n25 4\n9 20\n1 1 10\n25 4\n12 20\n1 1 10\n100 1\n45 2\n0 4 10\n9 2\n69 2\n4 2 7\n","output":"YES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUbiistvoMonstra"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let hc = input.read_long();
    let dc = input.read_long();
    let hm = input.read_long();
    let dm = input.read_long();
    let k = input.read_long();
    let w = input.read_long();
    let a = input.read_long();

    for i in 0..=k {
        let hc = hc + i * a;
        let dc = dc + (k - i) * w;
        if (hm + dc - 1) / dc <= (hc + dm - 1) / dm {
            out_line!("YES");
            return;
        }
    }
    out_line!("NO");
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
