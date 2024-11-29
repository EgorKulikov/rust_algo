//{"name":"G. Interview with Mithu","group":"Toph","url":"https://toph.co/arena?contest=diu-take-off-fall-24-preliminary-mock#!/p/6745c410cb14b2a6cffe4a0c","interactive":false,"timeLimit":1000,"tests":[{"input":"7\naococob\n","output":"a***b\n"},{"input":"13\nococmcocococo\n","output":"***cmc***\n"},{"input":"9\nocoocooco\n","output":"*********\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GInterviewWithMithu"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut ans = Str::new();
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && (j % 2 == i % 2 && s[j] == b'o' || j % 2 != i % 2 && s[j] == b'c') {
            j += 1;
        }
        if j - i >= 3 {
            if j % 2 == i % 2 {
                j -= 1;
            }
            ans += b"***";
            i = j;
        } else {
            ans += s[i];
            i += 1;
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
