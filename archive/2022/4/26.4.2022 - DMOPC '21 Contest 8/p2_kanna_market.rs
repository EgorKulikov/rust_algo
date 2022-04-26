//{"name":"P2 - Kanna Market","group":"DMOJ - DMOPC '21 Contest 8","url":"https://dmoj.ca/problem/dmopc21c8p2","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n2 1 0\n0 1 0\n","output":"3\n"},{"input":"3 20\n1 1 1\n1 1 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P2KannaMarket"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_int();
    let a = input.read_table::<i32>(2, n);

    type Mod = ModInt7;
    let mut ans = Mod::one();
    for i in 0..2 {
        let mut min = 2;
        let mut max = 2 * m;
        let mut free = 0;
        for j in (i..n).step_by(2) {
            let x = a[(0, j)];
            let y = a[(1, j)];
            if x == 0 {
                if y == 0 {
                    free += 1;
                } else {
                    min.maxim(y + 1);
                    max.minim(y + m);
                }
            } else {
                if y == 0 {
                    min.maxim(x + 1);
                    max.minim(x + m);
                } else {
                    min.maxim(x + y);
                    max.minim(x + y);
                }
            }
        }
        if min > max {
            ans = Mod::zero();
            break;
        }
        let mut cur = Mod::zero();
        for i in min..=max {
            if i <= m + 1 {
                cur += Mod::new(i - 1).power(free);
            } else {
                cur += Mod::new(2 * m + 1 - i).power(free);
            }
        }
        ans *= cur;
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
