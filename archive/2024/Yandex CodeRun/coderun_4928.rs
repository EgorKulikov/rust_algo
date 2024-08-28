//{"name":"coderun_4928","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_4928"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::primes::sieve::primes;
use std::collections::BTreeMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n + 1);
    let b = input.read_size_vec(m + 1);
    let mut ca = input.read_int_vec(n + 1);
    let mut cb = input.read_int_vec(m + 1);

    for i in 0..n {
        let cand = ca[i];
        ca[i + 1].minim(cand);
    }
    for i in 0..m {
        let cand = cb[i];
        cb[i + 1].minim(cand);
    }

    #[derive(Default)]
    struct Node {
        q: DefaultHashMap<usize, usize>,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            self.q = left_val.q.clone();
            for (&k, &v) in right_val.q.iter() {
                self.q[k] += v;
            }
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    let p = primes::<usize>(40_000);
    let divisors = |mut n: usize| {
        let mut res = Vec::new();
        for &i in &p {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                let mut q = 0;
                while n % i == 0 {
                    q += 1;
                    n /= i;
                }
                res.push((i, q));
            }
        }
        if n != 1 {
            res.push((n, 1));
        }
        res
    };

    let mut a_tree = SegmentTree::from_generator(n + 2, |i| {
        let mut map = DefaultHashMap::new();
        if i <= n {
            let d = divisors(a[i]);
            for (k, v) in d {
                map.insert(k, v);
            }
        }
        Node { q: map }
    });

    let mut b_tree = SegmentTree::from_generator(m + 2, |i| {
        let mut map = DefaultHashMap::new();
        if i <= m {
            let d = divisors(b[i]);
            for (k, v) in d {
                map.insert(k, v);
            }
        }
        Node { q: map }
    });

    for _ in 0..q {
        let k = input.read_size();
        let x = input.read_size_vec(k);
        let mut restrictions = BTreeMap::<usize, usize>::new();
        for x in x {
            let d = divisors(x);
            // eprintln!("d = {:?}", d);
            let mut map = DefaultHashMap::new();
            for &(k, v) in &d {
                map.insert(k, v);
            }
            let id = a_tree.binary_search(
                |left, _| {
                    for (&k, &v) in map.iter() {
                        if left.q[k] < v {
                            for (&k, v) in map.iter_mut() {
                                *v = v.saturating_sub(left.q[k]);
                            }
                            return Direction::Right;
                        }
                    }
                    Direction::Left
                },
                |_, id| id,
            );
            let mut map = DefaultHashMap::new();
            for &(k, v) in &d {
                map.insert(k, v);
            }
            let limit = b_tree.binary_search(
                |left, _| {
                    for (&k, &v) in map.iter() {
                        if left.q[k] < v {
                            for (&k, v) in map.iter_mut() {
                                *v = v.saturating_sub(left.q[k]);
                            }
                            return Direction::Right;
                        }
                    }
                    Direction::Left
                },
                |_, id| id,
            );
            restrictions.entry(id).or_insert(usize::MAX).minim(limit);
        }
        // eprintln!("restrictions = {:?}", restrictions);
        let mut ans = None;
        let mut end = m;
        for (k, v) in restrictions {
            ans.minim(ca[k - 1] + cb[end]);
            end.minim(v - 1);
        }
        ans.minim(ca[n] + cb[end]);
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
