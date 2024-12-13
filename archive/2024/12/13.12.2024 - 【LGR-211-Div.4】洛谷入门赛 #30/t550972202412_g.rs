//{"name":"T550972 202412G 正在联系教练退赛","group":"Luogu","url":"https://www.luogu.com.cn/problem/T550972?contestId=219467","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nyifusuyi\nerFusuer\nyiFusuYi\n2\nfusu\ner\n","output":"Yes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T550972202412G"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::aho_corasick::{AhoCorasick, Payload};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(n);
    let m = input.read_size();
    let t = input.read_str_vec(m);

    #[derive(Default)]
    struct Node(bool);
    impl Payload for Node {
        fn add_single(&mut self, _id: usize) {
            self.0 = true;
        }

        fn add_node(&mut self, other: &Self) {
            self.0 |= other.0;
        }
    }
    let ac = AhoCorasick::<Node, 94, 33>::new(&t);
    out.set_bool_output(BoolOutput::YesNo);
    for s in s {
        out.print_line(ac.iterate(&s).any(|x| x.0));
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
