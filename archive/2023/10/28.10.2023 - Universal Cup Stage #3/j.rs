//{"name":"j","group":"Manual","url":"","interactive":true,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::compress::compress;
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::UpperDiv;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    enum Query {
        Add(usize, usize, i32),
        Remove(usize, usize, i32),
        Get(usize),
    }

    impl Readable for Query {
        fn read(input: &mut Input) -> Self {
            let tp = input.read_char();
            match tp {
                '+' => {
                    let l = input.read_size() - 1;
                    let r = input.read_size();
                    let x = input.read();
                    Query::Add(l, r, x)
                }
                '-' => {
                    let l = input.read_size() - 1;
                    let r = input.read_size();
                    let x = input.read();
                    Query::Remove(l, r, x)
                }
                '?' => {
                    let x = input.read_size() - 1;
                    Query::Get(x)
                }
                _ => unreachable!(),
            }
        }
    }

    let _n = input.read_size();
    let q = input.read_size();
    let mut queries = Vec::with_capacity(q);

    let mut poi = Vec::new();
    poi.push(0);

    #[derive(Default)]
    struct Node {
        added: HashSet<i32>,
        removed: HashSet<i32>,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(
            &mut self,
            _left_val: &Self,
            _right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
        }

        fn accumulate(&mut self, value: &Self, _left: usize, _right: usize) {
            for &i in &value.removed {
                self.added.remove(&i);
                self.removed.insert(i);
            }
            for &i in &value.added {
                self.added.insert(i);
                self.removed.remove(&i);
            }
        }

        fn reset_delta(&mut self, _left: usize, _right: usize) {
            self.added.clear();
            self.removed.clear();
        }
    }

    impl Pushable<&Query> for Node {
        fn push(&mut self, delta: &Query, _left: usize, _right: usize) {
            match delta {
                Query::Add(_, _, x) => {
                    self.removed.remove(x);
                    self.added.insert(*x);
                }
                Query::Remove(_, _, x) => {
                    self.added.remove(x);
                    self.removed.insert(*x);
                }
                _ => {}
            }
        }
    }
    fn build_st(poi: &[usize], queries: &[Query]) -> SegmentTree<Node> {
        let mut st = SegmentTree::new(poi.len());
        for q in queries {
            match q {
                Query::Add(l, r, _) | Query::Remove(l, r, _) => {
                    let l = poi.lower_bound(l);
                    let r = poi.lower_bound(r);
                    st.update(l..r, q);
                }
                _ => {}
            }
        }
        st
    }

    const BUBEN: usize = 5000;

    let mut poi = vec![0];
    for i in 0..q.upper_div(BUBEN) {
        let mut st = build_st(&poi, &queries);
        let mut new_poi = Vec::new();
        let start = queries.len();
        for _ in i * BUBEN..((i + 1) * BUBEN).min(q) {
            let query = input.read();
            match query {
                Query::Get(x) => {
                    let pos = poi.upper_bound(&x) - 1;
                    let node = st.point_query(pos);
                    let mut to_add = HashSet::new();
                    let mut to_remove = HashSet::new();
                    for q in queries.iter().skip(start) {
                        match q {
                            Query::Add(l, r, v) => {
                                if *l <= x && x < *r {
                                    to_remove.remove(v);
                                    if !node.added.contains(v) {
                                        to_add.insert(*v);
                                    }
                                }
                            }
                            Query::Remove(l, r, v) => {
                                if *l <= x && x < *r {
                                    to_add.remove(v);
                                    if node.added.contains(v) {
                                        to_remove.insert(*v);
                                    }
                                }
                            }
                            Query::Get(_) => {}
                        }
                    }
                    out.print_line(node.added.len() + to_add.len() - to_remove.len());
                }
                Query::Add(l, r, _) | Query::Remove(l, r, _) => {
                    new_poi.push(l);
                    new_poi.push(r);
                    queries.push(query);
                }
            }
        }
        let (p, _) = compress([&poi, &new_poi]);
        poi = p;
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
    // input.skip_whitespace();
    // input.peek().is_none()
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
