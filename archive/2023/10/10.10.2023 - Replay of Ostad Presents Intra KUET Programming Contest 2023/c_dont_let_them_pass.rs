//{"name":"C. Don't Let Them Pass","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"7 5\n1 0 0 0 0\n0 0 1 0 0\n0 1 0 0 1\n0 0 0 0 0\n0 0 0 1 0\n0 0 0 0 0\n0 0 0 0 0\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDontLetThemPass"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::sign::Unsigned;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let grid = input.read_char_table(n, m);

    let row = (0..m)
        .map(|i| grid.column(i).find_eq(&&'1').unwrap())
        .collect_vec();
    let mut ans = Arr2d::new(n, m, 0);
    for i in 0..n {
        ans[(i, 0)] = i.distance(row[0]);
    }
    for i in 1..m {
        for j in 0..n {
            let mut left = ans[(j, i - 1)];
            if j > 0 {
                left = left.min(ans[(j - 1, i - 1)]);
            }
            if j + 1 < n {
                left = left.min(ans[(j + 1, i - 1)]);
            }
            ans[(j, i)] = left + j.distance(row[i]);
        }
    }
    out.print_line(ans.column(m - 1).min());
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
