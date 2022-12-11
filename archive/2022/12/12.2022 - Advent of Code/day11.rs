//{"name":"day11","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day11"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{EOLString, Input};
use algo_lib::io::output::output;
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::gcd::lcm;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::{dynamic_value, out_line, scan};
use std::cmp::Reverse;

fn solve(input: &mut Input, _test_case: usize) {
    enum Operation {
        Add,
        Multiply,
    }

    impl Operation {
        fn apply(&self, a: i64, b: i64) -> i64 {
            match self {
                Operation::Add => a + b,
                Operation::Multiply => a * b,
            }
        }
    }

    enum Operand {
        Value(i64),
        Old,
    }

    impl Operand {
        fn value(&self, old: i64) -> i64 {
            match self {
                Operand::Value(value) => *value,
                Operand::Old => old,
            }
        }
    }

    struct Monkey {
        items: Vec<i64>,
        operation: Operation,
        left: Operand,
        right: Operand,
        test: i64,
        if_true: usize,
        if_false: usize,
        inspected: usize,
    }

    let mut monkeys: Vec<Monkey> = Vec::new();

    while !input.is_exhausted() {
        scan!(
            input,
            format!(
                "Monkey {}:\n  Starting items: @\n  Operation: new = @ @ @\n  Test: divisible by @\n    If true: throw to monkey @\n    If false: throw to monkey @",
                monkeys.len(),
            ).as_str(),
            items: EOLString,
            left_operand: String,
            operation: String,
            right_operand: String,
            test: i64,
            if_true: usize,
            if_false: usize,
        );
        let items = items
            .0
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        monkeys.push(Monkey {
            items,
            operation: match operation.as_str() {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => panic!("Unknown operation"),
            },
            left: match left_operand.as_str() {
                "old" => Operand::Old,
                _ => Operand::Value(left_operand.parse::<i64>().unwrap()),
            },
            right: match right_operand.as_str() {
                "old" => Operand::Old,
                _ => Operand::Value(right_operand.parse::<i64>().unwrap()),
            },
            test,
            if_true,
            if_false,
            inspected: 0,
        });
        input.skip_whitespace();
    }

    let lcm = monkeys.iter().map(|m| m.test).fold(1, |a, b| lcm(a, b));

    // for _ in 0..20 {
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let (head, tail) = monkeys.split_at_mut(i);
            let (cur, tail) = tail.split_first_mut().unwrap();

            for &j in &cur.items {
                cur.inspected += 1;
                // let j = cur.operation.apply(cur.left.value(j), cur.right.value(j)) / 3;
                let j = cur.operation.apply(cur.left.value(j), cur.right.value(j)) % lcm;
                let id = if j % cur.test == 0 {
                    cur.if_true
                } else {
                    cur.if_false
                };
                if id < i {
                    head[id].items.push(j);
                } else {
                    tail[id - i - 1].items.push(j);
                }
            }
            cur.items.clear();
        }
    }

    monkeys.sort_by_key(|m| Reverse(m.inspected));
    out_line!(monkeys[0].inspected * monkeys[1].inspected);
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
