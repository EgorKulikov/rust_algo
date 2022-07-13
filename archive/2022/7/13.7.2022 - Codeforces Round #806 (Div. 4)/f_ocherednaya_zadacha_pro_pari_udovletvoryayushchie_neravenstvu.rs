//{"name":"F.  Очередная задача про пары, удовлетворяющие неравенству","group":"Codeforces - Codeforces Round #806 (Div. 4)","url":"https://codeforces.com/contest/1703/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n8\n1 1 2 3 8 2 1 4\n2\n1 2\n10\n0 2 1 6 3 4 1 2 8 3\n2\n1 1000000000\n3\n0 1000000000 2\n","output":"3\n0\n10\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FOcherednayaZadachaProPariUdovletvoryayushchieNeravenstvu"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut ft = FenwickTree::new(n);
    let mut ans = 0i64;
    for i in 0..n {
        if a[i] <= i {
            if a[i] > 0 {
                ans += ft.get(0, a[i] - 1);
            }
            ft.add(i, 1);
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
