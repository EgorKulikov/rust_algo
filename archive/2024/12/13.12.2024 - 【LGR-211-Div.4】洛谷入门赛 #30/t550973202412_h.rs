//{"name":"T550973 202412H 吃饭大赛总决赛","group":"Luogu","url":"https://www.luogu.com.cn/problem/T550973?contestId=219467","interactive":false,"timeLimit":1000,"tests":[{"input":"4 2 2 3\nquntongtai fr fusu woruo 2 1\nsdutcs wtz ysl zay 1 2\nlongname dx fr woruo 1 1\nlongname booot boot bot 2 2\n","output":"3\nlongname dx fr woruo\nsdutcs wtz ysl zay\nlongname booot boot bot\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T550973202412H"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _m = input.read_size();
    let _t = input.read_size();
    let k = input.read_size();
    let mut teams = input.read_vec::<(Str, [Str; 3], usize, usize)>(n);

    teams.sort_by_key(|x| (x.3, x.2));
    let mut ans = Vec::new();
    let mut promoted = FxHashSet::default();
    for (name, pigs, _, _) in teams {
        let mut good = true;
        for pig in &pigs {
            if promoted.contains(pig) {
                good = false;
                break;
            }
        }
        if good {
            ans.push((name, pigs.clone()));
            promoted.extend(pigs.into_iter());
        }
    }
    out.print_line(ans.len().min(k));
    out.print_per_line_iter(ans.into_iter().take(k));
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
