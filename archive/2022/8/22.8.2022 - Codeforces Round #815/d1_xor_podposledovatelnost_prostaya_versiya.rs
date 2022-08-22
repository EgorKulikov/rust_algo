//{"name":"D1. Xor-подпоследовательность (простая версия)","group":"Codeforces - Codeforces Round #815 (Div. 2)","url":"https://codeforces.com/contest/1720/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n5\n5 2 4 3 1\n10\n3 8 8 2 9 1 6 2 8 3\n","output":"2\n3\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1XorPodposledovatelnostProstayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    #[derive(Default)]
    struct Node {
        max00: usize,
        max01: usize,
        max10: usize,
        max11: usize,
        xor0: Option<Box<Node>>,
        xor1: Option<Box<Node>>,
    }

    impl Node {
        fn get_max(&self, val: usize, index: usize, height: usize) -> usize {
            let res = if val.is_set(height) {
                if index.is_set(height) {
                    self.max10
                } else {
                    self.max00
                }
            } else {
                if index.is_set(height) {
                    self.max11
                } else {
                    self.max01
                }
            };
            if height != 0 {
                if index.is_set(height) == val.is_set(height) {
                    if self.xor0.is_some() {
                        res.max(self.xor0.as_ref().unwrap().get_max(val, index, height - 1))
                    } else {
                        res
                    }
                } else {
                    if self.xor1.is_some() {
                        res.max(self.xor1.as_ref().unwrap().get_max(val, index, height - 1))
                    } else {
                        res
                    }
                }
            } else {
                res
            }
        }

        fn add(&mut self, val: usize, index: usize, height: usize, ans: usize) {
            if val.is_set(height) {
                if index.is_set(height) {
                    self.max11 = self.max11.max(ans);
                } else {
                    self.max10 = self.max10.max(ans);
                }
            } else {
                if index.is_set(height) {
                    self.max01 = self.max01.max(ans);
                } else {
                    self.max00 = self.max00.max(ans);
                }
            }
            if height != 0 {
                if index.is_set(height) == val.is_set(height) {
                    if self.xor0.is_none() {
                        self.xor0 = Some(Box::new(Node::default()));
                    }
                    self.xor0.as_mut().unwrap().add(val, index, height - 1, ans);
                } else {
                    if self.xor1.is_none() {
                        self.xor1 = Some(Box::new(Node::default()));
                    }
                    self.xor1.as_mut().unwrap().add(val, index, height - 1, ans);
                }
            }
        }
    }

    let mut root = Node::default();
    let mut ans = 0;
    const HEIGHT: usize = 29;
    for (i, a) in a.into_iter().enumerate() {
        let cur = root.get_max(a, i, HEIGHT) + 1;
        ans = ans.max(cur);
        root.add(a, i, HEIGHT, cur);
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
