//{"name":"D1. Слишком много предателей (простая версия)","group":"Codeforces - Codeforces Round #761 (Div. 2)","url":"https://codeforces.com/contest/1617/problem/D1","interactive":true,"timeLimit":3000,"tests":[{"input":"2\n6\n\n0\n\n1\n\n9\n\n1\n","output":"? 1 2 3\n\n? 3 4 5\n\n! 3 4 1 2\n\n? 7 1 9\n\n! 4 2 3 6 8\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1SlishkomMnogoPredateleiProstayaVersiya"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, OUTPUT};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut res = Vec::new();
    let mut ans = BitSet::new(n);

    for i in 0..(n - 2) {
        out_line!('?', i + 1, i + 2, i + 3);

        if input.read::<i32>() == 0 {
            res.push(true);
        } else {
            res.push(false);
        }
        if res[0] != res[i] {
            ans.set(i + 2, res[i]);
            ans.set(i - 1, res[i - 1]);
            out_line!('?', i, i + 3, i + 1);
            ans.set(i, input.read::<i32>() == 0);
            let x = ans[i];
            ans.set(i + 1, !x);
            for j in (0..i - 1).rev() {
                if ans[j + 1] != ans[j + 2] {
                    ans.set(j, res[j]);
                } else {
                    out_line!('?', i, i + 3, j + 1);
                    ans.set(j, input.read::<i32>() == 0);
                }
            }
            for j in (i + 3)..n {
                out_line!('?', i, i + 3, j + 1);
                ans.set(j, input.read::<i32>() == 0);
            }
            let bad = ans.iter().collect_vec().inc_by_one();
            out_line!("!", bad.len(), bad);
            return;
        }
    }
    panic!("unreachable");
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
    let mut sin = std::io::stdin();
    let input = Input::new_with_size(&mut sin, 1);
    unsafe {
        OUTPUT = Some(Output::new_with_auto_flush(Box::new(std::io::stdout())));
    }
    run(input);
}
//END MAIN
