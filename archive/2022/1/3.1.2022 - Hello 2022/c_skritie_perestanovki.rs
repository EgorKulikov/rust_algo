//{"name":"C. Скрытые перестановки","group":"Codeforces - Hello 2022","url":"https://codeforces.com/contest/1621/problem/C","interactive":true,"timeLimit":1000,"tests":[{"input":"2\n4\n\n3\n\n2\n\n1\n\n4\n\n2\n\n4\n\n4\n","output":"? 3\n\n? 2\n\n? 4\n\n! 4 2 1 3\n\n? 2\n\n? 3\n\n? 2\n\n! 1 3 4 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSkritiePerestanovki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    let mut steps = 0;
    let mut ans = vec![0; n];
    for i in 0..n {
        if ans[i] != 0 {
            continue;
        }
        let mut seq = Vec::new();
        loop {
            steps += 1;
            out_line!('?', i + 1);
            let v = input.read_usize();
            if Some(&v) == seq.first() {
                break;
            }
            seq.push(v);
        }
        let len = seq.len();
        seq.rotate_right((steps - 1) % len);
        assert_eq!(seq[0], i + 1);
        for j in 0..len {
            ans[seq[j] - 1] = seq[(j + 1) % len];
        }
    }
    out_line!('!', ans);
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
