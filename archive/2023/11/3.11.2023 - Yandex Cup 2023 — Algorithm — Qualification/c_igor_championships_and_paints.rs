//{"name":"C. Igor, championships and paints","group":"Yandex - Yandex Cup 2023 — Algorithm — Qualification","url":"https://contest.yandex.com/contest/54452/problems/C/","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n2\n8\n1024\n1000000000000000000\n","output":"1\n3\n120\n268174336\n969095352\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CIgorChampionshipsAndPaints"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_u64();

    type Mod = ModInt7;
    let mut ans = Mod::zero();
    for i in 0..60 {
        let full = n / (1u64 << (i + 1));
        let part = n % (1u64 << (i + 1));
        let qty = (full << i) + part.max((1 << i) - 1) - ((1 << i) - 1);
        ans +=
            Mod::new_from_wide(qty as i64) * Mod::new_from_wide(qty as i64) * Mod::new(2).power(i);
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
    let test_type = TestType::MultiNumber;
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
