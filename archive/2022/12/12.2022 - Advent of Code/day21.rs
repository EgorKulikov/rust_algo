//{"name":"day21","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day21"}}}

use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::rational::Rational;
use algo_lib::{out_line, scan};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
        Equality,
    }

    impl Operation {
        fn apply(
            &self,
            (a0, a1): (Rational<i64>, Rational<i64>),
            (b0, b1): (Rational<i64>, Rational<i64>),
        ) -> (Rational<i64>, Rational<i64>) {
            match self {
                Operation::Add => (a0 + b0, a1 + b1),
                Operation::Subtract => (a0 - b0, a1 - b1),
                Operation::Multiply => {
                    assert!(a0 == Rational::zero() || b0 == Rational::zero());
                    (a0 * b1 + a1 * b0, a1 * b1)
                }
                Operation::Divide => {
                    assert_eq!(b0, Rational::zero());
                    (a0 / b1, a1 / b1)
                }
                Operation::Equality => {
                    assert_ne!(a0, b0);
                    (Rational::zero(), (b1 - a1) / (a0 - b0))
                }
            }
        }
    }

    impl Readable for Operation {
        fn read(input: &mut Input) -> Self {
            let op = input.read_char();
            match op {
                '+' => Operation::Add,
                '-' => Operation::Subtract,
                '*' => Operation::Multiply,
                '/' => Operation::Divide,
                _ => panic!("Unknown operation: {}", op),
            }
        }
    }

    enum Value {
        Number((Rational<i64>, Rational<i64>)),
        Operation(Operation, String, String),
    }

    impl Value {
        fn get(
            &self,
            variables: &HashMap<String, (Rational<i64>, Rational<i64>)>,
        ) -> (Rational<i64>, Rational<i64>) {
            match self {
                Value::Number(n) => *n,
                Value::Operation(op, a, b) => op.apply(variables[a], variables[b]),
            }
        }

        fn can_calculate(
            &self,
            variables: &HashMap<String, (Rational<i64>, Rational<i64>)>,
        ) -> bool {
            match self {
                Value::Number(_) => true,
                Value::Operation(_, a, b) => variables.contains_key(a) && variables.contains_key(b),
            }
        }
    }

    impl Readable for Value {
        fn read(input: &mut Input) -> Self {
            let value = input.read_string();
            if let Ok(n) = value.parse::<i64>() {
                Value::Number((Rational::zero(), Rational::new(n, 1)))
            } else {
                let op = input.read();
                let b = input.read_string();
                Value::Operation(op, value, b)
            }
        }
    }

    let mut values = Vec::new();
    let mut calculated = HashMap::new();

    while !input.is_exhausted() {
        scan!(input, "@: @", id: String, value: Value);
        let mut value = value;
        match id.as_str() {
            "humn" => value = Value::Number((Rational::one(), Rational::zero())),
            "root" => {
                if let Value::Operation(op, _, _) = &mut value {
                    *op = Operation::Equality;
                } else {
                    unreachable!();
                }
            }
            _ => {}
        }
        values.push((id, value));
        input.skip_whitespace();
    }

    let mut rem = values.len();
    while rem > 0 {
        for (id, value) in &values {
            if !calculated.contains_key(id) && value.can_calculate(&calculated) {
                calculated.insert(id.clone(), value.get(&calculated));
                rem -= 1;
            }
        }
    }

    out_line!(calculated["root"].1);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
