//{"name":"D. Я хочу пить, босс","group":"Codeforces - Codeforces Round 977 (Div. 2, на основе COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n3 6\n79 20 49 5 -1000 500\n-105 9 109 24 -98 -499\n14 47 12 39 23 50\n","output":"475\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYaKhochuPitBoss"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_table(n, m);

    let mut prev = vec![i64::MIN; m + 1];
    let mut min = 0;
    let mut sum = 0;
    for i in 0..m {
        sum += a[(0, i)];
        prev[i + 1].maxim(sum - min);
        min.minim(sum);
    }
    min = 0;
    sum = 0;
    for i in (0..m).rev() {
        sum += a[(0, i)];
        prev[i].maxim(sum - min);
        min.minim(sum);
    }
    for j in 1..n {
        let mut next = vec![i64::MIN; m + 1];
        let mut min = 0;
        let mut sp_min = i64::MAX / 2;
        let mut sum = 0;
        for i in 0..m {
            sum += a[(j, i)];
            next[i + 1].maxim(sum - sp_min);
            sp_min.minim(min - prev[i + 1]);
            min.minim(sum);
        }
        min = 0;
        sp_min = i64::MAX / 2;
        sum = 0;
        for i in (0..m).rev() {
            sum += a[(j, i)];
            next[i].maxim(sum - sp_min);
            sp_min.minim(min - prev[i]);
            min.minim(sum);
        }
        prev = next;
    }
    out.print_line(prev.iter().max().unwrap());
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
