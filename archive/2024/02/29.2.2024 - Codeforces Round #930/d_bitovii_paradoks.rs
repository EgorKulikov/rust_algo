//{"name":"D. Битовый парадокс","group":"Codeforces - Codeforces Round 930 (Div. 1)","url":"https://codeforces.com/contest/1936/problem/D","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n3 7\n2 1 3\n2 2 3\n4\n2 1 3\n1 2 5\n2 2 3\n2 1 3\n4 5\n5 1 2 4\n4 2 3 3\n6\n2 1 4\n1 3 15\n2 3 4\n2 2 4\n1 2 13\n2 1 4\n1 5\n6\n4\n1\n2 1 1\n","output":"-1 3 2\n5 2 2 1\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBitoviiParadoks"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::value::{DynamicValue, Value};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let v = input.read_unsigned();
    let a = input.read_int_vec(n);
    let mut b = input.read_unsigned_vec(n);

    dynamic_value!(V: u32 = v);

    #[derive(Copy, Clone)]
    struct Value {
        a: i32,
        b: u32,
    }

    impl Value {
        fn new(a: i32, b: u32) -> Self {
            Self { a, b }
        }

        fn join(&self, other: &Self) -> Self {
            Self::new(self.a.max(other.a), self.b | other.b)
        }
    }

    trait Updatable {
        fn update(&mut self, val: Value) -> Self;
    }

    impl Updatable for Option<i32> {
        fn update(&mut self, val: Value) -> Self {
            if val.b < V::val() {
                return *self;
            }
            self.minim(val.a);
            *self
        }
    }

    #[derive(Clone)]
    struct Node {
        all: Value,
        answer: Option<i32>,
        from_left: Vec<Value>,
        from_right: Vec<Value>,
    }

    impl Node {
        fn single(a: i32, b: u32) -> Self {
            Self {
                all: Value::new(a, b),
                answer: None.update(Value::new(a, b)),
                from_left: vec![Value::new(0, 0), Value::new(a, b)],
                from_right: vec![Value::new(0, 0), Value::new(a, b)],
            }
        }

        fn check(&self) {
            for (a, b) in self.from_left.consecutive_iter() {
                assert!(a.a <= b.a);
                assert!(a.b < b.b);
            }
            for (a, b) in self.from_right.consecutive_iter() {
                assert!(a.a <= b.a);
                assert!(a.b < b.b);
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                all: Value::new(0, 0),
                answer: None,
                from_left: Vec::new(),
                from_right: Vec::new(),
            }
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            left_val.check();
            right_val.check();
            self.all = left_val.all.join(&right_val.all);
            self.from_left = left_val.from_left.clone();
            for rv in &right_val.from_left {
                let last = self.from_left.last().unwrap();
                if last.b >= V::val() {
                    break;
                }
                if last.b < (left_val.all.b | rv.b) {
                    self.from_left.push(left_val.all.join(rv));
                }
            }
            self.from_right = right_val.from_right.clone();
            for lv in left_val.from_right.iter() {
                let first = self.from_right.last().unwrap();
                if first.b >= V::val() {
                    break;
                }
                if first.b < (lv.b | right_val.all.b) {
                    self.from_right.push(lv.join(&right_val.all));
                }
            }
            self.answer = left_val.answer;
            if let Some(rv) = right_val.answer {
                self.answer.minim(rv);
            }
            let mut i = 0;
            for lv in left_val.from_right.iter().rev() {
                while i < right_val.from_left.len() && lv.b | right_val.from_left[i].b < V::val() {
                    i += 1;
                }
                if i < right_val.from_left.len() {
                    self.answer.update(lv.join(&right_val.from_left[i]));
                } else {
                    break;
                }
            }
            self.check();
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    impl Pushable<&(i32, u32)> for Node {
        fn push(&mut self, x: &(i32, u32), _: usize, _: usize) {
            *self = Self::single(x.0, x.1);
        }
    }

    let mut st = SegmentTree::from_generator(n, |i| Node::single(a[i], b[i]));

    let q = input.read_size();
    for _ in 0..q {
        let tp = input.read_size();

        match tp {
            1 => {
                let pos = input.read_size() - 1;
                let nb = input.read_unsigned();

                b[pos] = nb;
                st.update(pos..=pos, &(a[pos], nb));
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let res = st.query(l..r);
                out.print_line(res.answer);
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
