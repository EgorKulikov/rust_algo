//{"name":"L. Frog","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/L/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n0 0\n0 90\n180 0\n","output":"0\n1.0000000000 0.0000000000\n2\n1.0000000000 0.0000000000\n1.0000000000 1.0000000000\n0.0000000000 1.0000000000\n4\n-1.0000000000 0.0000000000\n-1.0000000000 -1.0000000000\n-0.0000000000 -1.0000000000\n1.0000000000 -1.0000000000\n1.0000000000 -0.0000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LFrog"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, Writable};
use algo_lib::out_line;
use std::f64::consts::PI;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let mut ds = input.read_float();
    let mut dt = input.read_float();

    if dt < ds {
        dt += 360.;
    }
    let rev = if dt <= ds + 180. {
        false
    } else {
        ds += 360.;
        swap(&mut ds, &mut dt);
        true
    };

    const SQ2: f64 = std::f64::consts::SQRT_2;

    #[derive(Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        fn from_angle(ang: f64) -> Self {
            Self::from_angle_mult(ang, 1.)
        }

        fn from_angle_mult(ang: f64, mul: f64) -> Self {
            Self::new(
                f64::cos(PI * ang / 180.) * mul,
                f64::sin(PI * ang / 180.) * mul,
            )
        }

        fn dist(&self, to: Point) -> f64 {
            f64::hypot(self.x - to.x, self.y - to.y)
        }

        fn add(&self, p: Point) -> Self {
            Self::new(self.x + p.x, self.y + p.y)
        }
    }

    impl Writable for Point {
        fn write(&self, output: &mut Output) {
            self.x.write(output);
            ' '.write(output);
            self.y.write(output);
        }
    }

    let start = Point::from_angle(ds);
    let end = Point::from_angle(dt);

    let mut ans = Vec::new();
    ans.push(start);

    let mut do_solve = || {
        if ds == dt {
            return;
        }
        if dt <= ds + 90. {
            let dm = (ds + dt) / 2.;
            let mut l = 1.;
            let mut r = 2.;
            for _ in 0..100 {
                let mid = (l + r) / 2.;
                let mp = Point::from_angle_mult(dm, mid);
                if mp.dist(start) > 1. {
                    r = mid;
                } else {
                    l = mid;
                }
            }
            ans.push(Point::from_angle_mult(dm, (l + r) / 2.));
            return;
        }
        let out_s = Point::from_angle_mult(ds + 45., SQ2);
        let out_t = Point::from_angle_mult(dt - 45., SQ2);
        if out_s.dist(out_t) <= 1. {
            let mut l = 0.;
            let mut r = 45.;
            for _ in 0..100 {
                let mid = (l + r) / 2.;
                let m_out_s = start.add(Point::from_angle(ds + 90. - mid));
                let m_out_t = end.add(Point::from_angle(dt - 90. + mid));
                if m_out_s.dist(m_out_t) > 1. {
                    r = mid;
                } else {
                    l = mid;
                }
            }
            ans.push(start.add(Point::from_angle(ds + 90. - (l + r) / 2.)));
            ans.push(end.add(Point::from_angle(dt - 90. + (l + r) / 2.)));
            return;
        }
        let dm = (ds + dt) / 2.;
        let mut l =
            Point::new((out_s.x + out_t.x) / 2., (out_s.y + out_t.y) / 2.).dist(Point::new(0., 0.));
        let mut r = l + 1.;
        for _ in 0..100 {
            let mid = (l + r) / 2.;
            let mp = Point::from_angle_mult(dm, mid);
            if mp.dist(out_s) > 1. {
                r = mid;
            } else {
                l = mid;
            }
        }
        ans.push(out_s);
        ans.push(Point::from_angle_mult(dm, (l + r) / 2.));
        ans.push(out_t);
    };
    do_solve();
    if ans.len() != 1 {
        ans.push(end);
    }
    if rev {
        ans.reverse();
    }
    out_line!(ans.len() - 1);
    output().print_per_line(&ans);
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
