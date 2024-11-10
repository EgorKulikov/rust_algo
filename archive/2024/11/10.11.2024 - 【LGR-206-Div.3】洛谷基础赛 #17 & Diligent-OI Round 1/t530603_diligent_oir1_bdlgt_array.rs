//{"name":"T530603 「Diligent-OI R1 B」DlgtArray","group":"Luogu","url":"https://www.luogu.com.cn/problem/T530603?contestId=127930","interactive":false,"timeLimit":1000,"tests":[{"input":"5 4\n0 0 1 0 1\n1 3 2\n2 4 0\n3 5 2\n1 1 1\n","output":"1\n1\n0\n-1\n"},{"input":"8 3\n1 1 1 1 1 1 1 1\n2 6 2\n5 8 2\n1 8 7\n","output":"3\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T530603DiligentOIR1BDlgtArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_int_vec(n);

    let s = a.partial_sums();
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let k = input.read_int();
        if k > (r - l - 1) as i32 {
            out.print_line(-1);
            continue;
        }
        let q = s[r] - s[l];
        if k == (r - l - 1) as i32 && q >= (r - l - 1) as i32 {
            out.print_line(0);
            continue;
        }
        out.print_line((q - k).abs());
    }
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
