//{"name":"day13","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day13"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Ordering;

fn solve(input: &mut Input, _test_case: usize) {
    #[derive(Eq, PartialEq)]
    enum Value {
        List(Vec<Value>),
        Number(i64),
    }

    impl Ord for Value {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => a.cmp(b),
                (Value::List(a), Value::List(b)) => a.cmp(b),
                (Value::Number(x), Value::List(_)) => {
                    Value::List(vec![Value::Number(*x)]).cmp(other)
                }
                (Value::List(_), Value::Number(x)) => {
                    self.cmp(&Value::List(vec![Value::Number(*x)]))
                }
            }
        }
    }

    impl PartialOrd for Value {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Value {
        fn parse(input: &[u8]) -> Self {
            if input[0] == b'[' {
                let mut start = 1;
                let mut level = 0;
                let mut result = Vec::new();
                for i in 1..input.len() - 1 {
                    if input[i] == b'[' {
                        level += 1;
                    } else if input[i] == b']' {
                        level -= 1;
                    } else if input[i] == b',' && level == 0 {
                        result.push(Value::parse(&input[start..i]));
                        start = i + 1;
                    }
                }
                if input.len() != 2 {
                    result.push(Value::parse(&input[start..input.len() - 1]));
                }
                Value::List(result)
            } else {
                Value::Number(std::str::from_utf8(input).unwrap().parse().unwrap())
            }
        }
    }

    /*let mut ans = 0;
    for i in 1.. {
        let left = Value::parse(input.read_line().as_bytes());
        let right = Value::parse(input.read_line().as_bytes());
        if left <= right {
            ans += i;
        }
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }
    }*/
    let mut packets = vec![Value::parse(b"[[2]]"), Value::parse(b"[[6]]")];
    while !input.is_exhausted() {
        packets.push(Value::parse(input.read_string().as_bytes()));
        input.skip_whitespace();
    }
    packets.sort();

    out_line!(
        (packets
            .iter()
            .position(|x| x == &Value::parse(b"[[6]]"))
            .unwrap()
            + 1)
            * (packets
                .iter()
                .position(|x| x == &Value::parse(b"[[2]]"))
                .unwrap()
                + 1)
    );
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
