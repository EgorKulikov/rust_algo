//{"name":"I. Identify Chord","group":"Universal Cup - The 3rd Universal Cup. Stage 25: Hangzhou","url":"https://contest.ucup.ac/contest/1893/problem/9734","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 2 4\n4 1 3\n","output":"4\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IIdentifyChord"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut query = |a: usize, b: usize| -> usize {
        output!(out, "? {} {}", a % n + 1, b % n + 1);
        out.flush();
        input.read_size()
    };
    let mut base = 0;
    let base_res = loop {
        let res = query(base, base + n / 2);
        if res != n / 2 {
            break res;
        }
        base = (base + n / 2 - 1) % n;
    };
    let end = base + n / 2;
    let (one_end, dist_to_end) = if query(base + 1, end) < base_res {
        let mut left = base + 1;
        let mut right = end - 1;
        while left < right {
            let mid = (left + right + 1) / 2;
            let res = query(mid, end);
            if res + (mid - base) == base_res {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        (left, base_res - (left - base))
    } else if query(base + n - 1, end) < base_res {
        let mut left = end + 1;
        let mut right = base + n - 1;
        while left < right {
            let mid = (left + right) / 2;
            let res = query(mid, end);
            if res + (base + n - mid) == base_res {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        (left, base_res - (base + n - left))
    } else {
        (base, base_res)
    };

    if dist_to_end == 1 {
        output!(out, "! {} {}", one_end % n + 1, end % n + 1);
    } else {
        if query(one_end, end + dist_to_end - 1) == 1 {
            output!(
                out,
                "! {} {}",
                one_end % n + 1,
                (end + dist_to_end - 1) % n + 1
            );
        } else {
            output!(
                out,
                "! {} {}",
                one_end % n + 1,
                (end + n - (dist_to_end - 1)) % n + 1
            );
        }
    }
    out.flush();
    assert_eq!(input.read_int(), 1);
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
