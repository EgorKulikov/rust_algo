//{"name":"day3a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day3a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::string::string::Str;
use algo_lib::{out_line, when};

fn solve(input: &mut Input, _test_case: usize) {
    fn priority(c: u8) -> usize {
        when! {
            c.is_ascii_lowercase() => (c - b'a').into_usize() + 1,
            c.is_ascii_uppercase() => (c - b'A').into_usize() + 27,
            else => unreachable!(),
        }
    }

    fn mask(s: &[u8]) -> u64 {
        let mut ans = 0u64;
        for &c in s {
            ans.set_bit(priority(c));
        }
        ans
    }

    let mut ans = 0;

    while !input.is_exhausted() {
        let s = input.read_vec::<Str>(3);
        let mut cur = u64::MAX;
        for s in &s {
            cur &= mask(s.as_slice());
        }

        assert_eq!(cur.count_ones(), 1);

        ans += cur.trailing_zeros();

        input.skip_whitespace();
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
