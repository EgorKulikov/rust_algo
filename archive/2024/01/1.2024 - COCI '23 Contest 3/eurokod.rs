//{"name":"#1 - Eurokod","group":"DMOJ - COCI '23 Contest 3","url":"https://dmoj.ca/problem/coci23c3p1","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2 3\n50 10 20\n","output":"1. Kod01 (6)\n2. Kod03 (3)\n3. Kod02 (3)\n"},{"input":"5\n5 2 4 1 3\n4 5 2 1 3\n","output":"1. Kod02 (9)\n2. Kod05 (8)\n3. Kod01 (6)\n4. Kod04 (4)\n5. Kod03 (3)\n"},{"input":"7\n6 3 2 1 5 4 7\n200 56 11 0 13 105 12\n","output":"1. Kod06 (13)\n2. Kod01 (11)\n3. Kod02 (10)\n4. Kod03 (8)\n5. Kod05 (7)\n6. Kod07 (4)\n7. Kod04 (3)\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Eurokod"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let place = input.read_size_vec(n).dec();
    let points = input.read_size_vec(n);

    let place = Permutation::new(place).inv().to_vec();
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| Reverse(points[i]));
    let mut place_points = vec![0; n];
    for (i, j) in order.into_iter().enumerate() {
        place_points[j] = i;
    }
    let all_place = place.into_iter().zip(place_points).collect_vec();
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| (all_place[i].0 + all_place[i].1, all_place[i].1));
    for (j, i) in order.into_iter().enumerate() {
        out.print_line(format!(
            "{}. Kod{:02} ({})",
            j + 1,
            i + 1,
            2 * n - (all_place[i].0 + all_place[i].1)
        ));
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
    //    tester::stress_test();
}
//END MAIN
