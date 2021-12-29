//{"name":"E. Лексикографически достаточно малая","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"https://codeforces.com/contest/1616/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\na\na\n3\nrll\nrrr\n3\ncaa\naca\n5\nababa\naabba\n","output":"-1\n0\n2\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ELeksikograficheskiDostatochnoMalaya"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Str = input.read();
    let t: Str = input.read();

    let mut pos = vec![Vec::new(); 26];
    for (i, c) in s.into_iter().enumerate().rev() {
        pos[(c - b'a').into_usize()].push(i);
    }
    let mut ft = FenwickTree::new(n);
    let mut ans = None;
    let mut cur = 0;
    for (i, c) in t.into_iter().enumerate() {
        let c = (c - b'a').into_usize();
        for j in 0..c {
            if let Some(v) = pos[j].last().cloned() {
                let cur = cur + v + ft.get(v, n) - i;
                ans.minim(cur);
            }
        }
        if let Some(v) = pos[c].last().cloned() {
            cur += v + ft.get(v, n) - i;
            ft.add(v, 1);
            pos[c].pop();
        } else {
            break;
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
