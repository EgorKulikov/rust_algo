//{"name":"D. Сделай равными","group":"Codeforces - Educational Codeforces Round 122 (Rated for Div. 2)","url":"https://codeforces.com/contest/1633/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 4\n1 7 5 2\n2 6 5 2\n3 0\n3 5 2\n5 4 7\n5 9\n5 2 5 6 3\n5 9 1 9 7\n6 14\n11 4 6 2 8 16\n43 45 9 41 15 38\n","output":"9\n0\n30\n167\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSdelaiRavnimi"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize, cost: &[usize]) {
    let n = input.read_usize();
    let k = input.read_usize();
    let b = input.read_usize_vec(n);
    let c = input.read_unsigned_vec(n);

    let mut ans = vec![0; k + 1];
    let mut sum = 0;
    for (b, c) in b.into_iter().zip(c.into_iter()) {
        let b = cost[b];
        sum += b;
        for i in (b..=sum.min(k)).rev() {
            let v = ans[i - b];
            ans[i].maxim(v + c);
        }
    }
    out_line!(ans.into_iter().max());
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut cost = vec![None; 1001];
    cost[0] = Some(0usize);
    cost[1] = Some(0usize);
    for i in 1..=1000 {
        let c = cost[i].unwrap();
        for j in 1..=i {
            if i + i / j <= 1000 {
                cost[i + i / j].minim(c + 1);
            }
        }
    }
    let cost = cost.into_iter().map(|i| i.unwrap()).collect_vec();
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &cost),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &cost);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &cost);
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
