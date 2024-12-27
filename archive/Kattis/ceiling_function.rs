//{"name":"Ceiling Function","group":"Kattis","url":"https://open.kattis.com/problems/ceiling","interactive":false,"timeLimit":5000,"tests":[{"input":"5 3\n2 7 1\n3 1 4\n1 5 9\n2 6 5\n9 7 3\n","output":"4\n"},{"input":"3 4\n3 1 2 40000\n3 4 2 1\n33 42 17 23\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CeilingFunction"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let prototypes = input.read_int_table(n, k);

    #[derive(Eq, PartialEq, Hash)]
    struct Tree {
        left: Option<Box<Tree>>,
        right: Option<Box<Tree>>,
    }

    impl Tree {
        fn build(arr: &[i32]) -> Option<Box<Self>> {
            if arr.is_empty() {
                return None;
            }
            let left = arr
                .iter()
                .copied()
                .skip(1)
                .filter(|&x| x < arr[0])
                .collect::<Vec<_>>();
            let right = arr
                .iter()
                .copied()
                .skip(1)
                .filter(|&x| x >= arr[0])
                .collect::<Vec<_>>();
            Some(Box::new(Self {
                left: Self::build(&left),
                right: Self::build(&right),
            }))
        }
    }

    let mut ans = FxHashSet::default();
    for prototype in prototypes.rows() {
        let tree = Tree::build(&prototypes[prototype]);
        ans.insert(tree);
    }
    out.print_line(ans.len());
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
