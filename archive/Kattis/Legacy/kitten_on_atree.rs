//{"name":"Kitten on a Tree","group":"Kattis","url":"https://open.kattis.com/problems/kitten","interactive":false,"timeLimit":1000,"tests":[{"input":"14\n25 24\n4 3 1 2\n13 9 4 11\n10 20 8 7\n32 10 21\n23 13 19 32 22\n19 12 5 14 17 30\n14 6 15 16\n30 18 31 29\n24 23 26\n26 27 28\n-1\n","output":"14 19 23 24 25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KittenOnATree"}}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size() - 1;
    let mut parent = vec![None; 100];
    while input.peek() != Some(b'-') {
        let a = input.read_size() - 1;
        let line = input.read_line();
        for b in line.split(b' ') {
            parent[b.parse::<usize>() - 1] = Some(a);
        }
    }

    let mut ans = vec![k];
    while let Some(k) = parent[ans[Back(0)]] {
        ans.push(k);
    }
    out.print_line(ans.inc());
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
