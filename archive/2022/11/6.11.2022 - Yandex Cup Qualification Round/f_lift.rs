//{"name":"F. Лифт","group":"Yandex - Yandex Cup 2022: Алгоритм, спринт (квалификация)","url":"https://contest.yandex.ru/yacup/contest/42199/problems/F/","interactive":false,"timeLimit":6000,"tests":[{"input":"5 10\n5 5 1\n10 2 3\n15 3 1\n20 4 5\n25 3 2\n","output":"40\n85\n50\n95\n40\n"},{"input":"5 10\n1 1 4\n101 1 4\n202 1 4\n300 4 1\n301 4 1\n","output":"0\n30\n30\n0\n59\n"},{"input":"2 10\n1 5 6\n2 2 3\n","output":"40\n89\n"},{"input":"6 1\n1 1 4\n2 2 4\n3 3 4\n7 4 2\n8 1 2\n9 2 1\n","output":"0\n0\n0\n0\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLift"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let t = input.read_long();
    let calls = input.read_vec::<(i64, i64, i64)>(n);

    let mut up: DefaultTreeMap<_, VecDeque<usize>> = DefaultTreeMap::new();
    let mut down: DefaultTreeMap<_, VecDeque<usize>> = DefaultTreeMap::new();
    let mut processed = BitSet::new(n);
    let mut cur_floor = 1;
    let mut cur_time = 0;
    let mut ans = vec![0; n];
    let mut next = 0;

    for at in 0..n {
        if processed[at] {
            continue;
        }
        let (time, from, to) = calls[at];
        cur_time.maxim(time);
        cur_time += t * (from - cur_floor).abs();
        while next < n && calls[next].0 <= cur_time {
            let (_, from, to) = calls[next];
            if from < to {
                up[from].push_back(next);
            } else {
                down[from].push_back(next);
            }
            next += 1;
        }
        let start_time = cur_time;
        let start_floor = from;
        let mut to_floor = to;
        let cur_next = next;
        if from < to {
            loop {
                if let Some((&floor, d)) = up.range_mut(start_floor..=to_floor).next() {
                    let arrive = (floor - start_floor) * t + start_time;
                    while let Some(i) = d.pop_front() {
                        if processed[i] {
                            continue;
                        }
                        let (time, _, to) = calls[i];
                        processed.set(i, true);
                        ans[i] = arrive - time;
                        to_floor.maxim(to);
                    }
                    up.remove(&floor);
                    continue;
                }
                if next < n && calls[next].0 <= start_time + (to_floor - start_floor) * t {
                    let (time, from, to) = calls[next];
                    if to > from
                        && from >= start_floor
                        && from <= to_floor
                        && time <= start_time + (from - start_floor) * t
                    {
                        ans[next] = (start_time + (from - start_floor) * t) - time;
                        processed.set(next, true);
                        to_floor.maxim(to);
                    }
                    next += 1;
                    continue;
                }
                break;
            }
        } else {
            loop {
                if let Some((&floor, d)) = down.range_mut(to_floor..=start_floor).next() {
                    let arrive = (start_floor - floor) * t + start_time;
                    while let Some(i) = d.pop_front() {
                        if processed[i] {
                            continue;
                        }
                        let (time, _, to) = calls[i];
                        processed.set(i, true);
                        ans[i] = arrive - time;
                        to_floor.minim(to);
                    }
                    down.remove(&floor);
                    continue;
                }
                if next < n && calls[next].0 <= start_time + (start_floor - to_floor) * t {
                    let (time, from, to) = calls[next];
                    if to < from
                        && from <= start_floor
                        && from >= to_floor
                        && time <= start_time + (start_floor - from) * t
                    {
                        ans[next] = (start_time + (start_floor - from) * t) - time;
                        processed.set(next, true);
                        to_floor.minim(to);
                    }
                    next += 1;
                    continue;
                }
                break;
            }
        }
        for i in cur_next..next {
            if !processed[i] {
                let (_, from, to) = calls[i];
                if from < to {
                    up[from].push_back(i);
                } else {
                    down[from].push_back(i);
                }
            }
        }
        cur_floor = to_floor;
        cur_time = start_time + (to_floor - start_floor).abs() * t;
    }
    output().print_per_line(&ans);
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
