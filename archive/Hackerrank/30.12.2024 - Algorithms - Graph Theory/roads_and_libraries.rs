//{"name":"Roads and Libraries","group":"HackerRank - Algorithms - Graph Theory","url":"https://www.hackerrank.com/challenges/torque-and-development/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign","interactive":false,"timeLimit":4000,"tests":[{"input":"STDIN       Function\n-----       --------\n2           q = 2\n3 3 2 1     n = 3, cities[] size m = 3, c_lib = 2, c_road = 1\n1 2         cities = [[1, 2], [3, 1], [2, 3]]\n3 1\n2 3\n6 6 2 5     n = 6, cities[] size m = 6, c_lib = 2, c_road = 5\n1 3         cities = [[1, 3], [3, 4],...]\n3 4\n2 4\n1 2\n2 3\n5 6\n","output":"4\n12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"RoadsAndLibraries"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let c_lib = input.read_size();
    let c_road = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    if c_lib <= c_road {
        out.print_line(n * c_lib);
        return;
    }
    let mut dsu = DSU::new(n);
    let mut ans = 0;
    for (u, v) in edges {
        if dsu.union(u, v) {
            ans += c_road;
        }
    }
    ans += c_lib * dsu.set_count();
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
