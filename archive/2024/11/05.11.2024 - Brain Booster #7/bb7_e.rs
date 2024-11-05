//{"name":"bb7_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"bb7_e"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    let qs = (0..n).filter(|i| s[*i] == b'?').collect_vec();
    let k = k.min(qs.len());
    let mut na = vec![0];
    let mut nb = vec![0];
    for i in 0..n {
        na.push(na[i] + (s[i] == b'A') as usize);
        nb.push(nb[i] + (s[i] == b'B') as usize);
    }
    let mut cur = 0;
    for i in 0..n {
        if s[i] == b'A' {
            cur += nb[n] - nb[i];
        }
    }
    for &i in qs.iter().rev().take(k) {
        cur += na[i];
    }
    let mut ans = cur;
    for i in 0..k {
        cur += nb[n] - nb[qs[i]];
        cur -= na[qs[Back(k - i - 1)]];
        ans.maxim(cur + (i + 1) * (k - i - 1));
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
