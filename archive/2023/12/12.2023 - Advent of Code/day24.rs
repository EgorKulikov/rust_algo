//{"name":"day24","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day24"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::ray::Ray;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::rational::Rational;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let from = input.read_long() as i128;
    let to = input.read_long() as i128;
    let mut stones = Vec::new();
    while !input.is_empty() {
        scan!(input, "#, #, # @ #, #, #", '#', x: i128, y: i128, z: i128, vx: i128, vy: i128, vz: i128);
        stones.push((x - from, y - from, z - from, vx, vy, vz));
    }

    type Coord = Rational<i128>;
    {
        // part 1
        let mut ans = 0;
        for i in stones.indices() {
            let (x1, y1, _z1, vx1, vy1, _vz1) = stones[i];
            let p11 = Point::new(Coord::from(x1), Coord::from(y1));
            let p12 = Point::new(Coord::from(x1 + vx1), Coord::from(y1 + vy1));
            let r1 = Ray::new(p11, p12);
            for j in 0..i {
                let (x2, y2, _z2, vx2, vy2, _vz2) = stones[j];
                let p21 = Point::new(Coord::from(x2), Coord::from(y2));
                let p22 = Point::new(Coord::from(x2 + vx2), Coord::from(y2 + vy2));
                let r2 = Ray::new(p21, p22);
                if let Some(p) = r1.intersect_ray(r2) {
                    if p.x >= Coord::zero()
                        && p.x <= Coord::from(to - from)
                        && p.y >= Coord::zero()
                        && p.y <= Coord::from(to - from)
                    {
                        ans += 1;
                    }
                }
            }
        }
        out.print_line(ans);
    }

    {
        // part 2
        const LIMIT: i128 = 300;
        let (x1, y1, z1, vx1, vy1, vz1) = stones[0];
        let p11 = Point::new(Coord::from(x1), Coord::from(y1));
        let (x2, y2, z2, vx2, vy2, vz2) = stones[1];
        let p21 = Point::new(Coord::from(x2), Coord::from(y2));
        for tx in -LIMIT..=LIMIT {
            for ty in -LIMIT..=LIMIT {
                if vx1 == tx || vx2 == tx {
                    continue;
                }
                let p12 = Point::new(Coord::from(x1 + vx1 - tx), Coord::from(y1 + vy1 - ty));
                let r1 = Ray::new(p11, p12);
                let p22 = Point::new(Coord::from(x2 + vx2 - tx), Coord::from(y2 + vy2 - ty));
                let r2 = Ray::new(p21, p22);
                if let Some(p) = r1.intersect_ray(r2) {
                    let t = (p.x - Coord::from(x1)) / Coord::from(vx1 - tx);
                    let x0 = Coord::from(x1) - t * Coord::from(tx - vx1);
                    if x0.den() != 1 {
                        continue;
                    }
                    let x = x0.num();
                    let y0 = Coord::from(y1) - t * Coord::from(ty - vy1);
                    if y0.den() != 1 {
                        continue;
                    }
                    let y = y0.num();
                    let t2 = (p.x - Coord::from(x2)) / Coord::from(vx2 - tx);
                    if t == t2 {
                        continue;
                    }
                    let tz = (Coord::from(z1) + t * Coord::from(vz1)
                        - Coord::from(z2)
                        - t2 * Coord::from(vz2))
                        / (t - t2);
                    let pz = Coord::from(z1) + t * Coord::from(vz1);
                    let z0 = pz - tz * t;
                    if z0.den() != 1 {
                        continue;
                    }
                    let z = z0.num();
                    let mut good = true;
                    for &(xx, yy, zz, vx, vy, vz) in &stones {
                        let time = {
                            let dx = Coord::from(vx) - Coord::from(tx);
                            if dx == Coord::zero() {
                                let dy = Coord::from(vy) - Coord::from(ty);
                                if dy == Coord::zero() {
                                    let dz = Coord::from(vz) - Coord::from(tz);
                                    if dz == Coord::zero() {
                                        Coord::zero()
                                    } else {
                                        (Coord::from(z) - Coord::from(zz)) / dz
                                    }
                                } else {
                                    (Coord::from(y) - Coord::from(yy)) / dy
                                }
                            } else {
                                (Coord::from(x) - Coord::from(xx)) / dx
                            }
                        };
                        let x0 = Coord::from(xx) + time * Coord::from(vx);
                        let x1 = Coord::from(x) + time * Coord::from(tx);
                        if x0 != x1 {
                            good = false;
                            break;
                        }
                        let y0 = Coord::from(yy) + time * Coord::from(vy);
                        let y1 = Coord::from(y) + time * Coord::from(ty);
                        if y0 != y1 {
                            good = false;
                            break;
                        }
                        let z0 = Coord::from(zz) + time * Coord::from(vz);
                        let z1 = Coord::from(z) + time * Coord::from(tz);
                        if z0 != z1 {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        out.print_line(x + y + z + 3 * from);
                        return;
                    }
                }
            }
        }
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
