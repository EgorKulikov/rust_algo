//{"name":"G. Yelkrab","group":"Universal Cup - The 3rd Universal Cup. Stage 23: Hong Kong","url":"https://contest.ucup.ac/contest/1885/problem/9921","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\naa\nab\nab\nac\nd\n1\naaaaa\n","output":"2 6 1 9 8\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GYelkrab"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::default::default_vec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::all_divisors;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(n);

    #[derive(Default, Clone)]
    struct Node {
        val: usize,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Default::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val ^ right_val.val;
        }
    }

    impl Pushable<usize> for Node {
        fn push(&mut self, delta: usize) {
            self.val += delta;
        }
    }

    let ad: Vec<Vec<usize>> = all_divisors(n + 1, false);

    #[derive(Default)]
    struct Bor {
        qty: usize,
        to: Vec<Option<Bor>>,
    }

    impl Bor {
        fn new() -> Self {
            Self {
                qty: 0,
                to: default_vec(26),
            }
        }
    }

    let mut root = Bor::new();
    let mut st: SegmentTree<Node> = SegmentTree::new(n);

    let mut ans = Vec::new();
    for s in s {
        let mut first = true;
        let mut rec = RecursiveFunction2::new(|rec, node: &mut Bor, s: &[u8]| {
            node.qty += 1;
            if !first {
                for d in ad[node.qty].copy_iter() {
                    st.point_update(d - 1, d);
                }
            } else {
                first = false;
            }
            if !s.is_empty() {
                let c = (s[0] - b'a') as usize;
                if node.to[c].is_none() {
                    node.to[c] = Some(Bor::new());
                }
                rec.call(node.to[c].as_mut().unwrap(), &s[1..]);
            }
        });
        rec.call(&mut root, s.as_slice());
        ans.push(st.query(..).val);
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
