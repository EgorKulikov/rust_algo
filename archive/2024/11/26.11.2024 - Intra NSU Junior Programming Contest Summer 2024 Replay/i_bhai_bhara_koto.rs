//{"name":"I - Bhai Bhara Koto?","group":"LightOJ","url":"https://lightoj.com/contest/injpc-2024-replay/arena/problem/6460","interactive":false,"timeLimit":1000,"tests":[{"input":"8 3\nNadda\nBashundhara\nJFP\nKuril\nKhilkhet\nAirport\nUttara\nDiabari\n5 5 10 5 5 10 5\nNadda 4\nBashundhara 25\nAirport 100\n","output":"Nadda\nAirport\nDiabari\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IBhaiBharaKoto"}}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iters_ref::ItersRef;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let stops = input.read_str_vec(n);
    let c = input.read_int_vec(n - 1);

    let s = c.partial_sums();
    let by_id = stops
        .ref_enumerate()
        .map(|(i, s)| (s.clone(), i))
        .collect::<FxHashMap<_, _>>();

    for _ in 0..q {
        let stop = input.read_str();
        let b = input.read_int();
        let at = *by_id.get(&stop).unwrap();
        let to = s.upper_bound(&(b + s[at])) - 1;
        out.print_line(&stops[to]);
    }
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
