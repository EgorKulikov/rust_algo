//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut ans = 0;
    let mut need_xor = false;
    for i in 0..(n.trailing_zeros() as usize) {
        let mut ones = DefaultHashMap::<_, usize>::new();
        let mut zeros = DefaultHashMap::<_, usize>::new();
        let mut one_zero = 0;
        let mut zero_one = 0;
        for &j in &a {
            let prefix = j >> (i + 1);
            if j.is_set(i) {
                zero_one += zeros[prefix];
                ones[prefix] += 1;
            } else {
                one_zero += ones[prefix];
                zeros[prefix] += 1;
            }
        }
        if zero_one < one_zero {
            need_xor = true;
        }
        ans += zero_one.min(one_zero);
    }
    if need_xor {
        ans += 1;
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
