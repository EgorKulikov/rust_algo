//{"name":"#2 - Labirint","group":"DMOJ - COCI '23 Contest 1","url":"https://dmoj.ca/problem/coci23c1p2","interactive":false,"timeLimit":1000,"tests":[{"input":"1 8\nCPZNCCP\n4\n1 1 1 8\n1 3 1 5\n1 8 1 4\n1 2 1 3\n","output":"4\n2\n3\n1\n"},{"input":"3 3\nPP\nPP\nPP\nCCC\nCCC\n3\n1 1 3 3\n3 3 2 2\n1 1 1 3\n","output":"2\n2\n1\n"},{"input":"4 4\nCCC\nCPC\nPPP\nCNP\nZZZZ\nPPPP\nCPZC\n4\n3 1 2 3\n1 1 4 4\n2 2 3 3\n1 4 4 1\n","output":"1\n2\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Labirint"}}}

use algo_lib::collections::dsu2d::DSU2d;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let horizontal = input.read_char_table(n, m - 1);
    let vertical = input.read_char_table(n - 1, m);

    fn to_id(c: char) -> usize {
        match c {
            'C' => 0,
            'P' => 1,
            'Z' => 2,
            'N' => 3,
            _ => unreachable!(),
        }
    }
    let map = Arr2d::generate(2 * n - 1, 2 * m - 1, |i, j| {
        if i % 2 == 0 && j % 2 == 1 {
            to_id(horizontal[(i / 2, j / 2)])
        } else if i % 2 == 1 && j % 2 == 0 {
            to_id(vertical[(i / 2, j / 2)])
        } else {
            4
        }
    });
    let mut dsus = Vec::with_capacity(16);
    for i in 0..16 {
        let mut dsu = DSU2d::new(n, m);
        for r in 0..n {
            for c in 0..m {
                for (nr, nc) in D4::iter(r, c, n, m) {
                    if i.is_set(map[(r + nr, c + nc)]) {
                        dsu.join(r, c, nr, nc);
                    }
                }
            }
        }
        dsus.push(dsu);
    }

    let q = input.read_size();
    for _ in 0..q {
        let r1 = input.read_size() - 1;
        let c1 = input.read_size() - 1;
        let r2 = input.read_size() - 1;
        let c2 = input.read_size() - 1;
        let mut ans = None;
        for (i, dsu) in dsus.iter().enumerate() {
            if dsu.get(r1, c1) == dsu.get(r2, c2) {
                ans.minim(i.count_ones());
            }
        }
        out.print_line(ans);
    }
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
