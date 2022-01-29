//{"name":"B - Reserve or Reverse","group":"AtCoder - AtCoder Regular Contest 134","url":"https://atcoder.jp/contests/arc134/tasks/arc134_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4\ndcab\n","output":"acdb\n"},{"input":"2\nab\n","output":"ab\n"},{"input":"16\ncabaaabbbabcbaba\n","output":"aaaaaaabbbbcbbbc\n"},{"input":"17\nsnwfpfwipeusiwkzo\n","output":"effwpnwipsusiwkzo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BReserveOrReverse"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let mut n = input.read_usize();
    let mut s: Str = input.read();

    let mut start = 0;
    for c in b'a'..=b'z' {
        while start < n {
            if s[start] <= c {
                start += 1;
                continue;
            }
            let mut pos = None;
            for i in (start + 1..n).rev() {
                if s[i] == c {
                    pos = Some(i);
                    break;
                }
            }
            match pos {
                None => {
                    break;
                }
                Some(pos) => {
                    s.as_slice_mut().swap(start, pos);
                    n = pos;
                    start += 1;
                }
            }
        }
    }
    out_line!(s);
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
