//{"name":"Expected Value Problem","group":"CodeChef - START156A","url":"https://www.codechef.com/START156A/problems/EVVAL","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n5\n2\n1 2\n3\n1 2 3\n4\n5 5 5 5\n","output":"5\n500000008\n12\n50\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ExpectedValueProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::with::With;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::invertible::Invertible;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    type Mod = ModInt7;
    let mut base =
        Mod::from_index(n).with(|n| (n * n * Mod::new(3) - n + Mod::new(2)) / Mod::new(2));
    let den = Mod::from_index(2 * n).inv().unwrap();
    let mut ans = Mod::zero();
    let mut delta = Mod::from_index(3 * n) - Mod::new(6);
    for a in a {
        ans += den * base * Mod::new(a);
        base -= delta;
        delta -= Mod::new(6);
    }
    out.print_line(ans);
}

#[cfg(test)]
mod test {
    use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
    use algo_lib::numbers::mod_int::ModInt7;
    use algo_lib::numbers::num_traits::algebra::Zero;
    use algo_lib::numbers::num_traits::as_index::AsIndex;
    use algo_lib::numbers::num_traits::invertible::Invertible;

    #[test]
    fn test() {
        type Mod = ModInt7;
        for n in 1..=10 {
            let mut p = vec![Mod::zero(); n];
            for i in 0..n {
                let add = Mod::from_index(n).inv().unwrap();
                p[i] += add;
                let mut rec = RecursiveFunction3::new(|rec, f: usize, t: usize, mut add: Mod| {
                    if f == 0 {
                        if t != n {
                            p[t] += add * Mod::from_index(t - f + 1);
                            rec.call(f, t + 1, add);
                        }
                    } else {
                        if t == n {
                            p[f - 1] += add * Mod::from_index(t - f + 1);
                            rec.call(f - 1, t, add);
                        } else {
                            add /= Mod::new(2);
                            p[f - 1] += add * Mod::from_index(t - f + 1);
                            p[t] += add * Mod::from_index(t - f + 1);
                            rec.call(f - 1, t, add);
                            rec.call(f, t + 1, add);
                        }
                    }
                });
                rec.call(i, i + 1, add);
            }
            for i in 0..n {
                p[i] *= Mod::from_index(n * 2);
            }
            println!("{:?}", p);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
