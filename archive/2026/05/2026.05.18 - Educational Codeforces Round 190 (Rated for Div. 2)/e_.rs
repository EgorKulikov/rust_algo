//{"name":"E. Минимальное влияние","group":"Codeforces - Educational Codeforces Round 190 (Rated for Div. 2)","url":"https://codeforces.com/contest/2230/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 4 1 6 0 10\n3 2 6 1 9 0\n5\n0 0 0 1 5\n0 0 9 5 2\n9 4 8 2 2\n","output":"5\n4\n1\n2\n2\n"},{"input":"5\n75 19 53 12 10\n34 75 67 84 95\n5\n55 14 46 97 14\n78 61 56 23 33\n10 4 7 11 3\n","output":"0\n18\n53\n34\n36\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n);
    let c = input.read_size_vec(n);
    let m = input.read_size();
    let tp = input.read_size_vec(m);
    let tc = input.read_size_vec(m);
    let d = input.read_size_vec(m);

    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    struct Request {
        pos: usize,
        typ: RequestType,
    }

    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    enum QueryType {
        X,
        Y,
        Any,
    }

    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    enum RequestType {
        Query {
            delta: usize,
            limit: usize,
            qt: QueryType,
            from: usize,
            to: usize,
            id: usize,
        },
        Point(usize),
    }

    #[derive(Default, Clone)]
    struct Node {
        x: Option<usize>,
        y: Option<usize>,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.x = left_val.x;
            if let Some(val) = right_val.x {
                self.x.minim(val);
            }
            self.y = left_val.y;
            if let Some(val) = right_val.y {
                self.y.minim(val);
            }
        }
    }

    let min_sum = p.copy_zip(&c).map(|(a, b)| a + b).min().unwrap();
    let mut ans = Vec::with_gen(m, |i| (tp[i] + tc[i] + 2 * d[i]).min(min_sum));

    let mut requests = Vec::new();
    for i in 0..n {
        requests.push(Request {
            pos: p[i],
            typ: RequestType::Point(c[i]),
        });
    }
    for i in 0..m {
        requests.push(Request {
            pos: tp[i],
            typ: RequestType::Query {
                delta: 0,
                limit: 0,
                qt: QueryType::Any,
                from: 0,
                to: tc[i],
                id: i,
            },
        });
        requests.push(Request {
            pos: tp[i],
            typ: RequestType::Query {
                delta: 0,
                limit: tc[i] + d[i],
                qt: QueryType::Y,
                from: tc[i],
                to: 1_000_001,
                id: i,
            },
        });
        requests.push(Request {
            pos: 1_000_001,
            typ: RequestType::Query {
                delta: 0,
                limit: tp[i] + d[i],
                qt: QueryType::X,
                from: 0,
                to: tc[i],
                id: i,
            },
        });
        requests.push(Request {
            pos: 1_000_001,
            typ: RequestType::Query {
                delta: tc[i] + d[i],
                limit: tp[i] + tc[i] + 2 * d[i],
                qt: QueryType::X,
                from: tc[i] + d[i],
                to: 1_000_001,
                id: i,
            },
        });
        requests.push(Request {
            pos: 1_000_001,
            typ: RequestType::Query {
                delta: tp[i] + d[i],
                limit: tp[i] + tc[i] + 2 * d[i],
                qt: QueryType::Y,
                from: tc[i],
                to: 1_000_001,
                id: i,
            },
        });
    }
    requests.sort();
    let mut st = SegmentTree::<Node>::new(1_000_001);
    for request in requests {
        match request.typ {
            RequestType::Point(c) => {
                st.for_each_mut(c..=c, |_, node| {
                    node.x.minim(request.pos);
                    node.y.minim(c);
                });
                // let mut node = st.point_query(c).clone();
                // node.x.minim(request.pos);
                // node.y.minim(c);
                // node.sum.minim(request.pos + c);
                // st.point_update(c, node);
            }
            RequestType::Query {
                delta,
                limit,
                qt,
                from,
                to,
                id,
            } => {
                let res = st.query(from..to);
                let val = match qt {
                    QueryType::X => res.x,
                    QueryType::Y => res.y,
                    QueryType::Any => res.x.map(|_| 0),
                };
                if let Some(mut val) = val {
                    val.minim(limit);
                    val += delta;
                    ans[id].minim(val);
                }
            }
        }
    }
    out.print_per_line(&ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
