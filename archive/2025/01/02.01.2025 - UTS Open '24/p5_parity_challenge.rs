//{"name":"P5 - Parity Challenge","group":"DMOJ - UTS Open '24","url":"https://dmoj.ca/problem/utso24p5","interactive":false,"timeLimit":1000,"tests":[{"input":"5 6\n3 1 4 1 5\n+++x\nQ 1 2\nO 2\nQ 3 4\nQ 5 5\nV 5\nQ 3 4\n","output":"odd\neven\neven\nodd\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5ParityChallenge"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    #[derive(Copy, Clone)]
    enum Value {
        Add,
        Mult,
        Single(bool),
        Full(bool, bool, bool),
    }

    impl Value {
        fn combine(left: Self, right: Self) -> Self {
            match (left, right) {
                (Value::Mult, x) | (x, Value::Mult) => x,
                (Value::Add, Value::Single(x)) => Value::Full(true, false, x),
                (Value::Single(x), Value::Add) => Value::Full(x, false, true),
                (Value::Full(a, b, c), Value::Add) => Value::Full(a, b ^ c, true),
                (Value::Add, Value::Full(a, b, c)) => Value::Full(true, a ^ b, c),
                (Value::Single(x), Value::Single(y)) => Value::Single(x & y),
                (Value::Single(x), Value::Full(a, b, c)) => Value::Full(a & x, b, c),
                (Value::Full(a, b, c), Value::Single(x)) => Value::Full(a, b, c & x),
                (Value::Full(a, b, c), Value::Full(x, y, z)) => Value::Full(a, b ^ (c & x) ^ y, z),
                _ => unreachable!(),
            }
        }

        fn compute(self) -> Self {
            match self {
                Value::Single(x) => Value::Single(x),
                Value::Full(a, b, c) => Value::Single(a ^ b ^ c),
                _ => unreachable!(),
            }
        }
    }

    impl SegmentTreeNode for Value {
        fn new(_left: usize, _right: usize) -> Self {
            Self::Single(false)
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            *self = Self::combine(*left_val, *right_val);
        }
    }

    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_int_vec(n);
    let mut op = input.read_str();

    let mut st = SegmentTree::gen(2 * n - 1, |i| {
        if i % 2 == 0 {
            Value::Single(a[i / 2] % 2 == 1)
        } else {
            match op[i / 2] {
                b'+' => Value::Add,
                b'x' => Value::Mult,
                _ => unreachable!(),
            }
        }
    });

    for _ in 0..q {
        let command = input.read_char();
        match command {
            b'V' => {
                let idx = input.read_size() - 1;
                a[idx] += 1;
                st.point_update(idx * 2, Value::Single(a[idx] % 2 == 1));
            }
            b'O' => {
                let idx = input.read_size() - 1;
                op[idx] = match op[idx] {
                    b'+' => b'x',
                    b'x' => b'+',
                    _ => unreachable!(),
                };
                st.point_update(
                    idx * 2 + 1,
                    match op[idx] {
                        b'+' => Value::Add,
                        b'x' => Value::Mult,
                        _ => unreachable!(),
                    },
                );
            }
            b'Q' => {
                let l = input.read_size() - 1;
                let r = input.read_size() - 1;
                let mut ans = st.query(l * 2..=r * 2).compute();
                if l != 0 {
                    let left = st.query(..l * 2);
                    ans = Value::combine(left, ans);
                }
                if r != n - 1 {
                    let right = st.query(r * 2 + 1..);
                    ans = Value::combine(ans, right);
                }
                ans = ans.compute();
                match ans {
                    Value::Single(x) => {
                        out.print_line(if x { "odd" } else { "even" });
                    }
                    _ => unreachable!(),
                }
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
