//{"name":"F. Final Picture","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"9 3\n","output":"92\n"},{"input":"20 22\n","output":"84450197\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFinalPicture"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let h = input.read_usize();
    let t = input.read_usize();

    type Mod = ModIntF;
    let mut ans = Arr3d::new(h + 1, h + 1, t + 1, None);
    let mut rec = RecursiveFunction3::new(|f, id, height, rem| -> Mod {
        match ans[(id, height, rem)] {
            None => {
                let res = {
                    if id == h {
                        if rem == 0 {
                            Mod::one()
                        } else {
                            Mod::zero()
                        }
                    } else {
                        let mut res = Mod::zero();
                        if id + height < h && rem > 0 {
                            res += f.call(id, height + 1, rem - 1);
                        }
                        if height == 0 {
                            res += f.call(id + 1, 0, rem);
                        } else if rem >= height - 1 {
                            res += f.call(id + 1, height - 1, rem - (height - 1));
                        }
                        res
                    }
                };
                ans[(id, height, rem)] = Some(res);
                res
            }
            Some(res) => res,
        }
    });
    out_line!(rec.call(0, 0, t));
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
