//{"name":"F. Саша и свадебное бинарное дерево поиска","group":"Codeforces - Codeforces Round 926 (Div. 2)","url":"https://codeforces.com/contest/1929/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 5\n2 3 -1\n-1 -1 2\n4 -1 3\n-1 5 -1\n-1 -1 -1\n3 69\n2 3 47\n-1 -1 13\n-1 -1 69\n3 3\n2 3 -1\n-1 -1 -1\n-1 -1 -1\n","output":"4\n1\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSashaISvadebnoeBinarnoeDerevoPoiska"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::combinations;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let c: isize = input.read();
    let vertices = input.read_vec::<(isize, isize, isize)>(n).dec();

    let mut order = Vec::with_capacity(n);
    let mut rec = RecursiveFunction::new(|rec, vert: isize| {
        if vert == -2 {
            return;
        }
        let vert = vert as usize;
        let (l, r, val) = vertices[vert];
        rec.call(l);
        order.push(val);
        rec.call(r);
    });
    type Mod = ModIntF;
    fn part(left: isize, right: isize, qty: usize) -> Mod {
        if qty == 0 {
            return Mod::one();
        }
        let variants = (right - left + 1) as usize;
        combinations(variants + qty - 1, qty)
    }
    rec.call(0);
    let mut left = 1;
    let mut qty = 0;
    let mut ans = Mod::one();
    for i in order {
        if i == -1 {
            qty += 1;
        } else {
            ans *= part(left, i, qty);
            left = i;
            qty = 0;
        }
    }
    ans *= part(left, c, qty);
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
