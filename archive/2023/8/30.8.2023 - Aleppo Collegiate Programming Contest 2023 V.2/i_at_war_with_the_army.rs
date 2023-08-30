//{"name":"I. At War With The Army","group":"Codeforces - Aleppo Collegiate Programming Contest 2023 V.2","url":"https://codeforces.com/gym/104544/problem/I","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n3 3\n1 0\n2 1\n1 2\n4 4\n1 3\n2 2\n3 1\n4 2\n","output":"4\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IAtWarWithTheArmy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mines = input.read_size_pair_vec(m);

    let mut bad = vec![Vec::new(); n];
    for (i, j) in mines {
        if j < i {
            continue;
        }
        bad[i - 1].push(j - i);
    }
    for b in &mut bad {
        b.sort();
    }
    let mut first = None;
    for i in 0..n {
        if !bad[i].is_empty() && bad[i][0] == 0 {
            first = Some(i);
            break;
        }
    }
    if let Some(p) = first {
        let mut b = HashSet::new();
        for i in p + 1..n {
            for &j in &bad[i] {
                b.insert(j);
            }
        }
        let mut f = 0;
        for i in (0..=p).rev() {
            if bad[i].is_empty() || bad[i][0] > f {
                out_line!(n + 1 + f);
                return;
            }
            for &j in &bad[i] {
                b.insert(j);
            }
            while b.contains(&f) {
                f += 1;
            }
        }
        out_line!(n + 1 + f);
    } else {
        out_line!(n + 1);
    }
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
