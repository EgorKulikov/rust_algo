//{"name":"I. Cloud Retainer’s Game","group":"Yandex - Stage 9: Grand Prix of Nanjing","url":"https://official.contest.yandex.ru/opencupXXII/contest/33444/problems/I/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4\n3\n1 1\n2 2\n6 2\n4\n3 1\n3 3\n5 1\n7 3\n3\n1\n4 2\n3\n1 1\n6 2\n9 1\n","output":"3\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ICloudRetainersGame"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let h: i32 = input.read();
    let h2 = h * 2;
    let mut boards: Vec<(i32, i32)> = input.read();
    let mut coins: Vec<(i32, i32)> = input.read();
    coins.iter_mut().for_each(|(_, y)| *y = -(*y));
    boards.extend_from_slice(coins.as_slice());
    boards.sort();
    let mut ans = HashMap::new();
    ans.insert(0, 0);
    for (x, y_strange) in boards {
        let y = y_strange.abs();
        let z1 = ((y - x) % h2 + h2) % h2;
        let z2 = ((h2 - y - x) % h2 + h2) % h2;
        let score1 = ans.get(&z1);
        let score2 = ans.get(&z2);
        if y_strange < 0 {
            let mut to_insert = Vec::new();
            if let Some(score1) = score1 {
                to_insert.push((z1, *score1 + 1));
            }
            if let Some(score2) = score2 {
                to_insert.push((z2, *score2 + 1));
            }
            for (key, value) in to_insert {
                ans.insert(key, value);
            }
        } else {
            if score1.is_none() && score2.is_none() {
                continue;
            }
            let mut score = 0;
            if let Some(score1) = score1 {
                score.maxim(*score1);
            }
            if let Some(score2) = score2 {
                score.maxim(*score2);
            }
            ans.insert(z1, score);
            ans.insert(z2, score);
        }
    }
    out_line!(ans.into_values().max().unwrap());
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
