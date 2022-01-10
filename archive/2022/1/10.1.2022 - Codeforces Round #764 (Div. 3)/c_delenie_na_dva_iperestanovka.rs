//{"name":"C. Деление на два и перестановка","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n4\n1 8 25 2\n2\n1 1\n9\n9 8 3 4 2 7 1 5 6\n3\n8 2 1\n4\n24 7 16 7\n5\n22 6 22 4 22\n","output":"YES\nNO\nYES\nNO\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDelenieNaDvaIPerestanovka"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_unsigned_vec(n);

    let mut used = BitSet::new(n);
    for i in (1..=n).rev() {
        let pos = a.iter().enumerate().find(|(j, v)| {
            if used[*j] {
                false
            } else {
                let mut v = v.into_usize();
                while v > i {
                    v /= 2;
                }
                v == i
            }
        });
        match pos {
            None => {
                out_line!("NO");
                return;
            }
            Some((j, _)) => {
                used.set(j, true);
            }
        }
    }
    out_line!("YES");
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
