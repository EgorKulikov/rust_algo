//{"name":"E. Composition of linear functions","group":"Yandex - Yandex Cup 2023 — Algorithm — Qualification","url":"https://contest.yandex.com/contest/54452/problems/E/","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n1 1\n1 2\n1 3\n1 2\n","output":"1000000001\n1000000002\n1000000005\n"},{"input":"3\n2 1\n3 2\n4 1\n1 1\n","output":"541666670\n583333337\n750000005\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECompositionOfLinearFunctions"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::treap_map::TreapSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    type Mod = ModInt7;

    let n = input.read_size();
    let f = input.read_vec::<(Mod, Mod)>(n);
    let c = input.read_size_vec(n - 1).dec();

    let mut id = unsafe { TreapSet::from_sorted(0..n) };

    #[derive(Clone, Copy)]
    struct Node {
        a: Mod,
        b: Mod,
        update: bool,
    }
    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                a: Mod::one(),
                b: Mod::zero(),
                update: false,
            }
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            assert!(!left_val.update);
            assert!(!right_val.update);
            assert!(!self.update);
            self.a = left_val.a * right_val.a;
            self.b = left_val.a * right_val.b + left_val.b;
        }

        fn accumulate(&mut self, value: &Self, _left: usize, _right: usize) {
            if value.update {
                self.a = value.a;
                self.b = value.b;
            }
        }

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }
    let mut st = SegmentTree::from_generator(n, |i| Node {
        a: f[i].0,
        b: f[i].1,
        update: false,
    });
    let print = |st: &mut SegmentTree<Node>, out: &mut Output| {
        let f = st.query(..);
        if f.a == Mod::zero() {
            out.print_line(-1);
        } else {
            out.print_line(-f.b / f.a);
        }
    };
    print(&mut st, out);
    for i in c {
        let dx = *id.get_at(i).unwrap();
        id.remove(&dx);
        st.point_update(
            dx,
            &Node {
                a: Mod::one(),
                b: Mod::zero(),
                update: true,
            },
        );
        print(&mut st, out);
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
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
