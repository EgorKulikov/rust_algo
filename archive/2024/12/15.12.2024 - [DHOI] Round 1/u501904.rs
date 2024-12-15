//{"name":"U501904 闪耀之塔","group":"Luogu","url":"https://www.luogu.com.cn/problem/U501904?contestId=218668","interactive":false,"timeLimit":1000,"tests":[{"input":"2 1\n2\n11\n","output":"3\n"},{"input":"10 3\n4\n1001\n8\n10110110\n3\n111\n","output":"84582\n5362\n163710\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"U501904"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::series::sum_geometric_series;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();

    type Mod = ModInt7;
    for _ in 0..q {
        let k = input.read_size();
        let _p = input.read_str();
        let left = sum_geometric_series(Mod::new(2).power(k), Mod::new(4), n - k + 1);
        let right1 = sum_geometric_series(Mod::new(2).inv().unwrap(), Mod::new(4), n - k + 1);
        let right2 = sum_geometric_series(Mod::new(2).inv().unwrap(), Mod::new(2), n - k + 1);
        out.print_line(left - right1 - right2);
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
