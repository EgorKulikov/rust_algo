//{"name":"level_6","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::segment::Segment;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::rational::Rational;
use algo_lib::scan;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    scan!(&mut input, "@,@ @", x: i128, y: i128, tl: i128);
    scan!(&mut input, "@,@", ax: i128, ay: i128);
    drop(input);

    /*fn go(pos: i128) -> Vec<i128> {
        let mut ans = vec![1; pos.abs() as usize];
        // ans[0] = 0;
        // ans[Back(0)] = 0;
        for i in 0..5.min(ans.len()) {
            ans[i].maxim(5 - i as i128);
            ans[Back(i)].maxim(5 - i as i128);
        }
        if pos < 0 {
            ans.iter_mut().for_each(|x| *x = -(*x));
        }
        ans
    }
    fn build_pos(x_ans: &[i128], sx: i128) -> Vec<Rational<i128>> {
        let mut px = vec![Rational::new(sx, 1)];
        for i in x_ans.indices() {
            let cx = px[Back(0)];
            if x_ans[i] == 0 {
                px.push(cx);
            } else if x_ans[i] > 0 {
                for _ in 0..x_ans[i] {
                    px.push(px[Back(0)] + Rational::new(1, x_ans[i]));
                }
            } else {
                for _ in 0..-x_ans[i] {
                    px.push(px[Back(0)] - Rational::new(1, -x_ans[i]));
                }
            }
        }
        px
    }
    fn path(
        x0: i128,
        y0: i128,
        x1: i128,
        y1: i128,
    ) -> (Vec<i128>, Vec<i128>, Vec<(Rational<i128>, Rational<i128>)>, i128) {
        let mut x_ans = go(x1 - x0);
        let mut y_ans = go(y1 - y0);
        let tx = x_ans
            .copy_map(|x| if x != 0 { x.abs() } else { 1 })
            .sum::<i128>();
        let ty = y_ans
            .copy_map(|y| if y != 0 { y.abs() } else { 1 })
            .sum::<i128>();
        if tx < ty {
            x_ans.extend(repeat_n(0, (ty - tx) as usize));
        }
        if ty < tx {
            y_ans.extend(repeat_n(0, (tx - ty) as usize));
        }
        let px = build_pos(&x_ans, x0);
        let py = build_pos(&y_ans, y0);
        assert_eq!(px.len(), py.len());
        assert_eq!(px[Back(0)], Rational::new(x1, 1));
        assert_eq!(py[Back(0)], Rational::new(y1, 1));
        (
            x_ans,
            y_ans,
            px.iter().cloned().zip(py.iter().cloned()).collect(),
            tx.max(ty),
        )
    }
    let check = |path: &[(Rational<i128>, Rational<i128>)]| {
        for i in path.indices() {
            let dx = (path[i].0) - Rational::new(ax, 1);
            let dy = (path[i].1) - Rational::new(ay, 1);
            if dx.abs() < Rational::new(3, 1) && dy.abs() < Rational::new(3, 1) {
                return false;
            }
        }
        true
    };

    let mut ans_x = Vec::new();
    let mut ans_y = Vec::new();
    let mut best = i128::MAX;
    let mut process = |pts: &[(i128, i128)]| {
        let mut len = 1;
        let mut px = vec![0i128];
        let mut py = vec![0i128];
        for i in 1..pts.len() {
            let (cpx, cpy, path, cl) = path(pts[i - 1].0, pts[i - 1].1, pts[i].0, pts[i].1);
            if !check(&path) {
                return;
            }
            if !cpx.is_empty() && cpx[0] != 0 && px[Back(0)].signum() == -cpx[0].signum()
                || !cpy.is_empty() && cpy[0] != 0 && py[Back(0)].signum() == -cpy[0].signum()
            {
                px.push(0);
                py.push(0);
                len += 1;
            }
            len += cl;
            px.extend(cpx);
            py.extend(cpy);
        }
        px.push(0);
        py.push(0);
        len += 1;
        if len < best {
            best = len;
            ans_x = px;
            ans_y = py;
        }
    };
    process(&[(0, 0), (x, y)]);
    for (x0, y0) in &[
        (ax + 3, ay + 3),
        (ax - 3, ay + 3),
        (ax + 3, ay - 3),
        (ax - 3, ay - 3),
    ] {
        process(&[(0, 0), (*x0, *y0), (x, y)]);
        for (x1, y1) in &[
            (ax + 3, ay + 3),
            (ax - 3, ay + 3),
            (ax + 3, ay - 3),
            (ax - 3, ay - 3),
        ] {
            process(&[(0, 0), (*x0, *y0), (*x1, *y1), (x, y)]);
        }
    }
    eprintln!("Best = {}, tl = {}", best, tl);
    assert!(best <= tl);
    out.print_line(ans_x);
    out.print_line(ans_y);*/

    let mut graph = Graph::new(0);
    let mut id = FxHashMap::default();
    let mut wh = Vec::new();
    let mut get_id = |graph: &mut Graph<WeightedEdge<i128, ()>>,
                      x: Rational<i128>,
                      y: Rational<i128>,
                      xs: i128,
                      ys: i128| {
        if let Some(&id) = id.get(&(x, y, xs, ys)) {
            id
        } else {
            let i = graph.vertex_count();
            graph.add_vertices(1);
            id.insert((x, y, xs, ys), i);
            wh.push((x, y, xs, ys));
            i
        }
    };
    let to = |mx: i128, xs: i128, tx: i128| {
        if tx % mx != 0 {
            vec![(tx, xs)]
        } else if xs == 0 {
            vec![(tx * 5, -5), (tx, 0), (tx * 5, 5)]
        } else if xs.abs() == 1 {
            vec![(tx, xs.signum()), (2 * tx, 2 * xs.signum())]
        } else if xs.abs() == 5 {
            vec![
                (tx / 5 * 4, 4 * xs.signum()),
                (tx, 5 * xs.signum()),
                (tx / 5, 0),
            ]
        } else {
            vec![
                (tx / mx * (mx - 1), (xs - xs.signum())),
                (tx, xs),
                (tx / mx * (mx + 1), (xs + xs.signum())),
            ]
        }
    };
    let end = Point::new(Rational::new(x, 1), Rational::new(y, 1));
    let line = Segment::new(end, Point::origin());
    let fx = x.min(0) - 4;
    let fy = y.min(0) - 4;
    let ex = x.max(0) + 4;
    let ey = y.max(0) + 4;
    for xs in -5i128..=5 {
        for ys in -5i128..=5 {
            let mx = if xs != 0 { xs.abs() } else { 1 };
            let my = if ys != 0 { ys.abs() } else { 1 };
            for x in mx * fx..=mx * ex {
                for y in my * fy..=my * ey {
                    let from = get_id(
                        &mut graph,
                        Rational::new(x, mx),
                        Rational::new(y, my),
                        xs,
                        ys,
                    );
                    let tx = x + xs.signum();
                    let ty = y + ys.signum();
                    let xx = Rational::new(tx, mx);
                    let yy = Rational::new(ty, my);
                    if (xx - Rational::new(ax, 1)).abs() < Rational::new(3, 1)
                        && (yy - Rational::new(ay, 1)).abs() < Rational::new(3, 1)
                    {
                        continue;
                    }
                    if line.square_dist_point(Point::new(xx, yy)) > Rational::new(16, 1) {
                        continue;
                    }
                    let xsv = to(mx, xs, tx);
                    let ysv = to(my, ys, ty);
                    for (tx, txs) in xsv.copy_iter() {
                        for (ty, tys) in ysv.copy_iter() {
                            let nmx = if txs != 0 { txs.abs() } else { 1 };
                            let nmy = if tys != 0 { tys.abs() } else { 1 };
                            let xx = Rational::new(tx, mx);
                            let yy = Rational::new(ty, my);
                            if (xx - Rational::new(ax, 1)).abs() < Rational::new(3, 1)
                                && (yy - Rational::new(ay, 1)).abs() < Rational::new(3, 1)
                            {
                                continue;
                            }
                            let to = get_id(
                                &mut graph,
                                Rational::new(tx, nmx),
                                Rational::new(ty, nmy),
                                txs,
                                tys,
                            );
                            graph.add_edge(WeightedEdge::new(from, to, 1));
                        }
                    }
                }
            }
        }
    }
    let from = get_id(&mut graph, Rational::zero(), Rational::zero(), 0, 0);
    let to = get_id(&mut graph, Rational::new(x, 1), Rational::new(y, 1), 0, 0);
    let path = graph.distance(from, to).unwrap();
    assert!(path.0 < tl);
    let mut x = Vec::new();
    let mut at = 0;
    while at < path.1.len() {
        let (vert, _) = path.1[at];
        let xs = wh[vert].2;
        x.push(xs);
        at += if xs == 0 { 1 } else { xs.abs() as usize };
    }
    at = 0;
    let mut y = Vec::new();
    while at < path.1.len() {
        let (vert, _) = path.1[at];
        let ys = wh[vert].3;
        y.push(ys);
        at += if ys == 0 { 1 } else { ys.abs() as usize };
    }
    x.push(0);
    y.push(0);
    out.print_line(x);
    out.print_line(y);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    eprint!("\x1B[0m");
    output.flush();
    is_exhausted
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
