//{"name":"F. Кормление кошек","group":"Codeforces - Codeforces Round 927 (Div. 3)","url":"https://codeforces.com/contest/1932/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n15 6\n2 10\n3 5\n2 4\n7 7\n8 12\n11 11\n1000 1\n1 1000\n5 10\n1 2\n3 4\n3 4\n3 4\n3 4\n1 1\n1 2\n3 3\n3 4\n3 4\n","output":"5\n1\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKormlenieKoshek"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut cats = input.read_size_pair_vec(m).dec();

    cats.sort();
    let mut delta = Vec::with_capacity(n);
    let mut remove = vec![0; n];
    let mut last = Vec::with_capacity(n);
    let mut cur_last = 0;
    let mut at = 0;
    let mut cur_cats = 0;
    for i in 0..n {
        while at < m && cats[at].0 == i {
            cur_cats += 1;
            cur_last.maxim(cats[at].1);
            remove[cats[at].1] += 1;
            at += 1;
        }
        delta.push(cur_cats);
        last.push(cur_last);
        cur_cats -= remove[i];
    }

    let mut mem = Memoization1d::new(n + 1, |mem, at| {
        if at == n {
            return 0;
        }
        if delta[at] == 0 {
            return mem.call(at + 1);
        }
        mem.call(at + 1).max(delta[at] + mem.call(last[at] + 1))
    });
    out.print_line(mem.call(0));
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
    //    tester::stress_test();
}
//END MAIN
