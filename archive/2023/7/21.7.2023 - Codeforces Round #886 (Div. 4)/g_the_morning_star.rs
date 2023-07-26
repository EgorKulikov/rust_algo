//{"name":"G. The Morning Star","group":"Codeforces - Codeforces Round 886 (Div. 4)","url":"https://codeforces.com/contest/1850/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n0 0\n-1 -1\n1 1\n4\n4 5\n5 7\n6 9\n10 13\n3\n-1000000000 1000000000\n0 0\n1000000000 -1000000000\n5\n0 0\n2 2\n-1 5\n-1 10\n2 11\n3\n0 0\n-1 2\n1 -2\n","output":"6\n2\n6\n8\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GTheMorningStar"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let p = input.read_int_pair_vec(n);

    let mut x = DefaultHashMap::<_, i64>::new();
    let mut y = DefaultHashMap::<_, i64>::new();
    let mut xy = DefaultHashMap::<_, i64>::new();
    let mut yx = DefaultHashMap::<_, i64>::new();

    for (xx, yy) in p {
        x[xx] += 1;
        y[yy] += 1;
        xy[xx + yy] += 1;
        yx[xx - yy] += 1;
    }

    let mut ans = 0;
    for map in [x, y, xy, yx] {
        for v in map.values() {
            ans += v * (v - 1);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
