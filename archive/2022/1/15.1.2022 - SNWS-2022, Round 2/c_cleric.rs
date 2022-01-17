//{"name":"C. Cleric","group":"Yandex - SNWS-2022, Round 2","url":"https://contest.yandex.ru/snws2022/contest/23958/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"7 2\n0.5\n2 3\n3 6\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCleric"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let p = input.read_float();
    let edges = input.read_vec::<(usize, usize)>(k).dec_by_one();

    let mut mat = Arr2d::new(n, n, 0.);
    mat[(n - 1, n - 1)] = 1.;
    for i in 0..n - 1 {
        for j in 1..=6 {
            mat[(i, (i + j).min(n - 1))] += 1. / 6.;
        }
    }
    let mut with_special = mat.clone();
    for (s, f) in edges {
        for i in 0..n {
            with_special[(i, s)] -= mat[(i, s)];
            with_special[(i, f)] += mat[(i, s)];
        }
    }
    fn mult(a: &Arr2d<f64>, b: &Arr2d<f64>) -> Arr2d<f64> {
        let n = a.d1();
        Arr2d::generate(n, n, |i, j| {
            let mut res = 0.;
            for k in 0..n {
                res += a[(i, k)] * b[(k, j)];
            }
            res
        })
    }
    let mut mat = vec![with_special];
    let mut ans = 1;
    while mat.last().unwrap()[(0, n - 1)] < p {
        ans *= 2;
        let n_mat = mult(mat.last().unwrap(), mat.last().unwrap());
        mat.push(n_mat);
    }
    if ans > 1 {
        ans /= 2;
        mat.pop();
        let mut cur = mat.pop().unwrap();
        let mut add = ans;
        for c_mat in mat.into_iter().rev() {
            add /= 2;
            let next = mult(&cur, &c_mat);
            if next[(0, n - 1)] < p {
                ans += add;
                cur = next;
            }
        }
        assert_eq!(add, 1);
        ans += 1;
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
