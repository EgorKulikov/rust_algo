//{"name":"#4 - Kocke","group":"DMOJ - COCI '23 Contest 1","url":"https://dmoj.ca/problem/coci23c1p4","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n","output":"8\n"},{"input":"3 5\n","output":"14\n"},{"input":"100 200\n","output":"410783331\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Kocke"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    type Mod = ModInt7;
    let mut mem = Memoization2d::new(n, k + 1, |mem, last, len| -> (Mod, Mod) {
        let mut res = Mod::new(2) * Mod::from_index(k - len + 1);
        if last > 1 && len < k {
            let (_, call) = mem.call(last - 1, len + 1);
            res += call;
        }
        if last > len && len < k {
            let (_, call) = mem.call(last - len, len + 1);
            res += call;
        }
        let mut sum = res;
        if last > 2 {
            let (_, call) = mem.call(last - 2, len);
            sum += call;
        }
        (res, sum)
    });
    out.print_line(mem.call(n - 1, 2).0);
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
