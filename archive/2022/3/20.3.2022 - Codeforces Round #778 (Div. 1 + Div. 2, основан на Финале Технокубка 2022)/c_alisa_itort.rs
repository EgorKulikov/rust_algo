//{"name":"C. Алиса и торт","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"https://codeforces.com/contest/1654/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"14\n1\n327\n2\n869 541\n2\n985214736 985214737\n3\n2 3 1\n3\n2 3 3\n6\n1 1 1 1 1 1\n6\n100 100 100 100 100 100\n8\n100 100 100 100 100 100 100 100\n8\n2 16 1 8 64 1 4 32\n10\n1 2 4 7 1 1 1 1 7 2\n10\n7 1 1 1 3 1 3 3 2 3\n10\n1 4 4 1 1 1 3 3 3 1\n10\n2 3 2 2 1 2 2 2 2 2\n4\n999999999 999999999 999999999 999999999\n","output":"YES\nNO\nYES\nYES\nNO\nYES\nNO\nYES\nYES\nYES\nYES\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAlisaITort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_long_vec(n);

    let sum: i64 = a.iter().sum();
    a.sort_unstable();
    let mut heap = BinaryHeap::new();
    heap.push(sum);
    while let Some(w) = heap.pop() {
        let last = *a.last().unwrap();
        if last > w {
            out_line!("NO");
            return;
        }
        if last == w {
            a.pop();
        } else {
            heap.push(w / 2);
            heap.push((w + 1) / 2);
            if heap.len() > a.len() {
                out_line!("NO");
                return;
            }
        }
    }
    out_line!("YES");
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
