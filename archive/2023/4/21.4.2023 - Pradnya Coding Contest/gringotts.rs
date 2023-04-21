//{"name":"Gringotts!","group":"CodeChef - PRAD2023","url":"https://www.codechef.com/PRAD2023/problems/GRINGOTTS","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n7\n33 63 50 49 34 52 47\n5\n17 24 18 30 29\n","output":"63\n17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Gringotts"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n);

    let target = u32::all_bits(32 - (a[0].leading_zeros()).into_usize());
    let mut ans = None;
    let mut w = None;
    for i in a {
        let mut j = i;
        let mut s = 0;
        while j != target {
            j ^= j >> 1;
            s += 1;
            if j == i {
                break;
            }
        }
        if j == target && w.minim(s) {
            ans = Some(i);
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
