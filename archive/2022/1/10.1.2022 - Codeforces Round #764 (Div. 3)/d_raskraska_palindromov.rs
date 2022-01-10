//{"name":"D. Раскраска палиндромов","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n8 2\nbxyaxzay\n6 3\naaaaaa\n6 1\nabcdef\n6 6\nabcdef\n3 2\ndxd\n11 2\nabcabcabcac\n6 6\nsipkic\n7 2\neatoohd\n3 1\nllw\n6 2\nbfvfbv\n","output":"3\n2\n1\n1\n1\n5\n1\n1\n3\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRaskraskaPalindromov"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let s: Str = input.read();

    let mut qty = [0; 26];
    for c in s {
        qty[(c - b'a').into_usize()] += 1;
    }
    let mut ans = vec![0; k];
    let mut at = 0;
    for i in &mut qty {
        while *i >= 2 {
            ans[at] += 2;
            *i -= 2;
            at += 1;
            if at == k {
                at = 0;
            }
        }
    }
    let full = ans[k - 1];
    for mut i in qty {
        while i >= 1 {
            ans[at] += 1;
            i -= 1;
            at += 1;
            if at == k {
                break;
            }
        }
        if at == k {
            break;
        }
    }
    out_line!(ans[k - 1].max((full + 1).min(n / k)));
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
