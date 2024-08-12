//{"name":"Back Front","group":"CodeChef - START146A","url":"https://www.codechef.com/START146A/problems/BACKFRONT","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n15\nefmbagrowcnukpt\n11\nbxabcacfkck\n","output":"8\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BackFront"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut t = s.clone();
    t.reverse();
    let mut ans = n;
    for (s, seq) in [(s, b"front".to_vec()), (t, b"kcab".to_vec())] {
        let mut qty = vec![0; seq.len() + 1];
        qty[0] = n;
        for c in s {
            for j in seq.indices() {
                if c == seq[j] && qty[j] > 0 {
                    qty[j] -= 1;
                    qty[j + 1] += 1;
                }
            }
        }
        let mut free = 0;
        for i in qty.indices().rev() {
            for _ in 0..qty[i] {
                if free >= seq.len() - i {
                    free -= seq.len() - i;
                    free += 1;
                    ans -= seq.len() - 1;
                }
            }
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
