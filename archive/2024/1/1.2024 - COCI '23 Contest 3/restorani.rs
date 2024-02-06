//{"name":"#4 - Restorani","group":"DMOJ - COCI '23 Contest 3","url":"https://dmoj.ca/problem/coci23c3p4","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1\n2\n3\n1 2\n1 3\n","output":"4\n1 1\n"},{"input":"9 4\n2 3 4 6\n4 5 8 9\n1 2\n1 3\n3 4\n3 5\n5 6\n1 7\n7 8\n7 9\n","output":"18\n3 1 4 2 2 4 1 3\n"},{"input":"10 5\n3 5 6 7 8\n1 2 4 9 10\n1 2\n2 3\n3 4\n4 5\n5 6\n6 7\n7 8\n8 9\n9 10\n","output":"24\n4 4 5 5 3 3 2 2 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Restorani"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use std::collections::VecDeque;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(m).dec();
    let b = input.read_size_vec(m).dec();
    let edges = input.read_size_pair_vec(n - 1).dec();

    enum State {
        AToA(Vec<VecDeque<usize>>),
        BToB(Vec<VecDeque<usize>>),
        AToB(VecDeque<usize>),
        Empty,
    }

    fn join(mut left: VecDeque<usize>, mut right: VecDeque<usize>) -> VecDeque<usize> {
        if left.len() < right.len() {
            for i in left.into_iter().rev() {
                right.push_front(i);
            }
            right
        } else {
            for i in right.into_iter() {
                left.push_back(i);
            }
            left
        }
    }

    impl State {
        fn score(&self) -> usize {
            2 * match self {
                State::AToA(a) => a.len(),
                State::BToB(b) => b.len(),
                State::AToB(_) => 1,
                State::Empty => 0,
            }
        }

        fn join(&mut self, mut other: Self) {
            match self {
                State::Empty => {
                    *self = other;
                }
                State::AToA(a) => match other {
                    State::AToA(mut oa) => {
                        if a.len() < oa.len() {
                            swap(a, &mut oa);
                        }
                        a.extend_from_slice(&oa);
                    }
                    State::BToB(mut ob) => {
                        if a.len() < ob.len() {
                            let mut cur = ob.pop().unwrap();
                            while let Some(c) = a.pop() {
                                cur = join(cur, c);
                                cur = join(cur, ob.pop().unwrap());
                            }
                            ob.push(cur);
                            *self = State::BToB(ob);
                        } else {
                            let mut cur = a.pop().unwrap();
                            while let Some(c) = ob.pop() {
                                cur = join(cur, c);
                                if let Some(c) = a.pop() {
                                    cur = join(cur, c);
                                } else {
                                    *self = State::AToB(cur);
                                    return;
                                }
                            }
                            a.push(cur);
                        }
                    }
                    State::AToB(oab) => {
                        let c = a.pop().unwrap();
                        let c = join(oab, c);
                        a.push(c);
                    }
                    _ => {
                        swap(self, &mut other);
                        self.join(other);
                    }
                },
                State::BToB(b) => match other {
                    State::BToB(mut ob) => {
                        if b.len() < ob.len() {
                            swap(b, &mut ob);
                        }
                        b.extend_from_slice(&ob);
                    }
                    State::AToB(oab) => {
                        let c = b.pop().unwrap();
                        let c = join(c, oab);
                        b.push(c);
                    }
                    _ => {
                        swap(self, &mut other);
                        self.join(other);
                    }
                },
                State::AToB(ab) => match other {
                    State::AToB(oab) => {
                        let mut c = VecDeque::new();
                        swap(ab, &mut c);
                        let c = join(c, oab);
                        *self = State::AToB(c);
                    }
                    _ => {
                        swap(self, &mut other);
                        self.join(other);
                    }
                },
            }
        }
    }

    let graph = Graph::from_biedges(n, &edges);
    let mut aa = vec![Vec::new(); n];
    let mut bb = vec![Vec::new(); n];
    for i in 0..m {
        aa[a[i]].push(i + 1);
        bb[b[i]].push(i + 1);
    }

    let mut t = 0;

    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        let mut aa = if aa[vert].is_empty() {
            State::Empty
        } else {
            State::AToA(aa[vert].iter().map(|&i| vec![i].into()).collect())
        };
        let bb = if bb[vert].is_empty() {
            State::Empty
        } else {
            State::BToB(bb[vert].iter().map(|&i| vec![i].into()).collect())
        };
        aa.join(bb);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            aa.join(call);
        }
        t += aa.score();
        aa
    });

    let ans = match dfs.call(0, 0) {
        State::AToB(ab) => ab,
        _ => unreachable!(),
    };
    out.print_line(t - 2);
    out.print_iter(ans.into_iter());
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
    //    tester::stress_test();
}
//END MAIN
