//{"name":"Fill the Blanks","group":"CodeChef - START164A","url":"https://www.codechef.com/START164A/problems/FILLBLANKS","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n1?\n2\n??\n","output":"1.00 4.50\n5.75 3.25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FillTheBlanks"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{IntoReal, Real};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let questions = s.iter().count_eq(&b'?');
    let mut ans = Vec::new();
    for i in 1..=questions {
        let mut next = vec![Real(0.); i];
        let mut pos = Vec::new();
        for d in 0..10 {
            for j in 0..i {
                if j == ans.len() || d.into_real() > ans[j] {
                    pos.push(j);
                    break;
                }
            }
        }
        let mut cur = 10.;
        let mut prev = 0.;
        for j in 0..i {
            let mut delta = 0.;
            for k in 0..10 {
                if pos[k] == j {
                    cur -= 1.;
                    delta += 1.;
                    next[j] += k.into_real() / 10.;
                }
            }
            if j != 0 {
                next[j] += prev * ans[j - 1] / 10.;
            }
            if j != ans.len() {
                next[j] += cur * ans[j] / 10.;
            }
            prev += delta;
        }
        ans = next;
    }
    let mut next = 0;
    let mut res = Vec::with_capacity(n);
    for i in s.indices() {
        if s[i] == b'?' {
            res.push(ans[next]);
            next += 1;
        } else {
            res.push((s[i] as usize - b'0' as usize).into_real());
        }
    }
    out.print_line(res);
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
