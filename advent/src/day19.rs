use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::{Input, Readable};
use std::collections::HashSet;

pub trait CommaList {
    fn read_list<T: Readable>(&mut self) -> Vec<T>;
}

impl CommaList for Input<'_> {
    fn read_list<T: Readable>(&mut self) -> Vec<T> {
        let mut s: String = self.read();
        s = s.replace(",", " ");
        let mut b = s.as_bytes();
        let input = Input::new(&mut b);
        input.into_iter().collect_vec()
    }
}

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    struct Rotation {
        mat: Arr2d<i64>,
    }

    impl Rotation {
        fn generate() -> Vec<Rotation> {
            let mut res = Vec::new();
            for i in 0..3 {
                for j in (-1..=1).step_by(2) {
                    let mut mat = Arr2d::new(3, 3, 0);
                    mat[(0, i)] = j;
                    mat[(1, (i + 1) % 3)] = 1;
                    mat[(2, (i + 2) % 3)] = j;
                    res.push(Rotation { mat: mat.clone() });
                    mat[(1, (i + 1) % 3)] *= -1;
                    mat[(2, (i + 2) % 3)] *= -1;
                    res.push(Rotation { mat: mat.clone() });
                    mat[(1, (i + 1) % 3)] = 0;
                    mat[(2, (i + 2) % 3)] = 0;
                    mat[(1, (i + 2) % 3)] = 1;
                    mat[(2, (i + 1) % 3)] = -j;
                    res.push(Rotation { mat: mat.clone() });
                    mat[(1, (i + 2) % 3)] *= -1;
                    mat[(2, (i + 1) % 3)] *= -1;
                    res.push(Rotation { mat: mat.clone() });
                }
            }
            res
        }

        fn apply(&self, p: Point) -> Point {
            Point {
                x: p.x * self.mat[(0, 0)] + p.y * self.mat[(0, 1)] + p.z * self.mat[(0, 2)],
                y: p.x * self.mat[(1, 0)] + p.y * self.mat[(1, 1)] + p.z * self.mat[(1, 2)],
                z: p.x * self.mat[(2, 0)] + p.y * self.mat[(2, 1)] + p.z * self.mat[(2, 2)],
            }
        }
    }

    #[derive(Eq, PartialEq, Hash, Copy, Clone)]
    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    impl Point {
        fn read(input: &mut Input) -> Option<Self> {
            let s = input.read_line();
            if s.is_empty() {
                None
            } else {
                let tok = s.split(',').collect_vec();
                assert_eq!(tok.len(), 3);
                let x = tok[0].parse::<i64>().unwrap();
                let y = tok[1].parse::<i64>().unwrap();
                let z = tok[2].parse::<i64>().unwrap();
                Some(Self { x, y, z })
            }
        }

        fn add(p1: Point, p2: Point) -> Point {
            Point {
                x: p1.x + p2.x,
                y: p1.y + p2.y,
                z: p1.z + p2.z,
            }
        }

        fn neg(&self) -> Point {
            Point {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }

        fn near(&self, pt: Point) -> bool {
            (self.x - pt.x).abs() <= 1000
                && (self.z - pt.z).abs() <= 1000
                && (self.y - pt.y).abs() <= 1000
        }

        fn dist(&self, pt: Point) -> i64 {
            (self.x - pt.x).abs() + (self.z - pt.z).abs() + (self.y - pt.y).abs()
        }
    }

    let rotations = Rotation::generate();
    let mut scanners = Vec::new();
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        inp.next_token();
        inp.next_token();
        inp.next_token();
        inp.next_token();
        let mut scanner = Vec::new();
        while let Some(pt) = Point::read(&mut inp) {
            scanner.push(pt);
        }
        scanners.push(scanner);
    }

    let mut used = BitSet::new(scanners.len());
    used.set(0, true);
    let mut pts = scanners[0].drain(..).collect_vec();
    let mut i = 0;
    let mut sc_pos = vec![Point { x: 0, y: 0, z: 0 }];
    while i < pts.len() {
        let pt = pts[i];
        for (j, sc) in scanners.iter().enumerate() {
            if used[j] {
                continue;
            }
            let mut found = false;
            for opt in sc.iter().cloned() {
                for rot in rotations.iter() {
                    let cur = rot.apply(opt);
                    let center = Point::add(pt, cur.neg());
                    let mut rotated = sc
                        .iter()
                        .map(|pt| Point::add(center, rot.apply(*pt)))
                        .collect::<HashSet<_>>();
                    let mut good = true;
                    let mut matched = 0;
                    for p in pts.iter().cloned() {
                        if center.near(p) {
                            if rotated.contains(&p) {
                                matched += 1;
                                rotated.remove(&p);
                            } else {
                                good = false;
                                break;
                            }
                        }
                    }
                    if good && matched >= 12 {
                        sc_pos.push(center);
                        pts.extend(rotated.into_iter());
                        found = true;
                        used.set(j, true);
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
        i += 1;
    }

    assert_eq!(used.iter().count(), scanners.len());
    let mut ans = 0;
    for p1 in sc_pos.iter() {
        for p2 in sc_pos.iter() {
            ans.maxim(p1.dist(*p2));
        }
    }
    println!("{}", pts.len());
    println!("{}", ans);
}
