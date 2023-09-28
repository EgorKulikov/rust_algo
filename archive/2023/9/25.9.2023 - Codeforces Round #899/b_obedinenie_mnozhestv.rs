//{"name":"B. Объединение множеств","group":"Codeforces - Codeforces Round 899 (Div. 2)","url":"https://codeforces.com/contest/1882/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n3 1 2 3\n2 4 5\n2 3 4\n4\n4 1 2 3 4\n3 2 5 6\n3 3 5 6\n3 4 5 6\n5\n1 1\n3 3 6 10\n1 9\n2 1 3\n3 5 8 9\n1\n2 4 28\n","output":"4\n5\n6\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BObedinenieMnozhestv"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut s = Vec::with_capacity(n);
    for _ in 0..n {
        let mut cur = 0u64;
        let k = input.read_size();
        for _ in 0..k {
            let x = input.read_size();
            cur.set_bit(x - 1);
        }
        s.push(cur);
    }
    let x = s.iter().fold(0u64, |acc, &x| acc | x);
    let mut ans = 0;
    for i in 0..60 {
        if !x.is_set(i) {
            continue;
        }
        let mut cur = 0;
        for &j in &s {
            if !j.is_set(i) {
                cur |= j;
            }
        }
        ans.maxim(cur.count_ones());
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
                i += 1;
            }
        }
    }
    output().flush();
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
