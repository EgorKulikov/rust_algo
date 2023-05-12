//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gauss::det;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let b = input.read_long_vec(n);
    let c = input.read_long_vec(n);

    if n > 100 {
        out_line!(0);
        return;
    }
    type Mod = ModIntF;
    let mut m = Arr2d::generate(n, n, |i, j| {
        let v = b[i] ^ c[j];
        Mod::new((v % 998244353).into_i32())
    });
    out_line!(det(&mut m));
}

#[test]
fn test() {
    use algo_lib::misc::random::random;
    let n = 300;
    let mut b = Vec::with_capacity(n);
    let mut c = Vec::with_capacity(n);
    let r = random();
    for _ in 0..n {
        b.push(r.next(1 << 60));
        c.push(r.next(1 << 60));
    }
    type Mod = ModIntF;
    let mut m = Arr2d::generate(n, n, |i, j| {
        let v = b[i] ^ c[j];
        Mod::new((v % 998244353).into_i32())
    });
    use algo_lib::numbers::gauss::gauss;
    gauss(&mut m);
    for i in 0..n {
        for j in 0..n {
            print!("{} ", m[(i, j)].val());
        }
        println!();
    }
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
