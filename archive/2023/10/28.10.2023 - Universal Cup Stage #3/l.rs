//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let tp = input.read_str();
    let n = input.read_size();
    let a = input.read_u64_vec(n);

    fn reverse_bits(mut n: u64) -> u64 {
        let mut res = 0;
        for _ in 0..8 {
            res <<= 8;
            res |= n & 0xff;
            n >>= 8;
        }
        res
    }

    match tp.as_slice() {
        b"encode" => {
            let mut coded = Vec::new();
            coded.push(n as u64);
            let mut bits = Vec::new();
            for i in a {
                if reverse_bits(i) < i {
                    bits.push(true);
                } else {
                    bits.push(false);
                }
                coded.push(i);
            }
            for i in 0..n.upper_div(48) {
                let mut cur = 1u64;
                for j in i * 48..((i + 1) * 48).min(n) {
                    if bits[j] {
                        cur.set_bit(j - i * 48 + 8);
                    }
                }
                coded.push(cur);
            }
            out.print_line(coded.len());
            out.print_line(coded);
        }
        b"decode" => {
            let len = a[0].min(reverse_bits(a[0])) as usize;
            let mut bits = Vec::new();
            for mut i in a.iter().copied().skip(len + 1) {
                if !i.is_set(0) {
                    i = reverse_bits(i);
                }
                for j in 0..48 {
                    bits.push(i.is_set(j + 8));
                }
            }
            let mut ans = Vec::with_capacity(len);
            for (id, i) in a.into_iter().skip(1).take(len).enumerate() {
                if bits[id] ^ (reverse_bits(i) < i) {
                    ans.push(reverse_bits(i));
                } else {
                    ans.push(i);
                }
            }
            out.print_line(ans);
        }
        _ => unreachable!(),
    }
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
