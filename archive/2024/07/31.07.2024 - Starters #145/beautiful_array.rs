//{"name":"Beautiful Array","group":"CodeChef - START145A","url":"https://www.codechef.com/START145A/problems/ARRBEAUT","interactive":false,"timeLimit":2500,"tests":[{"input":"1\n5 2\n1 2 3 4 5\n5 3\n1 4\n","output":"2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BeautifulArray"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_unsigned_vec(n);

    let mut all_ones = Vec::with_capacity(20);
    let mut all_zeroes_zeroes = Vec::with_capacity(20);
    let mut all_zeroes_ones = Vec::with_capacity(20);
    let mut segments = Vec::with_capacity(20);

    for i in 0..20 {
        let mut ones = u32::all_bits(20);
        let mut zeroes_zeroes = u32::all_bits(20);
        let mut zeroes_ones = u32::all_bits(20);
        let mut num_segments = 0;
        let mut last_zero = false;
        for &a in &a {
            if a.is_set(i) {
                last_zero = false;
                ones &= a;
            } else {
                if !last_zero {
                    num_segments += 1;
                }
                last_zero = true;
                zeroes_zeroes &= u32::all_bits(20) - a;
                zeroes_ones &= a;
            }
        }
        all_ones.push(ones);
        all_zeroes_zeroes.push(zeroes_zeroes);
        all_zeroes_ones.push(zeroes_ones);
        segments.push(num_segments);
    }

    for _ in 0..q {
        let k = input.read_size();
        let x = input.read_unsigned();

        let mut ans = all_ones[0] & all_zeroes_ones[0];
        for i in 0..20 {
            if x.is_set(i) && segments[i] <= k {
                let zeroes =
                    (all_zeroes_zeroes[i] & x) + (all_zeroes_ones[i] & (u32::all_bits(20) - x));
                ans.maxim(zeroes & all_ones[i]);
            }
        }
        out.print_line(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
