//{"name":"The Stark Reactor","group":"CodeChef - STRV2022","url":"https://www.codechef.com/STRV2022/problems/NUKE_REACTOR","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n0 1\n1 0\n1\n10 10 5 5\n","output":"1\n"},{"input":"4\n1 0\n0 1\n1 2\n2 1\n1\n10 10 2 2\n","output":"17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TheStarkReactor"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let k = input.read_usize();
    let shifts = input.read_usize_pair_vec(k);

    const BUBEN: usize = 200;
    type Mod = ModInt7;
    let mut ans = Arr2d::new(BUBEN + 1, BUBEN + 1, Mod::zero());
    ans[(0, 0)] = Mod::one();
    for (dx, dy) in shifts {
        assert!(dx != 0 || dy != 0);
        for i in 0..=BUBEN - dx {
            for j in 0..=BUBEN - dy {
                let add = ans[(i, j)];
                ans[(i + dx, j + dy)] += add;
            }
        }
    }

    let q = input.read_usize();
    for _ in 0..q {
        let m1 = input.read_usize();
        let z1 = input.read_usize();
        let m2 = input.read_usize();
        let z2 = input.read_usize();
        out_line!(ans[(m1 - m2, z1 - z2)]);
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
