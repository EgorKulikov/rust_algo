//{"name":"C. Перестановка столбцов","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 3\n1 2 3\n1 1 1\n2 2\n4 1\n2 3\n2 2\n2 1\n1 1\n2 3\n6 2 1\n5 4 3\n2 1\n1\n2\n","output":"1 1\n-1\n1 2\n1 3\n1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPerestanovkaStolbtsov"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut a = input.read_table::<i32>(n, m);

    let mut bad = BitSet::new(m);
    for i in 0..n {
        let mut r = a.row(i).cloned().collect_vec();
        r.sort_unstable();
        for (i, (&a, &b)) in r.iter().zip(a.row(i)).enumerate() {
            if a != b {
                bad.set(i, true);
            }
        }
    }
    let bad_cols = bad.iter().collect_vec();
    if bad_cols.len() > 2 {
        out_line!(-1);
    } else if bad_cols.len() == 2 {
        for i in 0..n {
            a.swap(i, bad_cols[0], i, bad_cols[1]);
            let mut r = a.row(i).cloned().collect_vec();
            r.sort_unstable();
            if r != a.row(i).cloned().collect_vec() {
                out_line!(-1);
                return;
            }
        }
        out_line!(bad_cols.inc_by_one());
    } else {
        out_line!(1, 1);
    }
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
