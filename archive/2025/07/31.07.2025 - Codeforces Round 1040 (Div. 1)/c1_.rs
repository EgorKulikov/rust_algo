//{"name":"C1. Интерактивная ПСП (простая версия)","group":"Codeforces - Codeforces Round 1040 (Div. 1)","url":"https://codeforces.com/contest/2129/problem/C1","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n3\n\n0\n\n1\n\n1\n\n2\n\n3\n","output":"\n\n? 4 1 2 3 3\n\n? 2 2 1\n\n? 2 3 1\n\n! )((\n\n? 4 1 2 1 2\n\n! ()\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut dsu = DSU::new(n);
    let mut seq = Vec::new();
    let mut sum = 0;
    for i in 1.. {
        let cur = i * (i + 1) / 2;
        if cur <= sum {
            continue;
        }
        seq.push(i);
        sum += cur;
        if seq.len() == 11 {
            break;
        }
    }
    for i in (1..n).step_by(11) {
        let mut query = Vec::new();
        for j in 0..11.min(n - i) {
            for _ in 0..seq[j] {
                query.push(1);
                query.push(i + j + 1);
            }
            query.push(1);
        }
        out.print_line(('?', query.len(), &query));
        out.flush();
        let mut res = input.read_int();
        for j in (0..11.min(n - i)).rev() {
            let qty = seq[j] * (seq[j] + 1) / 2;
            if res < qty {
                dsu.union(0, i + j);
            } else {
                res -= qty;
            }
        }
    }
    let mut open = true;
    for i in 1..n {
        if dsu.find(i) != dsu.find(0) {
            out.print_line(('?', 2, 1, i + 1));
            out.flush();
            let res = input.read_int();
            if res == 0 {
                open = false;
            }
            break;
        }
    }
    let mut ans = Str::new();
    for i in 0..n {
        if (dsu.find(i) == dsu.find(0)) ^ open {
            ans.push(b')');
        } else {
            ans.push(b'(');
        }
    }
    out.print_line(('!', ans));
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
