//{"name":"K - Triplet Xor","group":"LightOJ","url":"https://lightoj.com/contest/injpc-2024-replay/arena/problem/6462","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 4\n2 3 1 5 7\n5 3\n2 3 1 5 7\n","output":"2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KTripletXor"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let p = input.read_long_vec(n);

    let mut seen = DefaultHashMap::<_, usize>::new();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans += seen[x ^ p[i] ^ p[j]];
        }
        seen[p[i]] += 1;
    }
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
