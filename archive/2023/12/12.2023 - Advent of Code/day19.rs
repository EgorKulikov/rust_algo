//{"name":"day19","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day19"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::str_scan;
use algo_lib::string::str::{Str, StrReader};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut processes = HashMap::new();
    loop {
        let line = input.read_line();
        if line.is_empty() {
            break;
        }
        str_scan!(line, "@{@}", id: Str<'static>, conditions: Str<'static>);
        let conditions = conditions.split(b',');
        let mut cond = Vec::new();
        for c in conditions {
            let c = c.into_owned();
            if c.contains(&b'<') {
                str_scan!(c, "@<@:@", attr: char, val: i64, target: Str<'static>);
                cond.push((Ordering::Less, attr, val, target));
            } else if c.contains(&b'>') {
                str_scan!(c, "@>@:@", attr: char, val: i64, target: Str<'static>);
                cond.push((Ordering::Greater, attr, val, target));
            } else {
                cond.push((Ordering::Equal, ' ', 0, c));
            }
        }
        processes.insert(id, cond);
    }
    let mut shapes = Vec::new();
    type Shape = HashMap<char, i64>;
    while !input.is_exhausted() {
        let line = input.read_line();
        if line.is_empty() {
            break;
        }
        str_scan!(line, "{x=@,m=@,a=@,s=@}", x: i64, m: i64, a: i64, s: i64);
        shapes.push(Shape::from([('x', x), ('m', m), ('a', a), ('s', s)]));
    }

    {
        // part 1
        let mut ans = 0;
        for shape in &shapes {
            let mut cur = Str::from(b"in");
            loop {
                if !processes.contains_key(&cur) {
                    break;
                }
                for condition in &processes[&cur] {
                    match condition.0 {
                        Ordering::Less => {
                            if shape[&condition.1] < condition.2 {
                                cur = condition.3.clone();
                                break;
                            }
                        }
                        Ordering::Equal => {
                            cur = condition.3.clone();
                            break;
                        }
                        Ordering::Greater => {
                            if shape[&condition.1] > condition.2 {
                                cur = condition.3.clone();
                                break;
                            }
                        }
                    }
                }
            }
            if cur == Str::from(b"A") {
                ans += shape.values().sum::<i64>();
            }
        }
        out.print_line(ans);
    }

    {
        // part 2
        #[derive(Clone)]
        struct ShapeRange {
            from: Shape,
            to: Shape,
        }

        let mut queue = VecDeque::from(vec![ShapeRange {
            from: Shape::from([('x', 1), ('m', 1), ('a', 1), ('s', 1)]),
            to: Shape::from([('x', 4000), ('m', 4000), ('a', 4000), ('s', 4000)]),
        }]);

        let mut ans = 0;
        while let Some(mut range) = queue.pop_front() {
            let mut cur = Str::from(b"in");
            loop {
                if !processes.contains_key(&cur) {
                    break;
                }
                for condition in &processes[&cur] {
                    match condition.0 {
                        Ordering::Less => {
                            if range.to[&condition.1] < condition.2 {
                                cur = condition.3.clone();
                                break;
                            } else if range.from[&condition.1] < condition.2 {
                                queue.push_back(range.clone().do_with(|r| {
                                    r.to.insert(condition.1, condition.2 - 1);
                                }));
                                range.from.insert(condition.1, condition.2);
                            }
                        }
                        Ordering::Equal => {
                            cur = condition.3.clone();
                            break;
                        }
                        Ordering::Greater => {
                            if range.from[&condition.1] > condition.2 {
                                cur = condition.3.clone();
                                break;
                            } else if range.to[&condition.1] > condition.2 {
                                queue.push_back(range.clone().do_with(|r| {
                                    r.from.insert(condition.1, condition.2 + 1);
                                }));
                                range.to.insert(condition.1, condition.2);
                            }
                        }
                    }
                }
            }
            if cur == Str::from(b"A") {
                let mut cur = 1;
                for c in "xmas".chars() {
                    cur *= range.to[&c] - range.from[&c] + 1;
                }
                ans += cur;
            }
        }
        out.print_line(ans);
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
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
