//{"name":"Ball Sequence","group":"CodeChef - START55A","url":"https://www.codechef.com/START55A/problems-old/BALLSEQ","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5 2\n3 4\n1 1\n1\n1000000000 5\n271832455 357062697 396505195 580082912 736850926\n","output":"2\n0\n999999980\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BallSequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let mut a = input.read_usize_vec(k);

    a.push(n + 1);
    let mut cur = 0;
    let mut lens = Vec::with_capacity(k + 1);
    for i in a {
        lens.push(i - cur - 1);
        cur = i;
    }
    let mut val = 0;
    let mut ans = 0;
    for i in lens {
        ans += val & 1;
        val >>= 1;
        let will_be = val + i;
        for j in 0..30 {
            ans += (will_be >> (j + 1)) - (val >> (j + 1));
        }
        val = will_be;
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
