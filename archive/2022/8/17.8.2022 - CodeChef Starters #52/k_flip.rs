//{"name":"K-Flip","group":"CodeChef - START52A","url":"https://www.codechef.com/START52A/problems-old/KLIP","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 2\n101\n4 2\n1011\n5 3\n01010\n5 2\n01010\n","output":"000\n0001\n00011\n00000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KFlip"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let mut s = input.read_vec::<char>(n);

    let mut flip = BitSet::new(n + 1);
    let mut cur_flip = false;
    for i in 0..n {
        let mut c = s[i];
        cur_flip ^= flip[i];
        if cur_flip {
            if c == '0' {
                c = '1';
            } else {
                c = '0';
            }
        }
        if i <= n - k && c == '1' {
            c = '0';
            cur_flip ^= true;
            flip.set(i + k, true);
        }
        s[i] = c;
    }
    for c in s {
        out!(c);
    }
    out_line!();
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
}
//END MAIN
