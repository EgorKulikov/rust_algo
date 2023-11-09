//{"name":"D. Разноцветный конструктив","group":"Codeforces - Codeforces Round 908 (Div. 1)","url":"https://codeforces.com/contest/1893/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n10 3\n1 1 1 1 2 2 2 3 3 4\n6 2 2\n4 1 1\n8 2\n7 7 7 7 8 8 8 8\n4 4\n2 2\n5 1\n5 4 3 2 1\n5\n3\n7 3\n1 2 2 2 2 3 4\n1 2 4\n1 2 4\n12 7\n6 6 6 6 6 6 6 6 6 7 8 9\n2 2 2 2 2 1 1\n1 2 2 2 1 1 1\n20 2\n11 20 15 6 8 18 12 16 8 20 10 12 3 12 20 11 15 8 17 17\n8 12\n3 5\n","output":"1 3 4 2 1 3\n1 1\n2 2\n8 7 8 7\n8 7 8 7\n2 4 5 3 1\n-1\n6 6\n7 6\n8 6\n9 6\n6 6\n6\n6\n12 17 20 15 8 20 16 11\n15 20 17 12 10 8 3 18 12 11 8 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRaznotsvetniiKonstruktiv"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n).dec();
    let s = input.read_size_vec(m);
    let d = input.read_size_vec(m);

    let mut order = (0..m).collect_vec();
    order.sort_by_key(|&i| Reverse(d[i]));
    let mut qty = a.qty_bound(n);
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        if qty[i] != 0 {
            heap.push((qty[i], i));
        }
    }
    let mut ans = vec![Vec::new(); m];
    for i in order {
        for x in 0..s[i] {
            if x >= d[i] && qty[ans[i][x - d[i]]] > 0 {
                heap.push((qty[ans[i][x - d[i]]], ans[i][x - d[i]]));
            }
            if heap.is_empty() {
                out.print_line(-1);
                return;
            }
            let (_, j) = heap.pop().unwrap();
            qty[j] -= 1;
            ans[i].push(j);
        }
        for x in s[i] - d[i]..s[i] {
            if qty[ans[i][x]] > 0 {
                heap.push((qty[ans[i][x]], ans[i][x]));
            }
        }
        for x in &mut ans[i] {
            *x += 1;
        }
    }
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
