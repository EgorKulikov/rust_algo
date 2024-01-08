//{"name":"D. 01-дерево","group":"Codeforces - Hello 2024","url":"https://codeforces.com/contest/1919/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n2 1 0 1 1\n5\n1 0 2 1 3\n","output":"YES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D01Derevo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut pos = (0..n).collect::<BTreeSet<_>>();
    let mut p = vec![Vec::new(); n];
    for (i, &a) in a.iter().enumerate() {
        p[a].push(i);
    }
    for i in (1..n).rev() {
        for &j in &p[i] {
            let mut good = false;
            if let Some(&prev) = pos.range(..j).next_back() {
                if a[prev] + 1 == i {
                    good = true;
                }
            }
            if let Some(&next) = pos.range(j + 1..).next() {
                if a[next] + 1 == i || a[next] == i {
                    good = true;
                }
            }
            if !good {
                out.print_line(false);
                return;
            }
            pos.remove(&j);
        }
    }
    out.print_line(p[0].len() == 1);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
