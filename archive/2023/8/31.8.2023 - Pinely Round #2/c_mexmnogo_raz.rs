//{"name":"C. MEX много раз","group":"Codeforces - Pinely Round 2 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1863/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2\n1\n3 1\n0 1 3\n2 2\n0 2\n5 5\n1 2 3 4 5\n10 100\n5 3 0 4 2 1 6 9 10 8\n","output":"1\n2 0 1\n2 1\n2 3 4 5 0\n7 5 3 0 4 2 1 6 9 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMEXMnogoRaz"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut a = input.read_size_vec(n);

    let mut bs = BitSet::new(n + 1);
    for &i in &a {
        bs.set(i);
    }
    for i in 0..=n {
        if !bs[i] {
            a.push(i);
        }
    }
    let ans = a
        .iter()
        .skip(k * n % (n + 1))
        .chain(a.iter().take(k * n % (n + 1)))
        .take(n)
        .copied()
        .collect_vec();
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
