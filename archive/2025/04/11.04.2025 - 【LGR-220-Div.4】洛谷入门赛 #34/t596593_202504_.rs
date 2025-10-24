//{"name":"T596593 [语言月赛 202504] 古诗求和","group":"Luogu","url":"https://www.luogu.com.cn/problem/T596593?contestId=240317","interactive":false,"timeLimit":1000,"tests":[{"input":"1qu2_3li,yancun4_5jia.tingtai6_7zuo,8_9_10zhihua.\n","output":"5 Odd\n5 Even\n"},{"input":"40nianlaijiaguo,3000lidishanhe.fenggelongloulianxiaohan,yushuqiongzhizuoyanluo,jicengshigange?1danguiweichenlu,shenyaopanbinxiaomo.zuishicanghuangcimiaori,jiaofangyouzoubielige,chuileiduigonge.\n","output":"2 Even\n0 Even\n1 Odd\n0 Even\n"},{"input":"1daocanyangpushuizhong,banjiangsesebanjianghong.kelian9yuechu3ye,lusizhenzhuyuesigong.\n","output":"1 Odd\n2 Even\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    for s in s.split(|c| b".?!".contains(c)) {
        if s.is_empty() {
            continue;
        }
        let mut oddity = false;
        let mut qty = 0;
        for d in s.split(|c| !c.is_ascii_digit()) {
            if !d.is_empty() {
                qty += 1;
                oddity ^= d[Back(0)] % 2 == b'1' % 2;
            }
        }
        out.print_line((qty, if oddity { "Odd" } else { "Even" }));
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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
