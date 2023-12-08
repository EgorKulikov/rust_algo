//{"name":"K. Panda heroism","group":"CPython.uz - CPython Programming Contest #3","url":"https://cpython.uz/competitions/contests/contest/326/problem/K","interactive":false,"timeLimit":1000,"tests":[{"input":"4 510 25 8 41 21 32 32 43 41 4\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KPandaHeroism"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let edges = input.read_size_pair_vec(m).dec();
    let from = input.read_size() - 1;
    let to = input.read_size() - 1;

    let d = a[from].divisors();
    let mut dsu = DSU::new(n);
    for d in d.into_iter().rev() {
        let d = d as i32;
        for &(f, t) in &edges {
            if a[f] % d == 0 && a[t] % d == 0 {
                dsu.join(f, t);
            }
        }
        if dsu.get(from) == dsu.get(to) {
            out.print_line(d);
            return;
        }
    }
    unreachable!();
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
