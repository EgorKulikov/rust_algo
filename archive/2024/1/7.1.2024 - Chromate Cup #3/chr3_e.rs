//{"name":"chr3_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"chr3_e"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::md_arr::arr4d::Arr4d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::slice_ext::reversed::BackwardSlice;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::segment::Segment;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::real::{IntoReal, Real, RealTrait};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let pts = input.read_long_pair_vec(n);

    let pts = pts
        .into_iter()
        .map(|(x, y)| Point::new(Rational::from(x), Rational::from(y)))
        .collect_vec();
    let segments = Arr2d::generate(n, n, |i, j| Segment::new(pts[i], pts[j]));
    let len = Arr2d::generate(n, n, |i, j| {
        segments[i][j].square_len().num().into_real().sqrt()
    });
    let bad = Arr4d::generate(n, n, n, n, |i, j, k, l| {
        if i == j || i == k || i == l || j == k || j == l || k == l {
            false
        } else {
            segments[(i, j)]
                .intersect_segment(segments[(k, l)])
                .is_some()
        }
    });
    let mut ans = None;
    let check = |i: usize, j: usize, v: &[usize]| {
        for (&k, &l) in v.consecutive_iter() {
            if bad[(i, j, k, l)] {
                return false;
            }
        }
        true
    };
    let mut rec = RecursiveFunction3::new(|rec, mut v: Vec<usize>, mask: u32, length: Real| {
        if mask == 0 {
            if check(v[0], v.backward()[0], &v) {
                ans.minim(length + len[(v[0], v.backward()[0])]);
            }
            return;
        }
        for i in 0..n {
            if mask.is_set(i) && check(i, v.backward()[0], &v) {
                v.push(i);
                rec.call(
                    v.clone(),
                    mask.without_bit(i),
                    length + len[(i, v.backward()[1])],
                );
                v.pop();
            }
        }
    });
    rec.call(vec![0], u32::all_bits(n).without_bit(0), Real::zero());
    out.print_line(ans);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
