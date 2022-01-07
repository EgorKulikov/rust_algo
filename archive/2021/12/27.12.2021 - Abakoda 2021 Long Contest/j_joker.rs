//{"name":"J. Joker","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/J","interactive":false,"timeLimit":1000,"tests":[{"input":"5 26\n2JAK3\n","output":"YES\n2JAK3\n"},{"input":"6 23\n3A4A5*\n","output":"YES\n3A4A59\n"},{"input":"3 31\n***\n","output":"NO\n"},{"input":"26 27\nAAAAAAAAAAAAAAAAAAAAAAAAAA\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JJoker"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    input.next_token();
    let m = input.read_unsigned();
    let mut s: Str = input.read();

    let mut cur = 0;
    let mut num_jokers = 0;
    for c in s.iter() {
        match c {
            b'*' => num_jokers += 1,
            b'T' | b'J' | b'Q' | b'K' => cur += 10,
            b'A' => cur += 1,
            v => {
                assert!(v >= b'2' && v <= b'9');
                cur += (v - b'0').into_u32();
            }
        }
    }
    if cur + num_jokers > m || cur + num_jokers * 10 < m {
        out_line!("NO");
        return;
    }
    let mut delta = m - cur;
    for c in s.iter_mut() {
        if *c == b'*' {
            if delta <= 10 * (num_jokers - 1) + 1 {
                *c = b'A';
                delta -= 1;
            } else if num_jokers > 1 {
                *c = b'T';
                delta -= 10;
            } else {
                *c = {
                    assert!(delta >= 2 && delta <= 10);
                    if delta == 10 {
                        b'T'
                    } else {
                        b'0' + delta.into_u8()
                    }
                };
            }
            num_jokers -= 1;
        }
    }
    out_line!("YES");
    out_line!(s);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
