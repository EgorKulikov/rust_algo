//{"name":"E. Объединение городов","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"11\n1 3\n#.#\n2 2\n.#\n#.\n4 4\n..##\n...#\n#...\n##..\n6 6\n.##...\n##....\n......\n....##\n.....#\n...###\n6 5\n.#..#\n.#..#\n.#..#\n.#.##\n.#...\n##...\n5 5\n#####\n#...#\n#.#.#\n#...#\n#####\n4 4\n.##.\n##.#\n#.##\n.##.\n5 5\n..###\n....#\n.....\n#....\n#....\n5 6\n.##...\n##....\n#....#\n....##\n...##.\n6 5\n..##.\n...##\n....#\n#....\n##...\n.##..\n5 4\n..##\n..#.\n..#.\n#...\n#...\n","output":"###\n\n.#\n##\n\n..##\n..##\n###.\n##..\n\n.##...\n###...\n..#...\n..####\n...###\n...###\n\n.####\n.####\n.####\n.####\n.#...\n##...\n\n#####\n#####\n#####\n#####\n#####\n\n.##.\n####\n####\n.##.\n\n..###\n..###\n..#..\n###..\n#....\n\n.##...\n###...\n######\n...###\n...##.\n\n..##.\n..###\n..###\n###..\n###..\n.##..\n\n..##\n..#.\n..#.\n###.\n#...\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EObedinenieGorodov"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::{out, out_line};
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let mut s = input.read_table::<char>(n, m);

    let mut cities = Vec::with_capacity(2);
    for i in 0..n {
        for j in 0..m {
            if s[(i, j)] == '#' {
                let mut city = vec![(None, None); n];
                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                s[(i, j)] = '.';
                while let Some((x, y)) = queue.pop_front() {
                    city[x].0.minim(y);
                    city[x].1.maxim(y);
                    for (nx, ny) in D4::iter(x, y, n, m) {
                        if s[(nx, ny)] == '#' {
                            s[(nx, ny)] = '.';
                            queue.push_back((nx, ny));
                        }
                    }
                }
                cities.push(city);
            }
        }
    }
    assert_eq!(cities.len(), 2);
    fn pretify(city: &mut Vec<(Option<usize>, Option<usize>)>) {
        for i in 0..city.len() {
            if city[i].0.is_none() {
                continue;
            }
            let a = city[i].0.unwrap();
            let c = city[i].1.unwrap();
            for j in i + 1..city.len() {
                if city[j].0.is_none() {
                    continue;
                }
                let b = city[j].0.unwrap();
                let d = city[j].1.unwrap();
                for k in i + 1..j {
                    city[k].0.minim(a.max(b));
                    city[k].1.maxim(c.min(d));
                }
            }
        }
    }
    for city in cities.iter_mut() {
        pretify(city);
    }

    fn print(m: usize, city: &Vec<(Option<usize>, Option<usize>)>) {
        for row in city {
            match (row.0, row.1) {
                (None, None) => {
                    for _ in 0..m {
                        out!('.');
                    }
                    out_line!();
                }
                (Some(a), Some(b)) => {
                    for _ in 0..a {
                        out!('.');
                    }
                    for _ in a..=b {
                        out!('#');
                    }
                    for _ in b + 1..m {
                        out!('.');
                    }
                    out_line!();
                }
                _ => unreachable!(),
            }
        }
        out_line!();
    }

    for i in 0..n {
        if cities[0][i].0.is_some() && cities[1][i].0.is_some() {
            let mut city = Vec::with_capacity(n);
            for j in 0..n {
                if cities[0][j].0.is_none() {
                    city.push(cities[1][j]);
                } else if cities[1][j].0.is_none() {
                    city.push(cities[0][j]);
                } else {
                    let a = cities[0][j].0.unwrap();
                    let b = cities[0][j].1.unwrap();
                    let c = cities[1][j].0.unwrap();
                    let d = cities[1][j].1.unwrap();
                    city.push((Some(a.min(c)), Some(b.max(d))));
                }
            }
            pretify(&mut city);
            print(m, &city);
            return;
        }
    }
    let mut city = Vec::with_capacity(n);
    for j in 0..n {
        if cities[0][j].0.is_none() {
            city.push(cities[1][j]);
        } else if cities[1][j].0.is_none() {
            city.push(cities[0][j]);
        } else {
            unreachable!();
        }
    }
    pretify(&mut city);
    for i in 0..n {
        if city[i].0.is_none() {
            continue;
        }
        let a = city[i].0.unwrap();
        let b = city[i].1.unwrap();
        if a > b {
            for j in i..n {
                let c = city[j].0.unwrap();
                let d = city[j].1.unwrap();
                if c > d {
                    city[j] = (Some(a), Some(a));
                } else {
                    if city[j].0 == Some(a) {
                        city[i - 1].1 = Some(a);
                    } else {
                        city[j].1 = Some(a);
                    }
                    break;
                }
            }
            break;
        }
        if i > 0 && city[i - 1].0.is_some() && city[i - 1].0.unwrap() > city[i].1.unwrap() {
            city[i].1 = city[i - 1].0;
            break;
        }
        if i > 0 && city[i - 1].0.is_some() && city[i - 1].1.unwrap() < city[i].0.unwrap() {
            city[i].0 = city[i - 1].1;
            break;
        }
    }
    print(m, &city);
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
