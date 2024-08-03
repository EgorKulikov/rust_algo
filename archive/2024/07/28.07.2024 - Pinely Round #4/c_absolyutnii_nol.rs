//{"name":"C. Абсолютный ноль","group":"Codeforces - Pinely Round 4 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1991/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1\n5\n2\n0 0\n3\n4 6 8\n4\n80 40 20 10\n5\n1 2 3 4 5\n","output":"1\n5\n0\n\n3\n6 1 1\n7\n60 40 20 10 30 25 5\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAbsolyutniiNol"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_int_vec(n);

    for i in 1..n {
        if a[i] % 2 != a[0] % 2 {
            out.print_line(-1);
            return;
        }
    }
    let mut ans = Vec::new();
    loop {
        let max = *a.iter().max().unwrap();
        if max == 0 {
            break;
        }
        let cur = (max + 1) / 2;
        for i in a.iter_mut() {
            *i = (*i - cur).abs();
        }
        ans.push(cur);
    }
    out.print_line(ans.len());
    out.print_line(ans);
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
}
//END MAIN
