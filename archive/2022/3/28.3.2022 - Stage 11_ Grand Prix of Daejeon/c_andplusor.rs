//{"name":"C. AND PLUS OR","group":"Yandex - Stage 11: Grand Prix of Daejeon","url":"https://official.contest.yandex.com/opencupXXII/contest/35265/problems/C/","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n0 1 1 2\n","output":"-1\n"},{"input":"2\n0 1 1 3\n","output":"2 1\n"},{"input":"0\n100\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CANDPLUSOR"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(1 << n);

    for i in 0..a.len() {
        for j in 0..n {
            if i.is_set(j) {
                continue;
            }
            for k in j + 1..n {
                if i.is_set(k) {
                    continue;
                }
                if a[i.with_bit(j).with_bit(k)] + a[i] > a[i.with_bit(j)] + a[i.with_bit(k)] {
                    out_line!(i.with_bit(j), i.with_bit(k));
                    return;
                }
            }
        }
    }
    out_line!(-1);
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
