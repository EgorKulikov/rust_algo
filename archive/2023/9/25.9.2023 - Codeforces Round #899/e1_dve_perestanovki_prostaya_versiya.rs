//{"name":"E1. Две перестановки (простая версия)","group":"Codeforces - Codeforces Round 899 (Div. 2)","url":"https://codeforces.com/contest/1882/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n2 1 3\n5 2 1 4 3\n","output":"2\n3 4\n2 4\n"},{"input":"4 4\n3 4 2 1\n2 4 1 3\n","output":"5\n4 2\n3 3\n1 4\n3 2\n4 1\n"},{"input":"2 2\n1 2\n2 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1DvePerestanovkiProstayaVersiya"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = input.read_size_vec(n).dec();
    let mut b = input.read_size_vec(m).dec();

    fn solve(a: &mut [usize]) -> Vec<usize> {
        fn operation(a: &mut [usize], pos: usize) {
            a.rotate_left(pos);
            let to = a.len() - pos;
            a[..to].rotate_left(1);
        }
        let mut res = Vec::new();
        for i in a.indices().skip(1).rev() {
            if a[0] != i {
                let p = a.iter().position(|&x| x == i).unwrap();
                res.push(p);
                operation(a, p - 1);
            }
            let p = a.iter().position(|&x| x == i - 1).unwrap();
            res.push(p + 1);
            operation(a, p);
        }
        res
    }

    let mut left = solve(&mut a);
    let mut right = solve(&mut b);
    if n % 2 == 0 && m % 2 == 0 && left.len() % 2 != right.len() % 2 {
        out_line!(-1);
        return;
    }
    if left.len() % 2 != right.len() % 2 {
        if n % 2 == 1 {
            for _ in 0..n {
                left.push(1);
            }
        } else {
            for _ in 0..m {
                right.push(1);
            }
        }
    }
    while left.len() > right.len() {
        right.push(1);
        right.push(m);
    }
    while left.len() < right.len() {
        left.push(1);
        left.push(n);
    }
    let ans = left.into_iter().zip(right.into_iter()).collect::<Vec<_>>();
    out_line!(ans.len());
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
