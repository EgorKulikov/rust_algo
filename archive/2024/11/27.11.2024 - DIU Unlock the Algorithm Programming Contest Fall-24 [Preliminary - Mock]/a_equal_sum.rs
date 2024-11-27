//{"name":"A. Equal sum","group":"Toph","url":"https://toph.co/arena?practice=67470fb3aca7a845c3abf7a2#!/p/67417e886ca53f1880b565c4","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n-5 -3 -5\n-5 -2 -5\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AEqualSum"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_int_table(n, 3);

    fn ways(a: &[i32]) -> DefaultHashMap<i32, usize> {
        let mut res = DefaultHashMap::new();
        for i in 0..3 {
            for j in 0..i {
                res[a[i] + a[j]] += 1;
            }
        }
        res
    }
    let variants = ways(&t[0]).keys().copied().collect_vec();
    let mut ans = 0;
    for var in variants {
        let mut cur = 1;
        for i in 0..n {
            cur *= ways(&t[i])[var];
        }
        ans += cur;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
