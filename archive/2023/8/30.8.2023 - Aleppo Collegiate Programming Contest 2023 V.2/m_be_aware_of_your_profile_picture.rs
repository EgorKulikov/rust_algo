//{"name":"M. Be Aware of Your Profile Picture","group":"Codeforces - Aleppo Collegiate Programming Contest 2023 V.2","url":"https://codeforces.com/gym/104544/problem/M","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 4\n1 2 9 15 4\n2 2\n2 2\n","output":"8\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MBeAwareOfYourProfilePicture"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_unsigned_vec(n);

    let all_bits = u32::all_bits(k);
    let mut ans = all_bits;
    let mut used = BitSet::new(n);
    for i in (0..k).rev() {
        if !ans.is_set(i) {
            continue;
        }
        for (j, &a) in a.iter().enumerate() {
            if !used[j] && !a.is_set(i) {
                used.set(j);
                ans &= a.with_bit(i);
            }
        }
    }
    if ans == all_bits && used.count_ones() != n {
        ans -= 1;
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
