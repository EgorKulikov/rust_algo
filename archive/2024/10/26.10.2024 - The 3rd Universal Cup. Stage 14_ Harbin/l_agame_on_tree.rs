//{"name":"L. A Game On Tree","group":"Universal Cup - The 3rd Universal Cup. Stage 14: Harbin","url":"https://contest.ucup.ac/contest/1817/problem/9530","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3\n1 2\n2 3\n5\n1 2\n1 5\n3 2\n4 2\n","output":"443664158\n918384806\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LAGameOnTree"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    type Mod = ModIntF;
    struct State {
        sum: Mod,
        sum_linear: Mod,
        sum_square: Mod,
        dangling_one: Mod,
        dangling_ans: Mod,
        ans: Mod,
    }

    impl State {
        fn single() -> Self {
            Self {
                sum: Mod::one(),
                sum_linear: Mod::zero(),
                sum_square: Mod::zero(),
                dangling_one: Mod::zero(),
                dangling_ans: Mod::zero(),
                ans: Mod::zero(),
            }
        }

        fn add_root(&mut self) {
            self.ans += self.sum_square * Mod::new(2) + self.dangling_ans * Mod::new(2);
            self.dangling_ans += self.sum_square;
            self.dangling_one += Mod::one();
            self.sum_square += self.sum_linear * Mod::new(2) + self.sum;
            self.sum_linear += self.sum;
            self.sum += self.dangling_one * Mod::new(2);
        }

        fn join(mut left: Self, right: Self) -> Self {
            left.ans = Mod::new_from_wide(
                left.ans.val() as i64
                    + right.ans.val() as i64
                    + left.sum_square.val() as i64 * right.sum.val() as i64
                    + 2 * left.sum_linear.val() as i64 * right.sum_linear.val() as i64
                    + left.sum.val() as i64 * right.sum_square.val() as i64
                    + 2 * left.dangling_one.val() as i64 * right.dangling_ans.val() as i64
                    + 2 * right.dangling_one.val() as i64 * left.dangling_ans.val() as i64,
            );
            left.dangling_ans = Mod::new_from_wide(
                left.dangling_ans.val() as i64
                    + right.dangling_ans.val() as i64
                    + left.dangling_one.val() as i64 * right.sum_square.val() as i64
                    + right.dangling_one.val() as i64 * left.sum_square.val() as i64,
            );
            left.sum = Mod::new_from_wide(
                left.sum.val() as i64
                    + right.sum.val() as i64
                    + left.dangling_one.val() as i64 * right.dangling_one.val() as i64 * 2,
            );
            left.dangling_one += right.dangling_one;
            left.sum_linear += right.sum_linear;
            left.sum_square += right.sum_square;
            left
        }
    }

    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> State {
        let mut state = State::single();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let mut call = f.call(e.to(), vert);
            call.add_root();
            state = State::join(state, call);
        }
        state
    });
    let paths = Mod::from_index(n) * Mod::from_index(n - 1) / Mod::new(2);
    let ans = dfs.call(0, n).ans / paths / paths;
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
