//{"name":"G1. Магические тройки (простая версия)","group":"Codeforces - Codeforces Round 867 (Div. 3)","url":"https://codeforces.com/contest/1822/problem/G1","interactive":false,"timeLimit":4000,"tests":[{"input":"7\n5\n1 7 7 2 7\n3\n6 2 18\n9\n1 2 3 4 5 6 7 8 9\n4\n1000 993 986 179\n7\n1 10 100 1000 10000 100000 1000000\n8\n1 1 2 2 4 4 8 8\n9\n1 1 1 2 2 2 4 4 4\n","output":"6\n1\n3\n0\n9\n16\n45\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1MagicheskieTroikiProstayaVersiya"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::primes::all_divisors;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut q = DefaultHashMap::<_, usize>::new();
    for &x in &a {
        q[x] += 1;
    }
    let mut ans = 0;
    for &i in q.values() {
        if i >= 3 {
            ans += i * (i - 1) * (i - 2);
        }
    }
    const BUBEN: usize = 100000;
    let dd = all_divisors::<i64>(BUBEN, false);
    for &x in q.keys() {
        let qx = q[x];
        if x.into_usize() >= BUBEN {
            for j in 2..=1_000_000_000 / x {
                let d = x / j;
                if x == d * j {
                    ans += qx * q[d] * q[x * j];
                }
            }
        } else {
            for &d in &dd[x.into_usize()] {
                if d != 1 {
                    ans += qx * q[x / d] * q[x * d];
                }
            }
        }
        // let mut j = 2;
        // while x * j * j <= 1_000_000_000 {
        //     ans += q[x] * q[x * j] * q[x * j * j];
        //     j += 1;
        // }
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
