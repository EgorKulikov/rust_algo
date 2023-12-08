//{"name":"H. Dilyor and Domino","group":"CPython.uz - CPython Programming Contest #3","url":"https://cpython.uz/competitions/contests/contest/326/problem/H","interactive":false,"timeLimit":1000,"tests":[{"input":"37 45 45 6\n","output":"3\n"},{"input":"52 13 24 37 44 10\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HDilyorAndDomino"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let d = input.read_size_pair_vec(n);

    let mut mem = Memoization2d::new(n, 2, |mem, pos, side| {
        if pos == 0 {
            return 1;
        }
        let mut res = 1;
        let left = if side == 0 { d[pos].0 } else { d[pos].1 };
        if left == d[pos - 1].1 {
            res.maxim(1 + mem.call(pos - 1, 0));
        }
        if left == d[pos - 1].0 {
            res.maxim(1 + mem.call(pos - 1, 1));
        }
        res
    });
    let mut ans = None;
    for i in 0..n {
        for j in 0..2 {
            ans.maxim(mem.call(i, j));
        }
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
