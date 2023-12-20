//{"name":"G1. Лампочки (простая версия)","group":"Codeforces - Codeforces Round 916 (Div. 3)","url":"https://codeforces.com/contest/1914/problem/G1","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2\n2 2 1 1\n2\n1 2 2 1\n2\n1 2 1 2\n5\n3 4 4 5 3 1 1 5 2 2\n","output":"2 4\n1 2\n1 4\n2 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1LampochkiProstayaVersiya"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::random;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let c = input.read_size_vec(2 * n).dec();

    let mut set = BitSet::new(n);
    let mut qty = 0;
    let mut ends = Vec::new();
    for (i, &c) in c.iter().enumerate() {
        if qty == 0 {
            ends.push(i);
        }
        if set[c] {
            qty -= 1;
        } else {
            qty += 1;
        }
        set.flip(c);
    }
    ends.push(2 * n);
    type Mod = ModIntF;
    let mut r = Vec::with_capacity(n);
    for _ in 0..n {
        r.push(random().gen());
    }
    let mut ans = Mod::one();
    let mut bad = vec![0usize; n * 2];
    for (&f, &t) in ends.consecutive_iter() {
        let mut cur = 0;
        let mut h = 0;
        let mut was = HashMap::<_, usize>::new();
        for (i, &c) in (f..t).zip(c[f..t].iter()) {
            if let Some(&prev) = was.get(&h) {
                bad[prev].maxim(i);
            }
            was.insert(h, i);
            h ^= r[c];
        }
        let mut until = f;
        for i in f..t {
            if bad[i] != 0 {
                until.maxim(bad[i]);
            }
            if until <= i {
                cur += 1;
            }
        }
        ans *= Mod::from_index(cur.max(2));
    }
    out.print_line((ends.len() - 1, ans));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    tester::stress_test(run, tester::check);
}
//END MAIN
