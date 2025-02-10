//{"name":"Bing It On","group":"Kattis","url":"https://open.kattis.com/problems/bing","interactive":false,"timeLimit":1000,"tests":[{"input":"10\nhistories\nadventure\nhistory\nhis\nad\nhi\nadvent\nmouse\ncat\nhis\n","output":"0\n0\n0\n2\n1\n3\n1\n0\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BingItOn"}}}

use algo_lib::collections::vec_ext::default::default_vec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let words = input.read_str_vec(n);

    struct Node {
        count: usize,
        next: Vec<Option<Node>>,
    }

    impl Node {
        fn new() -> Self {
            Self {
                count: 0,
                next: default_vec(26),
            }
        }

        fn add(&mut self, w: &[u8]) -> usize {
            if w.is_empty() {
                self.count += 1;
                return self.count - 1;
            }
            self.count += 1;
            let c = (w[0] - b'a') as usize;
            if self.next[c].is_none() {
                self.next[c] = Some(Node::new());
            }
            self.next[c].as_mut().unwrap().add(&w[1..])
        }
    }
    let mut root = Node::new();
    for word in words {
        out.print_line(root.add(word.as_slice()));
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
