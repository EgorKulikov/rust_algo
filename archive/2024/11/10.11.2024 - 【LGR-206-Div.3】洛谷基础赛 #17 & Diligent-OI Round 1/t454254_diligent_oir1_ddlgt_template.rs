//{"name":"T454254 「Diligent-OI R1 D」DlgtTemplate","group":"Luogu","url":"https://www.luogu.com.cn/problem/T454254?contestId=127930","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 1 4 5 1 4\n1 0 0 2 1 1\n","output":"4\n1 2 3 4\n9\n"},{"input":"13\n-1 1 4 -5 -1 -4 1 9 -1 9 -8 -1 0\n1 0 2 1 3 0 0 2 0 0 2 0 1\n","output":"5\n1 2 7 8 10\n19\n"},{"input":"3\n-1 -1 0\n0 1 2\n","output":"0\n\n0\n"},{"input":"6\n1 1 4 5 1 4\n1 1 1 3 0 1\n","output":"2\n4 5\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T454254DiligentOIR1DDlgtTemplate"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_size_vec(n);

    let mut mem = Memoization2d::new(n + 1, n + 1, |mem, step, skips| -> (i64, bool) {
        if step == n {
            (0, false)
        } else {
            let mut res = (mem.call(step + 1, skips).0, false);
            if skips >= b[step] {
                res.maxim((a[step] + mem.call(step + 1, skips - b[step]).0, true));
            }
            res
        }
    });
    let mut ans = (0, n, 0);
    let mut has = 0usize;
    for i in 0..n {
        ans.maxim((a[i] + mem.call(i + 1, has.saturating_sub(b[i])).0, i, has));
        if b[i] == 0 || i == 0 {
            has += 1;
        }
    }
    if ans.0 == 0 {
        out.print_line(0);
        out.print_line(());
        out.print_line(0);
        return;
    }
    let (val, at, has) = ans;
    let mut skips = has.saturating_sub(b[at]);
    let mut included = vec![at];
    for i in at + 1..n {
        if mem.call(i, skips).1 {
            skips -= b[i];
            included.push(i);
        }
    }
    let mut need = has - skips;
    for i in 0..at {
        if need > 0 && (b[i] == 0 || i == 0) {
            included.push(i);
            need -= 1;
        }
    }
    included.sort();
    out.print_line(included.len());
    out.print_line(included.inc());
    out.print_line(val);
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
