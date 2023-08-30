//{"name":"A. Eh Seedie, Hot Bel Kherej","group":"Codeforces - Aleppo Collegiate Programming Contest 2023 V.2","url":"https://codeforces.com/gym/104544/problem/A","interactive":false,"timeLimit":4000,"tests":[{"input":"3 9\n15 48 3\n","output":"2\n"},{"input":"5 20\n6 15 2 2 14\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AEhSeedieHotBelKherej"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::{Bounds, Indices, Sorted};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::primes::Factorize;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let a = input.read_long_vec(n);

    if x == 1 {
        out_line!(1);
        return;
    }

    let ds = x.divisors().sorted();
    let edges = Arr2d::generate(ds.len(), ds.len(), |i, j| {
        ds.lower_bound(&gcd(x, ds[i] * ds[j]))
    });
    let mut ans = vec![None; ds.len()];
    ans[0] = Some(0);
    let mut current = vec![1; ds.len()];
    for mut i in a {
        i = gcd(i, x);
        let i = ds.lower_bound(&i);
        let g = gcd(x, current[i] * ds[i]);
        if g == current[i] {
            continue;
        }
        current[i] = g;
        for j in ans.indices().rev() {
            if let Some(v) = ans[j] {
                ans[edges[(i, j)]].minim(v + 1);
            }
        }
    }
    out_line!(ans.pop());
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
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
