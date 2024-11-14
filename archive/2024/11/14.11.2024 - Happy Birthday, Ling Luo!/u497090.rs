//{"name":"U497090 世界沉睡童话","group":"Luogu","url":"https://www.luogu.com.cn/problem/U497090?contestId=209328","interactive":false,"timeLimit":1000,"tests":[{"input":"6 8\n","output":"3 6 1 4 2 5\n"},{"input":"3 3\n","output":"5 5 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"U497090"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut k = input.read_size();

    let mut ans = Vec::with_capacity(n);
    let mut cur = n - 1;
    let mut qty = 0;
    let mut forbidden = BitSet::new(2 * n);
    let mut big = n;
    for i in 0..n {
        if k >= n - i - 1 && qty == 0 && big == n {
            ans.push(1);
            k -= n - i - 1;
            continue;
        }
        if k > 0 {
            if qty == 0 {
                forbidden.set(2 * (n - 1));
            }
            if k >= qty {
                ans.push(cur);
                k -= qty;
                qty += 1;
            } else {
                cur -= 1;
                ans.push(cur);
                qty = 1;
                for j in (cur..2 * n).step_by(cur) {
                    forbidden.set(j);
                }
            }
        } else {
            while forbidden[big] {
                big += 1;
            }
            ans.push(big);
            big += 1;
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
