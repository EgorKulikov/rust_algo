//{"name":"I. Semi-Palindromic Tree","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/I","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 2\n2 3\n3 4\n4 5\n","output":"abbba\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ISemiPalindromicTree"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let (u, v) = edges.detuple();
    let qty = u
        .into_iter()
        .chain(v.into_iter())
        .collect_vec()
        .qty_bound(n);
    let (leaf, non_leaf) = if qty[0] == 1 {
        (b'a', b'b')
    } else {
        (b'b', b'a')
    };
    let mut s = Str::with_capacity(n);
    for i in qty {
        if i == 1 {
            s.push(leaf);
        } else {
            s.push(non_leaf);
        }
    }
    out.print_line(s);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
