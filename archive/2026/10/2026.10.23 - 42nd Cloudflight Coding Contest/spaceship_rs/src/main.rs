//! Spaceship (CCC 41 classic, training). Usage: spaceship <level> <input> <output>

mod astar;
mod plan;
mod sim;

use plan::{plan_direct, Case};
use sim::Asteroid;
use std::fmt::Write as _;

fn parse_xy(s: &str) -> (i32, i32) {
    let (a, b) = s.split_once(',').expect("x,y");
    (a.parse().unwrap(), b.parse().unwrap())
}

/// Levels 5-7: station + limit, then asteroids (level 5/6: one line "x,y";
/// level 7: a count line and a line of "x,y" pairs).
fn parse_cases(level: usize, text: &str) -> Vec<Case> {
    let mut lines = text.lines().map(str::trim).filter(|l| !l.is_empty());
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut cases = Vec::with_capacity(n);
    for _ in 0..n {
        let head = lines.next().unwrap();
        let (xy, limit) = head.split_once(' ').unwrap();
        let goal = parse_xy(xy);
        let limit: usize = limit.parse().unwrap();
        let asteroids: Vec<Asteroid> = if level >= 7 {
            let _count: usize = lines.next().unwrap().parse().unwrap();
            lines.next().unwrap().split_whitespace().map(parse_xy).map(|(x, y)| Asteroid { x, y, vx: 0, vy: 0 }).collect()
        } else {
            let (x, y) = parse_xy(lines.next().unwrap());
            vec![Asteroid { x, y, vx: 0, vy: 0 }]
        };
        cases.push(Case { goal, asteroids, limit });
    }
    cases
}

fn fmt_paces(p: &[i32]) -> String {
    p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let level: usize = args[1].parse().unwrap();
    let text = std::fs::read_to_string(&args[2]).unwrap();
    let cases = parse_cases(level, &text);
    let mut out = String::new();
    for (i, case) in cases.iter().enumerate() {
        let (xp, yp, time) = if let Some(p) = plan_direct(case) {
            (p.xp, p.yp, p.time)
        } else {
            let field = astar::Field::new(case.goal, &case.asteroids, 8);
            let f = astar::search(case.goal, &field, case.limit)
                .unwrap_or_else(|| panic!("no plan for case {} goal {:?} limit {}", i, case.goal, case.limit));
            let v = sim::simulate(&f.xp, &f.yp, case.goal, &case.asteroids, case.limit);
            assert!(v.ok, "a* plan rejected by simulator: {}", v.reason);
            (f.xp, f.yp, f.time)
        };
        writeln!(out, "{}\n{}\n", fmt_paces(&xp), fmt_paces(&yp)).unwrap();
        eprintln!("case {}: time {} / {}", i, time, case.limit);
    }
    std::fs::write(&args[3], out).unwrap();
}
