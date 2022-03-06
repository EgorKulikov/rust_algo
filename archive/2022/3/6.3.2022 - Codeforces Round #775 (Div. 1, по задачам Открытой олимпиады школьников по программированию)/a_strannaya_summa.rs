//{"name":"A. Странная сумма","group":"Codeforces - Codeforces Round #775 (Div. 1, по задачам Открытой олимпиады школьников по программированию)","url":"https://codeforces.com/contest/1648/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3\n1 2 3\n3 2 1\n","output":"7\n"},{"input":"3 4\n1 1 2 2\n2 1 1 2\n2 2 1 1\n","output":"76\n"},{"input":"4 4\n1 1 2 3\n2 1 1 2\n3 1 2 1\n1 1 2 1\n","output":"129\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AStrannayaSumma"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::fast_clear_fenwick::FastClearFenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let c = input.read_table::<usize>(n, m);

    let mut pos: DefaultMap<usize, Vec<(i64, i64)>> = DefaultMap::new();
    for i in 0..n {
        for j in 0..m {
            pos[c[(i, j)]].push((i.into_i64(), j.into_i64()));
        }
    }
    let mut qty = FastClearFenwickTree::new(m);
    let mut sum = FastClearFenwickTree::new(m);
    let mut ans = 0;
    for p in pos.into_values() {
        for &(r, c) in &p {
            let cq = qty.get(0, c.into_usize());
            ans += cq * (r + c) - sum.get(0, c.into_usize());
            qty.add(c.into_usize(), 1);
            sum.add(c.into_usize(), r + c);
        }
        qty.clear();
        sum.clear();
        for (r, c) in p {
            let cq = qty.get(c.into_usize(), m);
            ans += cq * (r - c) - sum.get(c.into_usize(), m);
            qty.add(c.into_usize(), 1);
            sum.add(c.into_usize(), r - c);
        }
        qty.clear();
        sum.clear();
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
