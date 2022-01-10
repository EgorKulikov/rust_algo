//{"name":"G. Дерево минимального ора","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n\n3 3\n1 2 1\n2 3 2\n1 3 2\n\n5 7\n4 2 7\n2 5 8\n3 4 2\n3 2 1\n2 4 2\n4 1 2\n1 2 2\n\n3 4\n1 2 1\n2 3 2\n1 3 3\n3 1 4\n","output":"2\n10\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GDerevoMinimalnogoOra"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges: Vec<(usize, usize, u32)> = input.read_vec(m);

    let mut ans = u32::MAX;
    let mut dsu = DSU::new(n);
    for i in (0..u32::BITS).rev() {
        dsu.clear();
        let cand = ans.without_bit(i.into_usize());
        for &(f, t, w) in &edges {
            if (w & cand) == w {
                dsu.join(f - 1, t - 1);
            }
        }
        if dsu.count() == 1 {
            ans = cand;
        }
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
