//{"name":"C. Jumping on Tiles","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6\nlogic\ncodeforces\nbca\naaaaaaaaaaa\nadbaadabad\nto\n","output":"9 4\n1 4 3 5\n16 10\n1 8 3 4 9 5 2 6 7 10\n1 2\n1 3\n0 11\n1 8 10 4 3 5 7 2 9 6 11\n3 10\n1 9 5 4 7 3 8 6 2 10\n5 2\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CJumpingOnTiles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::sign::Unsigned;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let mut pos = vec![Vec::new(); 26];
    for (i, c) in s.iter().enumerate() {
        pos[c as usize - 'a' as usize].push(i + 1);
    }
    let a = (s[0] - b'a').into_usize();
    let b = (s[s.len() - 1] - b'a').into_usize();
    let mut ans = Vec::new();
    if a <= b {
        for i in a..=b {
            ans.extend(pos[i].iter().copied());
        }
    } else {
        for i in (b..=a).rev() {
            ans.extend(pos[i].iter().copied());
        }
    }
    out_line!(a.distance(b), ans.len());
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
