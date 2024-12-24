//{"name":"C. Sums on Segments","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 -1 10 1 1\n5\n-1 -1 -1 -1 -1\n2\n-1 2\n2\n7 1\n3\n1 4 -1\n","output":"8\n-1 0 1 2 9 10 11 12\n6\n-5 -4 -3 -2 -1 0\n4\n-1 0 1 2\n4\n0 1 7 8\n6\n-1 0 1 3 4 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSumsOnSegments"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let special = a.copy_position(|x| x.abs() != 1);
    let mut ans = Vec::new();
    let mut process = |from: usize, to: usize| {
        let mut min = 0;
        let mut max = 0;
        let mut cur = 0;
        let mut r1 = 0;
        let mut r2 = 0;
        for i in from..to {
            cur += a[i];
            min.minim(cur);
            max.maxim(cur);
            r1.maxim(cur - min);
            r2.minim(cur - max);
        }
        for r in r2..=r1 {
            ans.push(r);
        }
    };
    if let Some(mid) = special {
        process(0, mid);
        process(mid + 1, n);
        let mut cur = 0;
        let mut min1 = 0;
        let mut max1 = 0;
        for i in (0..mid).rev() {
            cur += a[i];
            min1.minim(cur);
            max1.maxim(cur);
        }
        cur = 0;
        let mut min2 = 0;
        let mut max2 = 0;
        for i in mid + 1..n {
            cur += a[i];
            min2.minim(cur);
            max2.maxim(cur);
        }
        for i in a[mid] + min1 + min2..=a[mid] + max1 + max2 {
            ans.push(i);
        }
    } else {
        process(0, n);
    }
    ans.sort_unstable();
    ans.dedup();
    out.print_line(ans.len());
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
