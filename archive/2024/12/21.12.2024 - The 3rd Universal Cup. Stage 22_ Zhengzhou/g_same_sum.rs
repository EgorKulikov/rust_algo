//{"name":"G. Same Sum","group":"Universal Cup - The 3rd Universal Cup. Stage 22: Zhengzhou","url":"https://contest.ucup.ac/contest/1873/problem/9774","interactive":false,"timeLimit":1000,"tests":[{"input":"8 4\n1 2 3 4 5 6 7 8\n2 1 8\n1 1 4 4\n2 1 6\n2 1 8\n","output":"YES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSameSum"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::random;
use algo_lib::misc::test_type::TaskType;
use algo_lib::{dynamic_value, value_ref};

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::num_utils::Powers;
use algo_lib::numbers::primes::prime::next_prime;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    dynamic_value!(HM: i64 = next_prime(
        random().gen_range(10i64.pow(18)..=2 * 10i64.pow(18)),
    ));
    type HashMod = ModInt<i64, HM>;
    let mult = HashMod::new(random().gen_range(4 * 10i64.pow(17)..=5 * 10i64.pow(17)));
    let rev = mult.inv().unwrap();
    value_ref!(Mult MULT: Powers<HashMod> = Powers::new(mult, 200_001));
    value_ref!(Rev REV: Powers<HashMod> = Powers::new(rev, 200_001));

    #[derive(Clone, Default)]
    struct Node {
        min: usize,
        max: usize,
        hash_up: HashMod,
        hash_down: HashMod,
        delta: usize,
    }

    impl Node {
        fn new(a: usize) -> Self {
            Self {
                min: a,
                max: a,
                hash_up: Mult::val().power(a),
                hash_down: Rev::val().power(a),
                delta: 0,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.min = left_val.min.min(right_val.min);
            self.max = left_val.max.max(right_val.max);
            self.hash_up = left_val.hash_up + right_val.hash_up;
            self.hash_down = left_val.hash_down + right_val.hash_down;
        }

        fn accumulate(&mut self, value: &Self) {
            self.delta += value.delta;
            self.min += value.delta;
            self.max += value.delta;
            self.hash_up *= Mult::val().power(value.delta);
            self.hash_down *= Rev::val().power(value.delta);
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let mut st = SegmentTree::gen(n, |i| Node::new(a[i]));
    for _ in 0..q {
        let command = input.read_int();
        match command {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let v = input.read_size();
                st.update(
                    l..r,
                    &Node {
                        delta: v,
                        ..Default::default()
                    },
                );
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let node = st.query(l..r);
                out.print_line(
                    node.hash_up * Rev::val().power(node.min)
                        == node.hash_down * Mult::val().power(node.max),
                );
            }
            _ => unreachable!(),
        }
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
