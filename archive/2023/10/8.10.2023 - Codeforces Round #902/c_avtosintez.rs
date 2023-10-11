//{"name":"C. Автосинтез","group":"Codeforces - Codeforces Round 902 (Div. 1, based on COMPFEST 15 - Final Round)","url":"https://codeforces.com/contest/1876/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4 2 2 3\n","output":"3\n3 2 3\n"},{"input":"3\n1 2 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAvtosintez"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut incoming = a.qty_bound(n);
    let mut queue = Vec::new();
    for (i, &j) in incoming.iter().enumerate() {
        if j == 0 {
            queue.push(i);
        }
    }
    let mut processed = BitSet::new(n);
    let mut in_ans = BitSet::new(n);
    while let Some(i) = queue.pop() {
        processed.set(i);
        in_ans.set(i);
        if !processed[a[i]] {
            processed.set(a[i]);
            incoming[a[a[i]]] -= 1;
            if incoming[a[a[i]]] == 0 {
                queue.push(a[a[i]]);
            }
        }
    }
    for i in 0..n {
        if processed[i] {
            continue;
        }
        let mut cur = i;
        let mut next = true;
        while !processed[cur] {
            processed.set(cur);
            in_ans.change(cur, next);
            next = !next;
            cur = a[cur];
        }
        if !next {
            out.print_line(-1);
            return;
        }
    }
    let ans = in_ans.iter().map(|i| a[i] + 1).collect_vec();
    out.print_line(ans.len());
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
