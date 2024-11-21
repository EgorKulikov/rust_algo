//{"name":"Min-Max Game","group":"CodeChef - START161A","url":"https://www.codechef.com/START161A/problems/MINMAXSUB","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 2\n3\n2 1 3\n3\n1 3 2\n4\n1 4 3 2\n","output":"1\n3\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinMaxGame"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    if n % 2 == 1 {
        let mut ans = None;
        for i in 0..n {
            if p[i] >= i {
                ans.maxim(p[i] + 1 - i);
            }
            if p[i] >= n - i - 1 {
                ans.maxim(p[i] + 1 - (n - i - 1));
            }
        }
        out.print_line(ans);
    } else {
        let at = p.inv();
        let mut left = at[n - 1];
        let mut right = at[n - 1];
        let mut ans = 1;
        for i in (0..n - 1).rev() {
            left.minim(at[i]);
            right.maxim(at[i]);
            let cost = left + n - 1 - right;
            if i > cost {
                ans.maxim(i + 1 - cost);
            }
        }
        out.print_line(ans);
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
