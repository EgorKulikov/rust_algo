//{"name":"Candies","group":"CodeChef - START74A","url":"https://www.codechef.com/START74A/problems/CANDIES3","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 6\n3 1 4 1 5\n1 4 5 5 8 99\n1 2\n1\n4 1\n","output":"20\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Candies"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);
    let c = input.read_long_vec(m);

    let mut add = vec![0; m + 1];
    let q = a.qty_bound(m + 1);
    for (i, q) in q.into_iter().enumerate() {
        if q == 0 {
            continue;
        }
        let mut j = 1;
        while j <= i {
            let d = i / j;
            let r = i / d + 1;
            add[j - 1] += (d * q).into_i64();
            add[r - 1] -= (d * q).into_i64();
            j = r;
        }
    }
    let mut sum = 0;
    let mut ans = 0;
    for (a, c) in add.into_iter().zip(c.into_iter()) {
        sum += a;
        ans.maxim(sum * c);
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
