//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt9;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::factorial;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();

    type Mod = ModInt9;
    let mut ans: Mod = factorial(n);
    let mut x = Mod::from_index(n) * Mod::from_index(n - 1) / Mod::new(2);
    for i in 1..=m {
        ans *= x;
        x -= Mod::one();
        ans /= Mod::from_index(i);
    }
    out_line!(ans);
}

#[test]
fn test() {
    use algo_lib::collections::arr2d::Arr2d;
    use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
    for n in 2..=5 {
        let mut ans = vec![0; n * (n - 1) / 2 + 1];
        let mut g = Arr2d::new(n, n, false);
        let mut cl = Arr2d::new(n, n, false);
        let mut f = RecursiveFunction2::new(|f, i, j| {
            if i == n {
                for x in 0..n {
                    for y in 0..n {
                        cl[(x, y)] = g[(x, y)];
                    }
                }
                for x in 0..n {
                    for y in 0..n {
                        for z in 0..n {
                            if cl[(y, x)] && cl[(x, z)] {
                                cl[(y, z)] = true;
                            }
                        }
                    }
                }
                let mut good = true;
                let mut cnt = 0;
                for x in 0..n {
                    let mut back = 0;
                    for y in 0..n {
                        if g[(x, y)] {
                            if cl[(y, x)] {
                                back += 1;
                            } else {
                                cnt += 1;
                            }
                        }
                    }
                    if back > 1 {
                        good = false;
                        break;
                    }
                }
                if good {
                    ans[cnt] += 1;
                }
                return;
            }
            if j == n {
                f.call(i + 1, 0);
                return;
            }
            f.call(i, j + 1);
            if i == j {
                return;
            }
            g[(i, j)] = true;
            f.call(i, j + 1);
            g[(i, j)] = false;
        });
        f.call(0, 0);
        let ff: i32 = factorial(n);
        for i in 0..ans.len() {
            ans[i] /= ff;
        }
        println!("n = {}, ans = {:?}", n, ans);
    }
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
