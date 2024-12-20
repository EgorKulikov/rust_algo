//{"name":"Bacon, Eggs, and Spam","group":"Kattis","url":"https://open.kattis.com/problems/baconeggsandspam","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nAlice bacon eggs spam\nSue pancakes sausage ham\nDavid eggs spam\n1\nBill diet-coke\n0\n","output":"bacon Alice\neggs Alice David\nham Sue\npancakes Sue\nsausage Sue\nspam Alice David\n\ndiet-coke Bill\n\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BaconEggsAndSpam"}}}

use algo_lib::collections::default_map::default_tree_map::DefaultTreeMap;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = DefaultTreeMap::<_, Vec<_>>::new();
    for _ in 0..n {
        let name = input.read_str();
        let items = input.read_line();
        for item in items.split(b' ') {
            ans[item.into_owned()].push(name.clone());
        }
    }
    for (k, v) in ans {
        out.print_line((k, v.sorted()));
    }
    out.print_line(());
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
