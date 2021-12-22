//{"name":"Task","group":"HackerEarth - December Circuits '21","url":"https://www.hackerearth.com/challenges/competitive/december-circuits-21-2/approximate/the-dark-library-fc5a9d36/","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n0 2\n0 4\n2 0\n2 4\n","output":"3\n0 2\n2 0\n2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::Shuffle;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashMap;
use std::ops::IndexMut;
use std::time::{Duration, Instant};

fn solve(input: &mut Input, _test_case: usize) {
    let time = Instant::now();
    let n = input.read();
    let r: f64 = input.read();
    let p: Vec<(f64, f64)> = input.read_vec(n);

    let step = 3f64.sqrt() / 2. * r;
    let grid = 5;
    let mut state = Arr2d::generate(grid, grid, |i, j| {
        (
            step / (grid as f64) * (i as f64),
            3. * r / (grid as f64) * (j as f64),
            HashMap::new(),
        )
    });
    let pt = p.clone();
    let mut ans = Vec::new();
    for (x, y) in p.into_iter() {
        let (ox, oy) = (x, y);
        for i in 0..grid {
            for j in 0..grid {
                let (shx, shy, map) = state.index_mut((i, j));
                let (mut x, mut y) = (x + *shx, y + *shy);

                let shift = x / step;
                let x_id = shift.floor() as i64;
                x -= (x_id as f64) * step;
                let start = if x_id % 2 == 0 { 0. } else { r * 3. / 2. };
                y -= start;
                let mut y_id = (y / (r / 2.)).floor() as i64;
                y -= (y_id as f64) * (r / 2.);
                let mut rem = y_id % 6;
                if rem < 0 {
                    rem += 6;
                }
                y_id = (y_id - rem) / 6;
                let (x_id, y_id) = match rem {
                    0 => (x_id, y_id),
                    1 => {
                        if r / 2. > x * r / (2. * step) + y {
                            (x_id, y_id)
                        } else {
                            (x_id + 1, y_id + if start != 0. { 1 } else { 0 })
                        }
                    }
                    2 | 3 => (x_id + 1, y_id + if start != 0. { 1 } else { 0 }),
                    4 => {
                        if y < x * r / (2. * step) {
                            (x_id + 1, y_id + if start != 0. { 1 } else { 0 })
                        } else {
                            (x_id, y_id + 1)
                        }
                    }
                    5 => (x_id, y_id + 1),
                    _ => panic!("unreachable"),
                };
                if !map.contains_key(&(x_id, y_id)) {
                    map.insert((x_id, y_id), vec![(ox, oy)]);
                } else {
                    map.get_mut(&(x_id, y_id)).unwrap().push((ox, oy));
                }
            }
        }
        let mut found = false;
        for (x0, y0) in ans.iter().cloned() {
            let dx = x - x0;
            let dy = y - y0;
            if dx * dx + dy * dy <= r * r {
                found = true;
                break;
            }
        }
        if !found {
            ans.push((x, y));
        }
    }
    for (shx, shy, map) in state {
        let mut c_ans = Vec::new();
        let mut rem = Vec::new();
        for ((x_id, y_id), pts) in map {
            if pts.len() <= 2 {
                rem.extend_from_slice(pts.as_slice());
            } else {
                c_ans.push((
                    (x_id as f64) * step - shx,
                    (y_id as f64) * 3. * r + if x_id % 2 == 0 { 0. } else { r * 3. / 2. } - shy,
                ));
            }
        }
        let mut was = BitSet::new(rem.len());
        for i in 0..rem.len() {
            if was[i] {
                continue;
            }
            let (x, y) = rem[i];
            let mut found = false;
            for (x0, y0) in c_ans.iter().cloned() {
                let dx = x - x0;
                let dy = y - y0;
                if dx * dx + dy * dy <= r * r {
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            for j in (i + 1)..rem.len() {
                if was[j] {
                    continue;
                }
                let (x0, y0) = rem[j];
                let dx = x - x0;
                let dy = y - y0;
                if dx * dx + dy * dy <= r * r * 4. {
                    c_ans.push(((x + x0) / 2., (y + y0) / 2.));
                    was.set(j, true);
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            c_ans.push((x, y));
        }
        if ans.len() > c_ans.len() {
            ans = c_ans.clone();
        }
    }

    fn mid_perp((x0, y0): (f64, f64), (x1, y1): (f64, f64)) -> (f64, f64, f64) {
        let a = x0 - x1;
        let b = y0 - y1;
        let c = -a * (x0 + x1) / 2. - b * (y0 + y1) / 2.;
        (a, b, c)
    }

    fn det(a: f64, b: f64, c: f64, d: f64) -> f64 {
        a * d - b * c
    }

    fn dist((x0, y0): (f64, f64), (x1, y1): (f64, f64)) -> f64 {
        f64::hypot(x0 - x1, y0 - y1)
    }

    fn welzl(
        pts: &mut Vec<(f64, f64)>,
        res: &mut Vec<(f64, f64)>,
        r: f64,
        start: Instant,
    ) -> Option<(f64, f64)> {
        if start.elapsed() > Duration::from_secs(4) {
            return None;
        }
        if pts.is_empty() || res.len() == 3 {
            let c = match res.len() {
                0 => {
                    return None;
                }
                1 => res[0],
                2 => ((res[0].0 + res[1].0) / 2., (res[0].1 + res[1].1) / 2.),
                3 => {
                    let (a1, b1, c1) = mid_perp(res[0], res[1]);
                    let (a2, b2, c2) = mid_perp(res[0], res[2]);
                    let zn = det(a1, b1, a2, b2);
                    if zn == 0. {
                        return None;
                    }
                    (-det(c1, b1, c2, b2) / zn, -det(a1, c1, a2, c2) / zn)
                }
                _ => panic!("unreachable"),
            };
            for d in res {
                if dist(c, *d) > r {
                    return None;
                }
            }
            for d in pts {
                if dist(c, *d) > r {
                    return None;
                }
            }
            return Some(c);
        }
        // let pos = (random().gen() % (pts.len() as u64)) as usize;
        // let pt = pts[pos];
        // pts.remove(pos);
        let pt = pts.pop().unwrap();
        match welzl(pts, res, r, start) {
            None => {
                res.push(pt);
                let ans = welzl(pts, res, r, start);
                pts.push(res.pop().unwrap());
                ans
            }
            Some(ans) => {
                if dist(ans, pt) <= r {
                    pts.push(pt);
                    Some(ans)
                } else {
                    res.push(pt);
                    let ans = welzl(pts, res, r, start);
                    pts.push(res.pop().unwrap());
                    ans
                }
            }
        }
    }

    loop {
        if time.elapsed() > Duration::from_secs(4) {
            break;
        }
        let mut c_ans = Vec::new();
        let mut rem = pt.clone();
        rem.as_mut_slice().shuffle();
        let mut was = BitSet::new(rem.len());
        for i in 0..rem.len() {
            if was[i] {
                continue;
            }
            if time.elapsed() > Duration::from_secs(4) {
                for i in i..rem.len() {
                    if was[i] {
                        continue;
                    }
                    let (x, y) = rem[i];
                    let mut found = false;
                    for j in (i + 1)..rem.len() {
                        if was[j] {
                            continue;
                        }
                        let (x0, y0) = rem[j];
                        let dx = x - x0;
                        let dy = y - y0;
                        if dx * dx + dy * dy <= r * r * 4. {
                            c_ans.push(((x + x0) / 2., (y + y0) / 2.));
                            was.set(j, true);
                            found = true;
                            break;
                        }
                    }
                    if found {
                        continue;
                    }
                    c_ans.push((x, y));
                }
                break;
            }
            let mut pts = vec![rem[i]];
            let mut ids = vec![i];
            let mut center = rem[i];
            for j in (i + 1)..rem.len() {
                if was[j] {
                    continue;
                }
                pts.push(rem[j]);
                if dist(center, rem[j]) <= r {
                    ids.push(j);
                    continue;
                }
                let mut p = pts.clone();
                let mut v = Vec::new();
                match welzl(&mut p, &mut v, r, time) {
                    None => {
                        pts.pop();
                    }
                    Some(c) => {
                        center = c;
                        ids.push(j);
                    }
                }
            }
            for j in ids {
                was.set(j, true);
            }
            c_ans.push(center);
            // let (x, y) = rem[i];
            // let mut found = false;
            // for (x0, y0) in c_ans.iter().cloned() {
            //     let dx = x - x0;
            //     let dy = y - y0;
            //     if dx * dx + dy * dy <= r * r {
            //         found = true;
            //         break;
            //     }
            // }
            // if found {
            //     continue;
            // }
            // for j in (i + 1)..rem.len() {
            //     if was[j] {
            //         continue;
            //     }
            //     let (x0, y0) = rem[j];
            //     let dx = x - x0;
            //     let dy = y - y0;
            //     if dx * dx + dy * dy <= r * r * 4. {
            //         c_ans.push(((x + x0) / 2., (y + y0) / 2.));
            //         was.set(j, true);
            //         found = true;
            //         break;
            //     }
            // }
            // if found {
            //     continue;
            // }
            // c_ans.push((x, y));
        }
        if ans.len() > c_ans.len() {
            ans = c_ans.clone();
        }
    }
    out_line!(ans.len());
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
