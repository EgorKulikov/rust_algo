//{"name":"E. Concave Hull","group":"Universal Cup - The 3rd Universal Cup. Stage 23: Hong Kong","url":"https://contest.ucup.ac/contest/1885/problem/9919","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n0 0\n2 0\n1 2\n1 1\n","output":"8\n"},{"input":"15\n3442 3341\n3136 3120\n3228 3113\n3143 2981\n3050 3052\n2970 2973\n2964 3011\n2921 2927\n2844 2715\n2655 2661\n2666 2637\n2755 2731\n2657 2684\n2662 2629\n2542 2508\n","output":"23993862\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EConcaveHull"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::polygon::ConvexHull;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut pts: Vec<Point<i64>> = input.read_vec(n);

    let hull = pts.convex_hull();
    let mut inner = Vec::new();
    for pt in pts {
        if !hull.points.contains(&pt) {
            inner.push(pt);
        }
    }
    type Mod = ModInt7;
    let mut ans = Mod::zero();
    let area = hull.double_area().abs();
    for i in inner.indices() {
        let mut others = inner.clone();
        others.swap_remove(i);
        let to = inner[i];
        let from = hull.points[0];
        let base = {
            let x = from.x - to.x;
            let y = from.y - to.y;
            if x > 0 && y >= 0 {
                (0, Rational::new(y, x))
            } else if y > 0 && x <= 0 {
                (1, Rational::new(-x, y))
            } else if x < 0 && y <= 0 {
                (2, Rational::new(-y, -x))
            } else if y < 0 && x >= 0 {
                (3, Rational::new(x, -y))
            } else {
                unreachable!();
            }
        };
        let key = Vec::gen(others.len(), |j, _| {
            let pt = others[j];
            let x = pt.x - inner[i].x;
            let y = pt.y - inner[i].y;
            let res = if x > 0 && y >= 0 {
                (0, Rational::new(y, x))
            } else if y > 0 && x <= 0 {
                (1, Rational::new(-x, y))
            } else if x < 0 && y <= 0 {
                (2, Rational::new(-y, -x))
            } else if y < 0 && x >= 0 {
                (3, Rational::new(x, -y))
            } else {
                unreachable!();
            };
            if res < base {
                (res.0 + 4, res.1)
            } else {
                res
            }
        });
        let mut order = (0..others.len()).collect::<Vec<_>>();
        order.sort_by_key(|&i| key[i]);
        // others.sort_by_key(|&pt| {
        //     let x = pt.x - inner[i].x;
        //     let y = pt.y - inner[i].y;
        //     let res = if x > 0 && y >= 0 {
        //         (0, Rational::new(y, x))
        //     } else if y > 0 && x <= 0 {
        //         (1, Rational::new(-x, y))
        //     } else if x < 0 && y <= 0 {
        //         (2, Rational::new(-y, -x))
        //     } else if y < 0 && x >= 0 {
        //         (3, Rational::new(x, -y))
        //     } else {
        //         unreachable!();
        //     };
        //     if res < base {
        //         (res.0 + 4, res.1)
        //     } else {
        //         res
        //     }
        // });
        let mut pts = vec![Vec::new(); hull.points.len()];
        let mut side = 0;
        for z in order {
            let pt = others[z];
            let line = inner[i].line(pt);
            loop {
                let v1 = line.value(hull.points[side]);
                let v2 = line.value(hull.points[(side + 1) % hull.points.len()]);
                if v2 < 0 && v1 > 0 {
                    break;
                }
                side = (side + hull.points.len() - 1) % hull.points.len();
            }
            pts[side].push(pt);
        }
        for j in hull.points.indices() {
            let mut add1 = Vec::with_capacity(pts[j].len() + 1);
            let from = hull.points[(j + 1) % hull.points.len()];
            let to = inner[i];
            let mut c_area: i64 = 0;
            add1.push(c_area);
            // let mut pts1 = vec![from, to];
            let mut v = Vec::new();
            /*let base = {
                let x = from.x - to.x;
                let y = from.y - to.y;
                if x > 0 && y >= 0 {
                    (0, Rational::new(y, x))
                } else if y > 0 && x <= 0 {
                    (1, Rational::new(-x, y))
                } else if x < 0 && y <= 0 {
                    (2, Rational::new(-y, -x))
                } else if y < 0 && x >= 0 {
                    (3, Rational::new(x, -y))
                } else {
                    unreachable!();
                }
            };
            pts[j].sort_by_key(|&pt| {
                let x = pt.x - inner[i].x;
                let y = pt.y - inner[i].y;
                let res = if x > 0 && y >= 0 {
                    (0, Rational::new(y, x))
                } else if y > 0 && x <= 0 {
                    (1, Rational::new(-x, y))
                } else if x < 0 && y <= 0 {
                    (2, Rational::new(-y, -x))
                } else if y < 0 && x >= 0 {
                    (3, Rational::new(x, -y))
                } else {
                    unreachable!();
                };
                if res < base {
                    (res.0 + 4, res.1)
                } else {
                    res
                }
            });*/
            for pt in pts[j].copy_iter() {
                // pts1.push(pt);
                while let Some(&vv) = v.last() {
                    let prev = if v.len() > 1 { v[Back(1)] } else { from };
                    let line = prev.line(pt);
                    if line.value(vv) > 0 {
                        c_area -= (vv.x - prev.x) * (vv.y + prev.y);
                        c_area -= (to.x - vv.x) * (to.y + vv.y);
                        c_area += (to.x - prev.x) * (to.y + prev.y);
                        v.pop();
                    } else {
                        break;
                    }
                }
                let prev = if v.is_empty() { from } else { v[Back(0)] };
                c_area -= (to.x - prev.x) * (to.y + prev.y);
                c_area += (to.x - pt.x) * (to.y + pt.y);
                c_area += (pt.x - prev.x) * (pt.y + prev.y);
                v.push(pt);
                add1.push(c_area.abs());
                // let ch = pts1.convex_hull();
                // if ch.double_area().abs() != c_area.abs() {
                //     eprintln!("Bad!");
                //     panic!();
                // }
                // add1.push(ch.double_area().abs());
            }
            pts[j].reverse();
            let mut add2 = Vec::with_capacity(pts[j].len() + 1);
            let from = hull.points[j];
            let to = inner[i];
            let mut c_area: i64 = 0;
            add2.push(c_area);
            let mut v = Vec::new();
            // pts1 = vec![from, to];
            for pt in pts[j].copy_iter() {
                // pts1.push(pt);
                while let Some(&vv) = v.last() {
                    let prev = if v.len() > 1 { v[Back(1)] } else { from };
                    let line = prev.line(pt);
                    if line.value(vv) < 0 {
                        c_area -= (vv.x - prev.x) * (vv.y + prev.y);
                        c_area -= (to.x - vv.x) * (to.y + vv.y);
                        c_area += (to.x - prev.x) * (to.y + prev.y);
                        v.pop();
                    } else {
                        break;
                    }
                }
                let prev = if v.is_empty() { from } else { v[Back(0)] };
                c_area -= (to.x - prev.x) * (to.y + prev.y);
                c_area += (to.x - pt.x) * (to.y + pt.y);
                c_area += (pt.x - prev.x) * (pt.y + prev.y);
                v.push(pt);
                add2.push(c_area.abs());
                // let ch = pts1.convex_hull();
                // if ch.double_area().abs() != c_area.abs() {
                //     eprintln!("Bad!");
                //     panic!();
                // }
                // add2.push(ch.double_area().abs());
            }
            add2.reverse();
            let p1 = inner[i];
            let p2 = hull.points[j];
            let p3 = hull.points[(j + 1) % hull.points.len()];
            let cur_area = ((p1.x - p2.x) * (p1.y + p2.y)
                + (p2.x - p3.x) * (p2.y + p3.y)
                + (p3.x - p1.x) * (p3.y + p1.y))
                .abs();
            // let mut pp = vec![p1, p2, p3];
            // let ch = pp.convex_hull();
            // if ch.double_area().abs() != cur_area {
            //     eprintln!("Very bad!");
            //     panic!();
            // }
            for j in add1.indices() {
                let area = area + add1[j] + add2[j] - cur_area;
                ans += Mod::new_from_wide(area);
            }
        }
    }
    out.print_line(ans);
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
