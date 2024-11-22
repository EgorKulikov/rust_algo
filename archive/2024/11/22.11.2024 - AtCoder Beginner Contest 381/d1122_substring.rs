//{"name":"D - 1122 Substring","group":"AtCoder - AtCoder Beginner Contest 381","url":"https://atcoder.jp/contests/abc381/tasks/abc381_d","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n2 3 1 1 2 2 1 1\n","output":"4\n"},{"input":"3\n1 2 2\n","output":"2\n"},{"input":"1\n1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1122Substring"}}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    fn check(a: &[usize]) -> usize {
        let mut res = 0;
        let mut last = FxHashMap::default();
        let mut start = 0;
        for i in a.indices() {
            if let Some(last) = last.get(&a[i]) {
                start.maxim(*last + 1);
            }
            res.maxim(i + 1 - start);
            last.insert(a[i], i);
        }
        res
    }

    let mut ans = 0;
    let mut cur = Vec::new();
    for i in (0..n - 1).step_by(2) {
        if a[i] != a[i + 1] {
            ans.maxim(check(&cur));
            cur.clear();
        } else {
            cur.push(a[i]);
        }
    }
    ans.maxim(check(&cur));
    let mut cur = Vec::new();
    for i in (1..n - 1).step_by(2) {
        if a[i] != a[i + 1] {
            ans.maxim(check(&cur));
            cur.clear();
        } else {
            cur.push(a[i]);
        }
    }
    ans.maxim(check(&cur));
    out.print_line(ans * 2);
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
