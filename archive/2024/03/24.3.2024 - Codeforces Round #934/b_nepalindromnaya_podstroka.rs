//{"name":"B. Непалиндромная подстрока","group":"Codeforces - Codeforces Round 934 (Div. 1)","url":"https://codeforces.com/contest/1943/problem/B","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n4 4\naaab\n1 4\n1 3\n3 4\n2 4\n3 2\nabc\n1 3\n1 2\n5 4\npqpcc\n1 5\n4 5\n1 3\n2 4\n2 1\naa\n1 2\n12 1\nsteponnopets\n1 12\n","output":"9\n0\n2\n5\n5\n2\n14\n0\n2\n5\n0\n65\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNepalindromnayaPodstroka"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::palindromes::Palindromes;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let _n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    let odd = s.odd_palindromes();
    let even = s.even_palindromes();
    #[derive(Clone)]
    struct Node(usize);
    impl Node {
        fn new(val: usize) -> Self {
            Self(val)
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self(0)
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            self.0 = left_val.0.min(right_val.0);
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    let mut st_odd = SegmentTree::from_generator(odd.len(), |pos| Node::new(odd[pos]));
    let mut st_even = SegmentTree::from_generator(even.len(), |pos| Node::new(even[pos]));

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();

        let mut left = 1;
        let mut right = (r - l + 1) / 2;
        while left < right {
            let mid = (left + right + 1) / 2;
            if st_odd.query(l + mid - 1..r - mid + 1).0 >= mid {
                right = mid - 1;
            } else {
                left = mid;
            }
        }
        let mut ans = left * left - 1;
        let mut left = 0;
        let mut right = (r - l) / 2;
        while left < right {
            let mid = (left + right + 1) / 2;
            if st_even.query(l + mid..=r - mid).0 >= mid {
                right = mid - 1;
            } else {
                left = mid;
            }
        }
        ans += left * (left + 1);
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
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
