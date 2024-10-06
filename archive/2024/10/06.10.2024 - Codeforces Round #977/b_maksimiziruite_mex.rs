//{"name":"B. Максимизируйте MEX","group":"Codeforces - Codeforces Round 977 (Div. 2, на основе COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 3\n0 3 2 1 5 2\n6 2\n1 3 4 1 0 2\n4 5\n2 5 10 3\n","output":"4\n6\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMaksimiziruiteMEX"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let a = input.read_long_vec(n);

    let mut by_mod = DefaultHashMap::<_, Vec<_>>::new();
    for &ai in &a {
        by_mod[ai % x].push(ai);
    }
    let mut ans = None;
    for i in 0..x {
        if by_mod[i].is_empty() {
            ans.minim(i);
            break;
        }
        by_mod[i].sort();
        let mut cur = i;
        for &a in &by_mod[i] {
            if a > cur {
                break;
            }
            cur += x;
        }
        ans.minim(cur);
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
