//{"name":"Gasoline","group":"CodeChef - START223A","url":"https://www.codechef.com/START223A/problems/GASOLINE7","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 2\n1 2\n3 2\n2 1 3\n3 1\n2 1 3\n5 3\n5 5 5 5 5\n","output":"5\n15\n19\n175\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);

    type Mod = ModIntF;
    #[derive(Default, Clone)]
    struct Node {
        val: Mod,
        weight: Mod,
        delta: Option<Mod>,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val + right_val.val;
            self.weight = left_val.weight + right_val.weight;
        }

        fn accumulate(&mut self, value: &Self) {
            if let Some(d) = value.delta {
                self.val = d * self.weight;
                self.delta = Some(d);
            }
        }

        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node {
        weight: Mod::from(n - i),
        ..Default::default()
    });
    let mut w = vec![(n, 0)];
    let mut ans = Mod::from(0);
    for i in (0..n).rev() {
        while a[i] < w[Back(0)].1 {
            w.pop();
        }
        let end = w[Back(0)].0.min(i + k);
        w.push((i, a[i]));
        st.update(
            i..end,
            &Node {
                delta: Some(Mod::from(a[i])),
                ..Default::default()
            },
        );
        ans += st.query(i..).val;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
