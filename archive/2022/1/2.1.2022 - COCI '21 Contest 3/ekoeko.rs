//{"name":"#4 - Ekoeko","group":"DMOJ - COCI '21 Contest 3","url":"https://dmoj.ca/problem/coci21c3p4","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nkoeeok\n","output":"3\n"},{"input":"3\nkekoeo\n","output":"1\n"},{"input":"4\nsoolnlsn\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Ekoeko"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let s: Str = input.read();

    let mut qty = vec![0; 26];
    for c in s.iter() {
        qty[(c - b'a').into_usize()] += 1;
    }
    for c in s.iter().take(n) {
        qty[(c - b'a').into_usize()] -= 2;
    }
    let mut front_front = Str::new();
    let mut front_back = Str::new();
    let mut ans = 0;
    for c in s.iter().take(n).rev() {
        let id = (c - b'a').into_usize();
        if qty[id] >= 0 {
            front_front += c;
        } else {
            qty[id] += 2;
            front_back += c;
            ans += front_front.len();
        }
    }
    front_front.reverse();
    front_back.reverse();
    let mut back_front = Str::new();
    let mut back_back = Str::new();
    for c in s.iter().skip(n) {
        let id = (c - b'a').into_usize();
        if qty[id] == 0 {
            back_back += c;
        } else {
            qty[id] -= 2;
            back_front += c;
            ans += back_back.len();
        }
    }
    ans += front_back.len() * back_front.len();
    let left = front_front + back_front;
    let right = front_back + back_back;
    let mut pos = vec![Vec::new(); 26];
    for (i, c) in right.iter().enumerate().rev() {
        pos[(c - b'a').into_usize()].push(i);
    }
    let mut ft = FenwickTree::new(n);
    for (i, c) in left.iter().enumerate() {
        let j = pos[(c - b'a').into_usize()].pop().unwrap();
        let j_pos = j + ft.get(j, n);
        ft.add(j, 1);
        ans += j_pos - i;
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
