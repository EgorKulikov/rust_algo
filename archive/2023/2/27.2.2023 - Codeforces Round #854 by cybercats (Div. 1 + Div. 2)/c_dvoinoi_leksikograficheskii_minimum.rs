//{"name":"C. Двойной лексикографический минимум","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"12\na\naab\nabb\nabc\naabb\naabbb\naaabb\nabbb\nabbbb\nabbcc\neaga\nffcaba\n","output":"a\naba\nbab\nbca\nabba\nabbba\nababa\nbbab\nbbabb\nbbcca\nagea\nacffba\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDvoinoiLeksikograficheskiiMinimum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let mut s = s.to_vec();
    s.sort();
    let mut head = Vec::new();
    let mut tail = Vec::new();
    for i in (0..s.len()).step_by(2) {
        if i == s.len() - 1 {
            head.push(s[i]);
            continue;
        }
        if s[i] == s[i + 1] {
            head.push(s[i]);
            tail.push(s[i]);
            continue;
        }
        if s[s.len() - 1] == s[i + 1] {
            let q = s.len() - i - 1;
            for _ in 0..(q + 1) / 2 {
                head.push(s[i + 1]);
            }
            for _ in 0..q / 2 {
                tail.push(s[i + 1]);
            }
            tail.push(s[i]);
            break;
        }
        tail.push(s[i]);
        head.extend_from_slice(&s[i + 1..]);
        break;
    }
    tail.reverse();
    head.extend(tail);
    out_line!(Str::from(head));
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
