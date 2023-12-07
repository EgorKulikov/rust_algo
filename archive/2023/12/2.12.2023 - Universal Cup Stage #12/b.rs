//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut v = Vec::new();
    let mut start = Vec::with_capacity(n);
    for (i, &a) in a.iter().enumerate() {
        start.push(v.len());
        for _ in 0..a {
            v.push(i);
        }
    }
    start.push(v.len());
    v.push(n);
    type Mod = ModIntF;
    let mut mem = Memoization2d::new(v.len(), v.len(), |mem, at, free| {
        if at == 0 && free == 0 {
            return Mod::one();
        }
        let mut res = Mod::zero();
        if free > 0 {
            res += mem.call(at, free - 1);
        }
        if at > 0 && v[at - 1] == v[at] {
            res += mem.call(at - 1, free);
        }
        for i in 0..v[at] {
            if a[i] == 0 {
                continue;
            }
            res += mem.call(start[i + 1] - 1, free + at - start[i + 1]);
        }
        res
    });
    out.print_line(mem.call(v.len() - 1, 0))
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
