//{"name":"D. Запросы НОД","group":"Codeforces - Codeforces Round #838 (Div. 2)","url":"https://codeforces.com/contest/1762/problem/D","interactive":true,"timeLimit":3000,"tests":[{"input":"2\n2\n\n1\n\n1\n5\n\n2\n\n4\n\n1\n","output":"\n? 1 2\n\n! 1 2\n\n\n? 1 2\n\n? 2 3\n\n! 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DZaprosiNOD"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();

    let mut left = 0;
    let mut right = 1;
    let mut query = |i: usize, j: usize| -> usize {
        out_line!('?', i + 1, j + 1);
        output().flush();
        input.read()
    };
    let mut g = query(left, right);
    for i in 2..n {
        let g1 = query(left, i);
        let g2 = query(right, i);

        if g1 % g != 0 || g2 % g != 0 {
            continue;
        }
        if g1 > g2 {
            right = i;
            g = g1;
        } else {
            left = i;
            g = g2;
        }
    }
    out_line!('!', left + 1, right + 1);
    assert_eq!(input.read_size(), 1);
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
