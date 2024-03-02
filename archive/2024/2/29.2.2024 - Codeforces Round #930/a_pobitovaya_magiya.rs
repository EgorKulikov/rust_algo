//{"name":"A. Побитовая магия","group":"Codeforces - Codeforces Round 930 (Div. 1)","url":"https://codeforces.com/contest/1936/problem/A","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n4\n\n<\n\n=\n\n>\n\n2\n","output":"\n\n? 0 2 3 1\n\n? 1 1 2 3\n\n? 1 2 0 3\n\n! 3 2\n\n! 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APobitovayaMagiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut max = 0;
    for i in 1..n {
        out.print_line(('?', max, max, i, i));
        out.flush();
        let res = input.read_char();
        if res == '<' {
            max = i;
        }
    }
    let mut candidates = vec![0];
    for i in 1..n {
        out.print_line(('?', max, candidates[0], max, i));
        out.flush();
        let res = input.read_char();
        match res {
            '<' => {
                candidates.clear();
                candidates.push(i);
            }
            '=' => {
                candidates.push(i);
            }
            '>' => {}
            _ => unreachable!(),
        }
    }
    let mut other = candidates[0];
    for i in candidates.into_iter().skip(1) {
        out.print_line(('?', other, other, i, i));
        out.flush();
        let res = input.read_char();
        if res == '>' {
            other = i;
        }
    }
    out.print_line(('!', max, other));
    out.flush();
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
