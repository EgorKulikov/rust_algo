//{"name":"D. Лексихроматография","group":"Codeforces - Codeforces Round 902 (Div. 1, based on COMPFEST 15 - Final Round)","url":"https://codeforces.com/contest/1876/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n1 3 1 2 3 2 3 3\n","output":"3\n"},{"input":"1\n265\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLeksikhromatografiya"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::slice_ext::compress::compress;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::powers;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let (c, [a]) = compress([&a]);
    let c = c.len();
    let mut dsu = DSU::new(c);

    type Mod = ModIntF;
    let mut ans = Mod::zero();
    let two = powers(Mod::new(2), n + 1);
    let mut blue = Vec::new();
    let mut red = Vec::new();
    let mut next_red = HashSet::new();
    let mut id = 0;

    for i in 0..n {
        if !next_red.contains(&a[i]) {
            if blue.len() == red.len() {
                id = blue.len();
            } else if dsu.get(a[i]) != dsu.get(blue[id]) {
                ans += two[dsu.set_count() - 2];
            }
            blue.push(a[i]);
            next_red.insert(a[i]);
            dsu.join(a[i], blue[id]);
        } else {
            if blue[red.len()] != a[i] {
                ans += two[dsu.set_count() - 1];
                break;
            }
            red.push(a[i]);
            next_red.remove(&a[i]);
        }
    }
    if blue != red && blue.len() + red.len() == n {
        ans += two[dsu.set_count() - 1];
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
            for i in 0usize..t {
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
    stress_test::stress_test(run, tester::check);
}
//END MAIN
