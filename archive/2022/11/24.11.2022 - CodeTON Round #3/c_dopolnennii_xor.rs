//{"name":"C. Дополненный XOR","group":"Codeforces - CodeTON Round 3 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1750/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n010\n101\n2\n11\n10\n4\n1000\n0011\n2\n10\n10\n3\n111\n111\n","output":"YES\n1\n2 2\nNO\nNO\nYES\n2\n1 2\n2 2\nYES\n2\n1 1\n2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDopolnenniiXOR"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_vec::<char>(n);
    let b = input.read_vec(n);

    let mut eq = a[0] == b[0];
    for (&a, &b) in a.iter().zip(b.iter()) {
        if eq != (a == b) {
            out_line!(false);
            return;
        }
    }
    out_line!(true);
    let mut ans = Vec::new();
    for i in 0..n {
        if a[i] == '1' {
            ans.push((i + 1, i + 1));
            eq = !eq;
        }
    }
    if !eq {
        ans.push((1, 1));
        ans.push((2, 2));
        ans.push((1, 2));
    }
    out_line!(ans.len());
    output().print_per_line(&ans);
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
