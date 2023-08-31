//{"name":"F. Разделяй, считай XOR и властвуй","group":"Codeforces - Pinely Round 2 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1863/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6\n3 2 1 3 7 4\n5\n1 1 1 1 1\n10\n1 2 4 8 4 1 2 3 4 5\n5\n0 0 0 0 0\n5\n1 2 3 0 1\n1\n100500\n","output":"111111\n10101\n0001000000\n11111\n11001\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRazdelyaiSchitaiXORIVlastvui"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_u64_vec(n);

    let mut x = Vec::with_capacity(n + 1);
    x.push(0);
    let mut cur = 0;
    for &i in &a {
        cur ^= i;
        x.push(cur);
    }
    let mut can = vec![BitSet::new(n + 1); n];
    can[0].set(n);
    let mut left = vec![0u64; n + 1];
    let mut right = vec![0u64; n + 1];
    let mut left_zero = vec![false; n + 1];
    let mut right_zero = vec![false; n + 1];
    for i in 0..n {
        for j in (i + 1..=n).rev() {
            let xx = x[i] ^ x[j];
            let mut good =
                can[i][j] || ((left[i] | right[j]) & xx) != 0 || left_zero[i] || right_zero[j];
            if good {
                can[i].set(j);
                if xx == 0 {
                    left_zero[i] = true;
                    right_zero[j] = true;
                } else {
                    let k = (63 - xx.leading_zeros()).into_usize();
                    left[i].set_bit(k);
                    right[j].set_bit(k);
                }
            }
        }
    }
    let mut ans = Str::new();
    for i in 0..n {
        if can[i][i + 1] {
            ans.push(b'1');
        } else {
            ans.push(b'0');
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
