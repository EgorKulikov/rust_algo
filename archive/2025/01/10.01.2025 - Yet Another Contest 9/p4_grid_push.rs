//{"name":"P4 - Grid Push","group":"DMOJ - Yet Another Contest 9","url":"https://dmoj.ca/problem/yac9p4","interactive":false,"timeLimit":3000,"tests":[{"input":"1\n1\n1\n","output":"1\n"},{"input":"1\n1\n2\n","output":"2\n"},{"input":"2\n1 1\n1 2\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P4GridPush"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    type Mod = ModInt7;
    let c = Combinations::<Mod>::new(2 * n + 1);
    let mut same_a = 0;
    while same_a < n && a[same_a] == b[0] {
        same_a += 1;
    }
    let mut same_b = 0;
    while same_b < n && b[same_b] == a[0] {
        same_b += 1;
    }
    let mut ans = Mod::zero();
    for i in 0..n - same_a {
        ans += c.c(n - 1 + i, n - 1);
    }
    for i in 0..n - same_b {
        ans += c.c(n - 1 + i, n - 1);
    }
    if same_b != n {
        for i in 0..same_a {
            ans += c.c(n - 1 - i + n - same_b - 1, n - i - 1);
        }
    }
    if same_a != n {
        for i in 0..same_b {
            ans += c.c(n - 1 - i + n - same_a - 1, n - i - 1);
        }
    }
    if same_b == n && same_a == n {
        ans += Mod::one();
    }
    out.print_line(ans);
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
