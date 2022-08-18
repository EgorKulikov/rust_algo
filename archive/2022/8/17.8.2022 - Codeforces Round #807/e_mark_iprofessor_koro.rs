//{"name":"E. Марк и профессор Коро","group":"Codeforces - Codeforces Round #807 (Div. 2)","url":"https://codeforces.com/contest/1705/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n2 2 2 4 5\n2 3\n5 3\n4 1\n1 4\n","output":"6\n5\n4\n5\n"},{"input":"2 1\n200000 1\n2 200000\n","output":"200001\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMarkIProfessorKoro"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let mut a = input.read_usize_vec(n);

    const LEN: usize = 300_000;
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum State {
        AllOff,
        AllOn,
        Mixed,
    }

    fn turn_impl(
        root: usize,
        left: usize,
        right: usize,
        at: usize,
        from_state: State,
        to_state: State,
        states: &mut Vec<State>,
    ) -> bool {
        if left + 1 == right {
            if states[root] == from_state {
                states[root] = to_state;
                return false;
            } else {
                states[root] = from_state;
                return true;
            }
        }
        if states[root] != State::Mixed {
            states[2 * root + 1] = states[root];
            states[2 * root + 2] = states[root];
        }
        let mid = (left + right) >> 1;
        if at < mid {
            if turn_impl(2 * root + 1, left, mid, at, from_state, to_state, states) {
                if states[2 * root + 2] == to_state {
                    states[2 * root + 2] = from_state;
                    if states[2 * root + 1] == states[2 * root + 2] {
                        states[root] = states[2 * root + 1];
                    } else {
                        states[root] = State::Mixed;
                    }
                    return true;
                } else {
                    let mut change_first =
                        RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                            if left + 1 == right {
                                states[root] = to_state;
                                return;
                            }
                            if states[root] != State::Mixed {
                                states[2 * root + 1] = states[root];
                                states[2 * root + 2] = states[root];
                            }
                            let mid = (left + right) >> 1;
                            if states[2 * root + 1] == to_state {
                                states[2 * root + 1] = from_state;
                                f.call(2 * root + 2, mid, right);
                            } else {
                                f.call(2 * root + 1, left, mid);
                            }
                            if states[2 * root + 1] == states[2 * root + 2] {
                                states[root] = states[2 * root + 1];
                            } else {
                                states[root] = State::Mixed;
                            }
                        });
                    change_first.call(2 * root + 2, mid, right);
                    if states[2 * root + 1] == states[2 * root + 2] {
                        states[root] = states[2 * root + 1];
                    } else {
                        states[root] = State::Mixed;
                    }
                    return false;
                }
            } else {
                if states[2 * root + 1] == states[2 * root + 2] {
                    states[root] = states[2 * root + 1];
                } else {
                    states[root] = State::Mixed;
                }
                return false;
            }
        } else {
            let up = turn_impl(2 * root + 2, mid, right, at, from_state, to_state, states);
            if states[2 * root + 1] == states[2 * root + 2] {
                states[root] = states[2 * root + 1];
            } else {
                states[root] = State::Mixed;
            }
            return up;
        }
    }

    fn turn(at: usize, to_state: State, states: &mut Vec<State>) {
        let from_state = if to_state == State::AllOn {
            State::AllOff
        } else {
            State::AllOn
        };
        turn_impl(0, 0, LEN, at, from_state, to_state, states);
    }

    fn turn_on(at: usize, state: &mut Vec<State>) {
        turn(at, State::AllOn, state);
    }

    fn turn_off(at: usize, state: &mut Vec<State>) {
        turn(at, State::AllOff, state);
    }

    let mut states = vec![State::AllOff; 1_200_000];
    let mut b = a.clone();
    b.sort_unstable();
    let mut last = 0;
    let mut qty = 0;
    for i in b {
        if i == last {
            qty += 1;
        } else {
            while last < i {
                if qty % 2 == 1 {
                    turn_on(last, &mut states);
                }
                last += 1;
                qty /= 2;
            }
            qty += 1;
        }
    }
    while qty > 0 {
        if qty % 2 == 1 {
            turn_on(last, &mut states);
        }
        last += 1;
        qty /= 2;
    }
    /*out_line!("!!!");
    let mut print = RecursiveFunction3::new(
        |f, root: usize, left: usize, right: usize| match states[root] {
            State::AllOff => {}
            State::AllOn => {
                for i in left..right {
                    out_line!(i);
                }
            }
            State::Mixed => {
                let mid = (left + right) >> 1;
                f.call(2 * root + 1, left, mid);
                f.call(2 * root + 2, mid, right);
            }
        },
    );
    print.call(0, 0, LEN);
    out_line!("---");*/
    for _ in 0..q {
        let pos = input.read_usize() - 1;
        let n_value = input.read_usize();
        turn_off(a[pos], &mut states);
        /*out_line!("!!!");
        let mut print = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            match states[root] {
                State::AllOff => {}
                State::AllOn => {
                    for i in left..right {
                        out_line!(i);
                    }
                }
                State::Mixed => {
                    let mid = (left + right) >> 1;
                    f.call(2 * root + 1, left, mid);
                    f.call(2 * root + 2, mid, right);
                }
            }
        });
        print.call(0, 0, LEN);
        out_line!("---");*/
        a[pos] = n_value;
        turn_on(a[pos], &mut states);

        /*out_line!("!!!");
        let mut print = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            match states[root] {
                State::AllOff => {}
                State::AllOn => {
                    for i in left..right {
                        out_line!(i);
                    }
                }
                State::Mixed => {
                    let mid = (left + right) >> 1;
                    f.call(2 * root + 1, left, mid);
                    f.call(2 * root + 2, mid, right);
                }
            }
        });
        print.call(0, 0, LEN);
        out_line!("---");*/
        let mut root = 0;
        let mut left = 0;
        let mut right = LEN;
        loop {
            if states[root] == State::AllOn {
                out_line!(right - 1);
                break;
            }
            assert_eq!(states[root], State::Mixed);
            let mid = (left + right) >> 1;
            if states[2 * root + 2] == State::AllOff {
                root = 2 * root + 1;
                right = mid;
            } else {
                root = 2 * root + 2;
                left = mid;
            }
        }
    }
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
