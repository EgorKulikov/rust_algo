//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::primes::primes;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_int();
    let k = input.read_int();

    if n % 2 != k % 2 {
        out_line!(-1);
        return;
    }
    let mut cur = n;
    let mut ans = vec![1i32; n.into_usize()];
    let p = primes(n.into_usize() + 1);
    for i in p {
        let was_cur = cur;
        for j in (i..=n).step_by(i.into_usize()) {
            let mut times = 1;
            let mut x = j / i;
            while x % i == 0 {
                x /= i;
                times += 1;
            }
            if times % 2 == 1 {
                let id = j.into_usize() - 1;
                cur -= ans[id];
                ans[id] *= -1;
                cur += ans[id];
            }
        }
        if cur < k || cur > was_cur {
            for j in (i..=n).step_by(i.into_usize()) {
                let mut times = 1;
                let mut x = j / i;
                while x % i == 0 {
                    x /= i;
                    times += 1;
                }
                if times % 2 == 1 {
                    let id = j.into_usize() - 1;
                    cur -= ans[id];
                    ans[id] *= -1;
                    cur += ans[id];
                }
            }
        }
    }
    if cur > k {
        out_line!(-1);
        return;
    }
    assert_eq!(cur, k);
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
