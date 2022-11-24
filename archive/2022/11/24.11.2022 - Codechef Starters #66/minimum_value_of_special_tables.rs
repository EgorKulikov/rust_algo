//{"name":"Minimum value of special tables","group":"CodeChef - START66A","url":"https://www.codechef.com/START66A/problems-old/MNVST","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 3\n4 5\n","output":"5\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinimumValueOfSpecialTables"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize, ans: &[usize]) {
    let n = input.read_usize();
    let m = input.read_usize();

    out_line!(ans[n.min(m)]);
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut ans = Vec::with_capacity(5001);
    ans.push(0);
    let mut sum = 0;
    let mut sum_sq = 0;
    for i in 1.. {
        sum += i;
        sum_sq += i * i;
        while ans.len() <= sum && ans.len() <= 5000 {
            let d = sum - ans.len();
            ans.push(sum_sq - d * d);
        }
        if ans.len() > 5000 {
            break;
        }
    }

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &ans),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &ans);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &ans);
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
