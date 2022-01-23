//{"name":"B. Особые предпочтения в фильмах","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\nzx\nab\ncc\nzx\nba\n2\nab\nbad\n4\nco\ndef\norc\nes\n3\na\nb\nc\n3\nab\ncd\ncba\n2\nab\nab\n","output":"YES\nNO\nNO\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOsobiePredpochteniyaVFilmakh"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Vec<Str> = input.read_vec(n);

    let mut twos = HashSet::new();
    let mut threes = HashSet::new();
    let mut prefixes = HashSet::new();
    for s in s {
        if s.len() == 1 {
            out_line!("YES");
            return;
        }
        let mut t = s.clone();
        t.reverse();
        if s == t {
            out_line!("YES");
            return;
        }
        if t.len() == 2 {
            if twos.contains(&t) || prefixes.contains(&t) {
                out_line!("YES");
                return;
            }
        } else {
            if threes.contains(&t) || twos.contains(&Str::from(&t[..2])) {
                out_line!("YES");
                return;
            }
        }
        if s.len() == 2 {
            twos.insert(s);
        } else {
            prefixes.insert(Str::from(&s[..2]).into_owned());
            threes.insert(s);
        }
    }
    out_line!("NO");
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
