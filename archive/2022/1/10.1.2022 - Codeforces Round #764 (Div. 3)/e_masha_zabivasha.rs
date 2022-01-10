//{"name":"E. Маша-забываша","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n\n4 8\n12340219\n20215601\n56782022\n12300678\n12345678\n\n2 3\n134\n126\n123\n\n1 4\n1210\n1221\n\n4 3\n251\n064\n859\n957\n054\n\n4 7\n7968636\n9486033\n4614224\n5454197\n9482268\n","output":"3\n1 4 1\n5 6 2\n3 4 3\n-1\n2\n1 2 1\n2 3 1\n-1\n3\n1 3 2\n5 6 3\n3 4 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMashaZabivasha"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let old: Vec<Str> = input.read_vec(n);
    let target: Str = input.read();

    let mut known = HashMap::new();
    for (i, s) in old.into_iter().enumerate() {
        for j in 0..m {
            if j + 2 <= m {
                known.insert(s[j..j + 2].to_vec(), (j + 1, j + 2, i + 1));
            }
            if j + 3 <= m {
                known.insert(s[j..j + 3].to_vec(), (j + 1, j + 3, i + 1));
            }
        }
    }
    let mut last = vec![None; m + 1];
    last[0] = Some(Vec::new());
    for i in 0..m {
        if last[i].is_some() {
            if i + 2 <= m && known.contains_key(&target[i..i + 2].to_vec()) {
                last[i + 2] = Some(target[i..i + 2].to_vec());
            }
            if i + 3 <= m && known.contains_key(&target[i..i + 3].to_vec()) {
                last[i + 3] = Some(target[i..i + 3].to_vec());
            }
        }
    }
    match last[m] {
        None => {
            out_line!(-1);
        }
        Some(_) => {
            let mut at = m;
            let mut ans = Vec::new();
            while at > 0 {
                ans.push(known[last[at].as_ref().unwrap()]);
                at -= last[at].as_ref().unwrap().len();
            }
            ans.reverse();
            out_line!(ans.len());
            output().print_per_line(&ans);
        }
    }
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
