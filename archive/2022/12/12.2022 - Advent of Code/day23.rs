//{"name":"day23","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day23"}}}

use algo_lib::collections::default_map::DefaultHashMap;
// use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::{HashMap, HashSet, VecDeque};

fn solve(input: &mut Input, _test_case: usize) {
    let mut table = Vec::new();

    while !input.is_exhausted() {
        table.push(input.read::<Str>());
        input.skip_whitespace();
    }

    let mut elves = HashSet::new();
    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if table[i][j] == b'#' {
                elves.insert((i.into_isize(), j.into_isize()));
            }
        }
    }
    let mut directions: VecDeque<(isize, isize)> =
        VecDeque::from(vec![(-1, 0), (1, 0), (0, -1), (0, 1)]);

    // for _ in 0..10 {
    for i in 1.. {
        let mut proposed: DefaultHashMap<_, i32> = DefaultHashMap::new();
        let mut moves = HashMap::new();
        for &(i, j) in &elves {
            let mut found = false;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    if elves.contains(&(i + di, j + dj)) {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                continue;
            }
            for &(di, dj) in &directions {
                let (ni, nj) = (i + di, j + dj);
                let mut bad = false;
                for x in -1..=1 {
                    let (ci, cj) = if di == 0 { (ni + x, nj) } else { (ni, nj + x) };
                    if elves.contains(&(ci, cj)) {
                        bad = true;
                        break;
                    }
                }
                if bad {
                    continue;
                }
                moves.insert((i, j), (ni, nj));
                proposed[(ni, nj)] += 1;
                break;
            }
        }
        let mut new_elves = HashSet::new();
        for &(i, j) in &elves {
            match moves.get(&(i, j)) {
                Some(&(ni, nj)) => {
                    if proposed[(ni, nj)] == 1 {
                        new_elves.insert((ni, nj));
                    } else {
                        new_elves.insert((i, j));
                    }
                }
                None => {
                    new_elves.insert((i, j));
                }
            }
        }
        if elves == new_elves {
            out_line!(i);
            break;
        }
        elves = new_elves;
        directions.rotate_left(1);
        // let mut min_i = None;
        // let mut max_i = None;
        // let mut min_j = None;
        // let mut max_j = None;
        // for &(i, j) in &elves {
        //     min_i.minim(i);
        //     max_i.maxim(i);
        //     min_j.minim(j);
        //     max_j.maxim(j);
        // }
        // let table = Arr2d::generate(
        //     (max_i.unwrap() - min_i.unwrap() + 1).into_usize(),
        //     (max_j.unwrap() - min_j.unwrap() + 1).into_usize(),
        //     |i, j| {
        //         if elves.contains(&(
        //             i.into_isize() + min_i.unwrap(),
        //             j.into_isize() + min_j.unwrap(),
        //         )) {
        //             '#'
        //         } else {
        //             '.'
        //         }
        //     },
        // );
        // output().print_table(&table);
        // out_line!("===================");
    }

    /*let mut min_i = None;
    let mut max_i = None;
    let mut min_j = None;
    let mut max_j = None;
    for &(i, j) in &elves {
        min_i.minim(i);
        max_i.maxim(i);
        min_j.minim(j);
        max_j.maxim(j);
    }
    out_line!(
        ((max_i.unwrap() - min_i.unwrap() + 1) * (max_j.unwrap() - min_j.unwrap() + 1))
            .into_usize()
            - elves.len()
    );*/
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
