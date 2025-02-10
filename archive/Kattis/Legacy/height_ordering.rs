//{"name":"Height Ordering","group":"Kattis","url":"https://open.kattis.com/problems/height","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 900 901 902 903 904 905 906 907 908 909 910 911 912 913 914 915 916 917 918 919\n2 919 918 917 916 915 914 913 912 911 910 909 908 907 906 905 904 903 902 901 900\n3 901 902 903 904 905 906 907 908 909 910 911 912 913 914 915 916 917 918 919 900\n4 918 917 916 915 914 913 912 911 910 909 908 907 906 905 904 903 902 901 900 919\n","output":"1 0\n2 190\n3 19\n4 171\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HeightOrdering"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    assert_eq!(test_case, input.read_size());
    let height = input.read_size_vec(20);

    let mut ans = 0;
    for i in 0..20 {
        for j in 0..i {
            if height[j] > height[i] {
                ans += 1;
            }
        }
    }
    out.print_line((test_case, ans));
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
