//{"name":"F1. Разделение поля (простая версия)","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/F1","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n2 2 3\n1 1\n1 2\n2 2\n5 5 4\n1 2\n2 2\n3 4\n4 3\n2 5 9\n1 2\n1 5\n1 1\n2 2\n2 4\n2 5\n1 4\n2 3\n1 3\n6 4 4\n6 2\n1 3\n1 4\n1 2\n3 4 5\n2 1\n3 2\n1 4\n1 3\n2 4\n","output":"1\n1 0 1\n11\n0 1 0 1\n1\n0 0 1 1 0 0 0 0 0\n6\n1 0 0 0\n1\n1 1 0 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1RazdeleniePolyaProstayaVersiya"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let marked = input.read_size_pair_vec(k);

    let mut order = (0..k).collect_vec();
    order.sort_by_key(|&i| (marked[i].1, Reverse(marked[i].0)));

    let mut last_row = 0;
    let mut ans = n * m;
    let mut a = vec![0; k];
    for i in order {
        let (row, col) = marked[i];
        if row > last_row {
            ans -= (row - last_row) * (m + 1 - col);
            a[i] = 1;
            last_row = row;
        }
    }
    out.print_line(ans);
    out.print_line(a);
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
}
//END MAIN
