//{"name":"B. Потерянная перестановка","group":"Codeforces - Codeforces Round  #834 (Div. 3)","url":"https://codeforces.com/contest/1759/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 13\n3 1 4\n1 1\n1\n3 3\n1 4 2\n2 1\n4 3\n5 6\n1 2 3 4 5\n","output":"YES\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPoteryannayaPerestanovka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let m = input.read_usize();
    let s = input.read_usize();
    let b = input.read_usize_vec(m);

    let sum = b.iter().sum::<usize>() + s;
    for i in 1.. {
        let val = i * (i + 1) / 2;
        if val > sum {
            out_line!(false);
            return;
        }
        if val == sum {
            out_line!(b.into_iter().max().unwrap() <= i);
            return;
        }
    }
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
