//{"name":"C. DIY","group":"Codeforces - 2024-2025 ICPC, NERC, Southern and Volga Russian Regional Contest (Unrated, Online Mirror, ICPC Rules, Preferably Teams)","url":"https://codeforces.com/contest/2038/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n16\n-5 1 1 2 2 3 3 4 4 5 5 6 6 7 7 10\n8\n0 0 -1 2 2 1 1 3\n8\n0 0 0 0 0 5 0 5\n","output":"YES\n1 2 1 7 6 2 6 7\nNO\nYES\n0 0 0 5 0 0 0 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDIY"}}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n).sorted();

    let mut opts = Vec::new();
    let mut i = 0;
    while i + 1 < n {
        if a[i] == a[i + 1] {
            opts.push(a[i]);
            i += 2;
        } else {
            i += 1;
        }
    }
    if opts.len() < 4 {
        out.print_line(false);
    } else {
        out.print_line(true);
        out.print_line((
            opts[0],
            opts[1],
            opts[0],
            opts[Back(0)],
            opts[Back(1)],
            opts[1],
            opts[Back(1)],
            opts[Back(0)],
        ));
    }
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
