//{"name":"Subway Surfer","group":"CodeChef - STRV2022","url":"https://www.codechef.com/STRV2022/problems/SUBSURF","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n13 11\n2 6 7 8 21 25 31 39 55 56 57 60 62\n1 4 7 11 14 25 44 47 55 57 100\n","output":"450\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SubwaySurfer"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::recursive_function::RecursiveFunction2;
use algo_lib::out_line;
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut v = Vec::with_capacity(2);
    v.push(input.read_long_vec(n));
    v.push(input.read_long_vec(m));

    let mut pos = vec![HashMap::new(); 2];
    for i in 0..2 {
        for (j, &x) in v[i].iter().enumerate() {
            pos[i].insert(x, j);
        }
    }
    for i in 0..2 {
        let mut max = 0usize;
        for &j in &v[i] {
            if let Some(&t) = pos[1 - i].get(&j) {
                if t < max {
                    loop {}
                }
                max.maxim(t);
            }
        }
    }
    let mut ans = Vec::with_capacity(2);
    ans.push(vec![None; n + 1]);
    ans.push(vec![None; m + 1]);
    let mut f = RecursiveFunction2::new(|f, line: usize, p: usize| -> i64 {
        match ans[line][p] {
            Some(x) => x,
            None => {
                let mut res = 0;
                if p < v[line].len() {
                    res = f.call(line, p + 1) + v[line][p];
                    if let Some(&y) = pos[1 - line].get(&v[line][p]) {
                        res.maxim(f.call(1 - line, y + 1) + v[line][p]);
                    }
                }
                ans[line][p] = Some(res);
                res
            }
        }
    });
    out_line!(f.call(0, 0).max(f.call(1, 0)));
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
