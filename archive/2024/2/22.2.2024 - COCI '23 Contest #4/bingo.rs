//{"name":"#1 - Bingo","group":"DMOJ - COCI '23 Contest 4","url":"https://dmoj.ca/problem/coci23c4p1","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nbabylasagna\n10 11 12 13 14\n15 16 17 18 19\n20 21 22 23 24\n25 26 27 28 29\n30 31 32 33 34\nnataliebalmix\n10 20 30 40 50\n11 21 31 41 51\n12 22 32 42 52\n13 23 33 43 53\n14 24 34 44 54\nlettri\n89 88 87 86 10\n85 84 83 11 82\n81 80 12 79 78\n77 13 76 75 74\n14 73 72 71 70\n6\n10 11 12 13 14 15\n","output":"3\nbabylasagna\nnataliebalmix\nlettri\n"},{"input":"1\nhoni\n1 2 3 4 5\n6 7 8 9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25\n4\n1 2 49 50\n","output":"0\n"},{"input":"4\nrim\n15 23 14 26 34\n12 11 13 16 17\n90 67 45 24 18\n85 82 77 66 22\n62 71 32 35 7\ntim\n61 89 25 63 12\n29 30 31 32 33\n11 17 42 24 18\n88 82 77 66 22\n44 71 54 35 7\ndagi\n15 23 14 26 34\n12 11 13 16 17\n90 67 45 24 18\n85 82 77 66 22\n62 71 36 35 7\ndim\n15 23 14 26 34\n12 11 13 16 17\n90 67 45 24 18\n85 82 77 66 22\n42 51 32 33 7\n7\n15 11 66 7 42 30 61\n","output":"1\ntim\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Bingo"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut cards = Vec::with_capacity(n);
    for _ in 0..n {
        let name = input.read_str();
        let card = input.read_size_table(5, 5);
        cards.push((name, card));
    }
    let m = input.read_size();
    let c = input.read_size_vec(m);

    let mut ans = Vec::new();
    for (name, card) in cards {
        let mut rows = vec![0; 5];
        let mut cols = vec![0; 5];
        let mut diag1 = 0;
        let mut diag2 = 0;
        for &c in &c {
            for i in 0..5 {
                for j in 0..5 {
                    if card[(i, j)] == c {
                        rows[i] += 1;
                        cols[j] += 1;
                        if i == j {
                            diag1 += 1;
                        }
                        if i + j == 4 {
                            diag2 += 1;
                        }
                    }
                }
            }
        }
        if rows.contains(&5) || cols.contains(&5) || diag1 == 5 || diag2 == 5 {
            ans.push(name);
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
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
