//{"name":"Xometry (Hard Version)","group":"CodeChef - START153A","url":"https://www.codechef.com/START153A/problems/XSQRH","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n2 4 2 4\n8\n2 4 3 4 2 5 3 1\n","output":"8\n216\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"XometryHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut qty = vec![0usize; 1 << 20];
    let mut base_qty = vec![0usize; 1000001];
    let mut ans = 0;
    for i in 0..n {
        base_qty[a[i]] += 1;
        for j in i + 1..n {
            ans += qty[a[i] ^ a[j]] * 24;
        }
        for j in 0..i {
            qty[a[i] ^ a[j]] += 1;
        }
    }
    let mut total = 0;
    for i in base_qty {
        if i >= 2 {
            if i >= 4 {
                ans -= i * (i - 1) * (i - 2) * (i - 3);
            }
            ans -= 4 * total * i * (i - 1);
            total += i * (i - 1);
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
