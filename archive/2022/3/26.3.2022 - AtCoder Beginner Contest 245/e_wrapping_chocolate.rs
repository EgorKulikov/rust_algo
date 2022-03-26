//{"name":"E - Wrapping Chocolate","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_e","interactive":false,"timeLimit":4000,"tests":[{"input":"2 3\n2 4\n3 2\n8 1 5\n2 10 5\n","output":"Yes\n"},{"input":"2 2\n1 1\n2 2\n100 1\n100 1\n","output":"No\n"},{"input":"1 1\n10\n100\n100\n10\n","output":"No\n"},{"input":"1 1\n10\n100\n10\n100\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EWrappingChocolate"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Ordering;
use std::collections::BTreeSet;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let c = input.read_int_vec(m);
    let d = input.read_int_vec(m);

    #[derive(Eq, PartialEq)]
    enum Event {
        Add(i32, i32),
        Remove(i32, i32),
    }

    impl Event {
        fn at(&self) -> i32 {
            match self {
                Event::Add(time, _) | Event::Remove(time, _) => *time,
            }
        }
    }

    impl PartialOrd for Event {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.at() != other.at() {
                other.at().partial_cmp(&self.at())
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                match self {
                    Event::Add(_, _) => Some(Ordering::Less),
                    Event::Remove(_, _) => Some(Ordering::Greater),
                }
            }
        }
    }

    impl Ord for Event {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    let mut events = Vec::with_capacity(n + m);
    for (a, b) in a.into_iter().zip(b.into_iter()) {
        events.push(Event::Remove(a, b));
    }
    for (c, d) in c.into_iter().zip(d.into_iter()) {
        events.push(Event::Add(c, d));
    }
    events.sort();
    #[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
    struct Box {
        width: i32,
        id: usize,
    }
    let mut next_id = 1;
    let mut set = BTreeSet::new();
    for event in events {
        match event {
            Event::Add(_, width) => {
                set.insert(Box { width, id: next_id });
                next_id += 1;
            }
            Event::Remove(_, width) => match set.range(Box { width, id: 0 }..).next() {
                None => {
                    out_line!("No");
                    return;
                }
                Some(b) => {
                    let b = b.clone();
                    set.remove(&b);
                }
            },
        }
    }
    out_line!("Yes");
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
