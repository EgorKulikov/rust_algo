//{"name":"G - Replace","group":"AtCoder - AtCoder Beginner Contest 261","url":"https://atcoder.jp/contests/abc261/tasks/abc261_g","interactive":false,"timeLimit":2000,"tests":[{"input":"ab\ncbca\n3\na b\nb ca\na efg\n","output":"4\n"},{"input":"a\naaaaa\n2\na aa\na aaa\n","output":"2\n"},{"input":"a\nz\n1\na abc\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GReplace"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::all_distances::AllDistances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::HashMap;

fn solve(input: &mut Input) {
    let s: Str = input.read();
    let t: Str = input.read();
    let k = input.read_usize();
    let op = input.read_vec::<(char, Str)>(k);

    let mut graph = Graph::new(26);
    let mut ops = Vec::new();
    for (c, s) in op {
        let id = (c as usize) - ('a' as usize);
        if s.len() == 1 {
            graph.add_edge(id, WeightedEdge::new((s[0] - b'a').into_usize(), 1));
        } else {
            ops.push((id, s));
        }
    }
    let dist = graph.all_distances();

    struct Solver {
        string_to_string: HashMap<(Vec<u8>, usize, usize), Option<i32>>,
        letter_to_string: HashMap<(usize, usize, usize), Option<i32>>,
        letter_to_letter: Arr2d<i32>,
        t: Str<'static>,
        ops: Vec<(usize, Str<'static>)>,
    }

    let len = t.len();

    let mut solver = Solver {
        string_to_string: Default::default(),
        letter_to_string: Default::default(),
        letter_to_letter: dist,
        t,
        ops,
    };

    impl Solver {
        fn string_to_string(&mut self, s: &[u8], i: usize, j: usize) -> Option<i32> {
            match self.string_to_string.get(&(s.to_vec(), i, j)) {
                Some(&res) => res,
                None => {
                    let mut res = None;
                    if s.is_empty() {
                        if i == j {
                            res.minim(0);
                        }
                    } else if j - i >= s.len() {
                        for k in i + 1..=(j - s.len() + 1) {
                            let left = self.letter_to_string((s[0] - b'a').into_usize(), i, k);
                            if let Some(left) = left {
                                let str = &s[1..];
                                let right = self.string_to_string(&str, k, j);
                                if let Some(right) = right {
                                    res.minim(left + right);
                                }
                            }
                        }
                    }
                    self.string_to_string.insert((s.to_vec(), i, j), res);
                    res
                }
            }
        }

        fn letter_to_string(&mut self, l: usize, i: usize, j: usize) -> Option<i32> {
            match self.letter_to_string.get(&(l, i, j)) {
                Some(&res) => res,
                None => {
                    let mut res = None;
                    if i + 1 == j {
                        let id = (self.t[i] - b'a').into_usize();
                        if self.letter_to_letter[(l, id)] != std::i32::MAX {
                            res.minim(self.letter_to_letter[(l, id)]);
                        }
                    } else {
                        for (id, s) in self.ops.clone() {
                            if self.letter_to_letter[(l, id)] != std::i32::MAX {
                                let call = self.string_to_string(s.as_slice(), i, j);
                                if let Some(call) = call {
                                    res.minim(self.letter_to_letter[(l, id)] + call + 1);
                                }
                            }
                        }
                    }
                    self.letter_to_string.insert((l, i, j), res);
                    res
                }
            }
        }
    }

    out_line!(solver.string_to_string(s.as_slice(), 0, len));
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
