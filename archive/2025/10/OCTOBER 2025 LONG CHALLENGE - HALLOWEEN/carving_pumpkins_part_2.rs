//{"name":"carving_pumpkins_part_2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::polygon::{ConvexHull, Polygon};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::Square;
use algo_lib::numbers::rational::Rational;
use algo_lib::output;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let w = input.read_long();
    let h = input.read_long();
    let e = input.read_long();
    let m = input.read_long();
    drop(input);

    output!(out, "Case #{test_case}:");
    let mut pts = Vec::new();
    let mut edge = Vec::new();
    for x in 1..(w + 1) / 2 {
        for y in 1..(h + 1) / 2 {
            if Rational::new(x * x, w * w) + Rational::new(y * y, h * h) < Rational::new(1, 4) {
                pts.push(Point::new(x, y));
                if x == 1
                    || y == 1
                    || Rational::new((x + 1).square(), w * w) + Rational::new(y * y, h * h)
                        >= Rational::new(1, 4)
                    || Rational::new(x * x, w * w) + Rational::new((y + 1).square(), h * h)
                        >= Rational::new(1, 4)
                {
                    edge.push(Point::new(x, y));
                }
            }
        }
    }

    'outer: for i in edge.indices() {
        for j in 0..i {
            let line = edge[i].line(edge[j]);
            let mut over = Vec::new();
            let mut under = Vec::new();
            for p in pts.copy_iter() {
                let val = line.value(p);
                if val > 0 {
                    over.push(p);
                } else if val < 0 {
                    under.push(p);
                }
            }
            let build_map = |pts: &[Point<i64>]| {
                let mut map = FxHashMap::default();
                for &p in pts {
                    let triangle = Polygon::new(vec![edge[i], edge[j], p]);
                    let area = triangle.double_area().abs();
                    map.insert(area, p);
                }
                map
            };
            let over_map = build_map(&over);
            let under_map = build_map(&under);
            for (&over_aria, &p1) in over_map.iter() {
                let need = 2 * e - over_aria;
                if let Some(&p2) = under_map.get(&need) {
                    let v = vec![edge[i], p1, edge[j], p2];
                    let pol = v.clone().convex_hull();
                    if pol.points.len() < 4 {
                        continue;
                    }
                    out.print_line_iter(v.copy_map(|p| (-p.x, p.y)));
                    out.print_line(&v);
                    break 'outer;
                }
            }
        }
    }

    let mut pts = Vec::new();
    let mut edge = Vec::new();
    for x in -(w - 1) / 2..(w + 1) / 2 {
        for y in 1..(h + 1) / 2 {
            if Rational::new(x * x, w * w) + Rational::new(y * y, h * h) < Rational::new(1, 4) {
                pts.push(Point::new(x, y));
                if y == 1
                    || Rational::new((x + 1).square(), w * w) + Rational::new(y * y, h * h)
                        >= Rational::new(1, 4)
                    || Rational::new((x - 1).square(), w * w) + Rational::new(y * y, h * h)
                        >= Rational::new(1, 4)
                    || Rational::new(x * x, w * w) + Rational::new((y + 1).square(), h * h)
                        >= Rational::new(1, 4)
                {
                    edge.push(Point::new(x, y));
                }
            }
        }
    }

    'outer: for i in edge.indices() {
        for j in 0..i {
            let line = edge[i].line(edge[j]);
            let mut over = Vec::new();
            let mut under = Vec::new();
            for p in pts.copy_iter() {
                let val = line.value(p);
                if val > 0 {
                    over.push(p);
                } else if val < 0 {
                    under.push(p);
                }
            }
            let build_map = |pts: &[Point<i64>]| {
                let mut map = FxHashMap::default();
                for &p in pts {
                    let triangle = Polygon::new(vec![edge[i], edge[j], p]);
                    let area = triangle.double_area().abs();
                    map.insert(area, p);
                }
                map
            };
            let over_map = build_map(&over);
            let under_map = build_map(&under);
            for (&over_aria, &p1) in over_map.iter() {
                let need = 2 * m - over_aria;
                if let Some(&p2) = under_map.get(&need) {
                    let v = vec![edge[i], p1, edge[j], p2];
                    let pol = v.clone().convex_hull();
                    if pol.points.len() < 4 {
                        continue;
                    }
                    out.print_line_iter(v.copy_map(|p| (p.x, -p.y)));
                    break 'outer;
                }
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, false, pre_calc, solve);
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
