//{"name":"Xoperations","group":"CodeChef - START168A","url":"https://www.codechef.com/START168A/problems/XOP","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n5 5 100\n2\n6 7\n4\n2 2 2 2\n","output":"4\n3\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Xoperations"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let b = input.read_long_vec(n);

    let mut dsu = DSU::new(n);
    let mut ans = 0;
    for i in (0..n).rev() {
        let mut j = 2;
        let mut k = 2;
        while i + j <= n {
            if (b[i + j - 2] >> k) == (b[i + j - 1] >> k) {
                dsu.union(i + j - 2, i + j - 1);
            }
            j *= 2;
            k += 1;
        }
        let mut left = i;
        let mut right = n - 1;
        while left < right {
            let mid = (left + right + 1) / 2;
            if dsu.find(i) == dsu.find(mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        ans += left - i + 1;
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
