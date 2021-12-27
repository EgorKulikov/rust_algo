//{"name":"E. Тест по математике","group":"Codeforces - Educational Codeforces Round 120 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1622/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 3\n5 1 2 2\n110\n100\n101\n100\n4 4\n6 2 0 10\n1001\n0010\n0110\n0101\n3 6\n20 3 15\n010110\n000101\n111111\n","output":"3 1 2\n2 3 4 1\n3 1 4 5 2 6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETestPoMatematike"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::permutation::Permutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let x = input.read_int_vec(n);
    let s: Vec<Str> = input.read_vec(n);

    let s = s
        .into_iter()
        .map(|s| {
            let mut res = BitSet::new(m);
            for (i, c) in s.into_iter().enumerate() {
                if c == b'1' {
                    res.set(i, true);
                }
            }
            res
        })
        .collect_vec();
    let mut order = (0..m).collect_vec();
    let mut score = vec![0; m];
    let mut ans = -1;
    let mut ans_at = vec![0usize; m];
    for mask in 0..(1 << n) {
        let mut base = 0;
        score.fill(0);
        for i in 0..n {
            let up = mask.is_set(i);
            if up {
                base -= x[i];
            } else {
                base += x[i];
            }
            for j in s[i].iter() {
                if up {
                    score[j] += 1;
                } else {
                    score[j] -= 1;
                }
            }
        }
        order.sort_by_key(|i| score[*i]);
        for (j, i) in order.iter().cloned().enumerate() {
            base += (j + 1).into_i32() * score[i];
        }
        if base > ans {
            ans = base;
            ans_at.as_mut_slice().copy_from_slice(order.as_slice());
        }
    }
    let p = Permutation::new(ans_at);
    let mut ans = p.inv();
    ans.set_base(1);
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
