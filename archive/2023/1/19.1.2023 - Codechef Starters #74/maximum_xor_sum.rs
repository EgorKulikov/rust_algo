//{"name":"Maximum Xor Sum","group":"CodeChef - START74A","url":"https://www.codechef.com/START74A/problems/MAXXORSUM","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3\n0 3 3\n0 2 2\n2\n2 3\n0 1\n","output":"1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MaximumXorSum"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n);
    let b = input.read_unsigned_vec(n);

    #[derive(Copy, Clone)]
    enum State {
        Forbidden,
        Mandatory,
        Any,
    }
    let mut states = Arr2d::new(n, 30, State::Any);
    let mut cur = 0;
    for (i, a) in a.into_iter().enumerate() {
        for j in 0..30 {
            if !a.is_set(j) {
                states[(i, j)] = State::Forbidden;
            } else if !cur.is_set(j) {
                states[(i, j)] = State::Mandatory;
            }
        }
        cur = a;
    }
    let mut cur = u32::all_bits(30);
    for (i, b) in b.into_iter().enumerate().rev() {
        for j in 0..30 {
            if b.is_set(j) {
                states[(i, j)] = State::Mandatory;
            } else if cur.is_set(j) {
                states[(i, j)] = State::Forbidden;
            }
        }
        cur = b;
    }
    let mut ans = 0;
    for i in 0..30 {
        let mut num_mandatory = 0;
        let mut has_any = false;
        for j in 0..n {
            match states[(j, i)] {
                State::Forbidden => {}
                State::Mandatory => num_mandatory += 1,
                State::Any => has_any = true,
            }
        }
        if num_mandatory % 2 == 1 || has_any {
            ans.set_bit(i);
        }
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
}
//END MAIN
