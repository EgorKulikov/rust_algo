//{"name":"Sorting the Binary String","group":"CodeChef - START66A","url":"https://www.codechef.com/START66A/problems-old/BSSORT","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n0011\n6\n111000\n","output":"10\n17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SortingTheBinaryString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s = input.read_vec::<char>(n);

    let next = |c: char| -> Vec<usize> {
        let mut res = vec![n; n + 2];
        let mut cur = n;
        for i in (0..n).rev() {
            if s[i] == c {
                cur = i;
            }
            res[i] = cur;
        }
        res
    };
    let next_0 = next('0');
    let next_1 = next('1');

    let mut ans = 0;
    for i in 0..n {
        ans += next_0[next_0[next_1[next_1[i] + 1]] + 1] - i;
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
