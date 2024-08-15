//{"name":"E. Фотосессия для горилл","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4 2\n9\n1 1 1 1 1 1 1 1 1\n2 1 1\n2\n5 7\n20 15 7\n9\n4 1 4 5 6 1 1000000000 898 777\n1984 1 1\n4\n5 4 1499 2004\n9 5 5\n6\n6 7 14 16 16 6\n","output":"21\n12\n49000083104\n3512\n319\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFotosessiyaDlyaGorill"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let w = input.read_size();
    let mut a = input.read_size_vec(w);

    let b = Arr2d::generate(n, m, |i, j| {
        let from_i = i.max(k - 1) - (k - 1);
        let to_i = i.min(n - k) + 1;
        let from_j = j.max(k - 1) - (k - 1);
        let to_j = j.min(m - k) + 1;
        (to_i - from_i) * (to_j - from_j)
    });
    let mut c = b.into_iter().collect_vec();
    c.sort();
    c.reverse();
    a.sort();
    a.reverse();
    let mut ans = 0;
    for (i, j) in a.into_iter().zip(c) {
        ans += i * j;
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
