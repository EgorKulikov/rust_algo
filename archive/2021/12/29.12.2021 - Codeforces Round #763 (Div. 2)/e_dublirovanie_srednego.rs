//{"name":"E. Дублирование среднего","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"https://codeforces.com/contest/1623/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4 3\nabab\n2 3\n0 0\n0 4\n0 0\n","output":"baaaab\n"},{"input":"8 2\nkadracyn\n2 5\n3 4\n0 0\n0 0\n6 8\n0 7\n0 0\n0 0\n","output":"daarkkcyan\n"},{"input":"8 3\nkdaracyn\n2 5\n0 3\n0 4\n0 0\n6 8\n0 7\n0 0\n0 0\n","output":"darkcyan\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDublirovanieSrednego"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k = input.read();
    let c: Str = input.read();
    let vertices: Vec<(usize, usize)> = input.read_vec(n);

    struct Node {
        l: Option<Box<Node>>,
        r: Option<Box<Node>>,
        c: u8,
        should_dupl: bool,
        will_dupl: bool,
    }

    impl Node {
        fn build_should_dupl(&mut self, mut to_left: u8, mut to_left_should: bool) -> (u8, bool) {
            if let Some(r) = &mut self.r {
                let call = r.build_should_dupl(to_left, to_left_should);
                to_left = call.0;
                to_left_should = call.1;
            }
            if to_left > self.c || to_left == self.c && to_left_should {
                self.should_dupl = true;
                to_left_should = true;
            } else {
                to_left_should = false;
            }
            to_left = self.c;
            if let Some(l) = &mut self.l {
                let call = l.build_should_dupl(to_left, to_left_should);
                to_left = call.0;
                to_left_should = call.1;
            }
            (to_left, to_left_should)
        }

        fn solve(&mut self, rem: usize) -> usize {
            if rem == 0 {
                return 0;
            }
            if self.should_dupl {
                self.will_dupl = true;
                let mut res = 1;
                if let Some(l) = &mut self.l {
                    res += l.solve(rem - res);
                }
                if let Some(r) = &mut self.r {
                    res += r.solve(rem - res);
                }
                res
            } else {
                match self.l.as_mut() {
                    None => 0,
                    Some(l) => {
                        let call = l.solve(rem - 1);
                        if call == 0 {
                            0
                        } else {
                            let mut res = call + 1;
                            self.will_dupl = true;
                            if let Some(r) = &mut self.r {
                                res += r.solve(rem - res);
                            }
                            res
                        }
                    }
                }
            }
        }

        fn build_s(&self, s: &mut Str) {
            if let Some(l) = &self.l {
                l.build_s(s);
            }
            *s += self.c;
            if self.will_dupl {
                *s += self.c;
            }
            if let Some(r) = &self.r {
                r.build_s(s);
            }
        }
    }

    let mut build = RecursiveFunction::new(|f, at: usize| -> Node {
        Node {
            l: if vertices[at].0 == 0 {
                None
            } else {
                Some(Box::new(f.call(vertices[at].0 - 1)))
            },
            r: if vertices[at].1 == 0 {
                None
            } else {
                Some(Box::new(f.call(vertices[at].1 - 1)))
            },
            c: c[at],
            should_dupl: false,
            will_dupl: false,
        }
    });
    let mut root = build.call(0);
    root.build_should_dupl(b'a', false);
    let res = root.solve(k);
    let mut s = Str::with_capacity(n + res);
    root.build_s(&mut s);
    assert_eq!(s.len(), n + res);
    out_line!(s);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
