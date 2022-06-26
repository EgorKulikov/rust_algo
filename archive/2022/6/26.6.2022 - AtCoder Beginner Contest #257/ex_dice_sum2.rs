//{"name":"Ex - Dice Sum 2","group":"AtCoder - NS Solutions Corporation Programming Contest 2022（AtCoder Beginner Contest 257）","url":"https://atcoder.jp/contests/abc257/tasks/abc257_h","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 2 3\n1 1 1 1 1 1\n2 2 2 2 2 2\n3 3 3 3 3 3\n","output":"20\n"},{"input":"10 5\n2 5 6 5 2 1 7 9 7 2\n5 5 2 4 7 6\n2 2 8 7 7 9\n8 1 9 6 10 8\n8 6 10 3 3 9\n1 10 5 8 1 10\n7 8 4 8 6 5\n1 10 2 5 1 7\n7 4 1 4 5 4\n5 10 1 5 1 2\n5 1 2 3 6 2\n","output":"1014\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ExDiceSum2"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let c = input.read_long_vec(n);
    let a = input.read_table::<i64>(n, 6);

    let mut dice = Vec::with_capacity(n);
    for i in 0..n {
        let s = a.row(i).sum::<i64>();
        let k = s;
        let b = 6 * a.row(i).map(|&v| v * v).sum::<i64>() - s * s - 36 * c[i];
        dice.push((k, b));
    }
    let mut events: DefaultMap<Rational<_>, Vec<_>> = DefaultMap::new();
    let mut same = vec![Vec::new(); n];
    for i in 0..n {
        let (k1, b1) = dice[i];
        for j in 0..i {
            let (k2, b2) = dice[j];
            if k1 * b2 == k2 * b1 {
                same[i].push(j);
                same[j].push(i);
            }
            if k1 > k2 {
                if b1 < b2 {
                    events[Rational::new(b1 - b2, k2 - k1)].push((i, j));
                }
            } else if k1 < k2 {
                if b1 > b2 {
                    events[Rational::new(b1 - b2, k2 - k1)].push((i, j));
                }
            }
        }
    }
    let mut events = events.into_iter().collect_vec();
    events.sort();
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&it| (-dice[it].1, -dice[it].0));
    let mut pos = vec![0; n];
    for i in 0..n {
        pos[order[i]] = i;
    }
    let mut sum_k = order.iter().take(k).map(|&it| dice[it].0).sum::<i64>();
    let mut sum_b = order.iter().take(k).map(|&it| dice[it].1).sum::<i64>();
    let mut ans = sum_k * sum_k + sum_b;
    for (_, pairs) in events {
        let mut last = n;
        let mut from = None;
        let mut to = None;
        let mut done = HashSet::new();
        let mut cur = Vec::new();
        for (i, j) in pairs.into_iter().rev() {
            if i != last {
                let mut reverse = |from: Option<usize>, to: Option<usize>| {
                    if !from.is_some() {
                        return;
                    }
                    let mut from = from.unwrap();
                    let mut to = to.unwrap();
                    while from < to {
                        let i = order[from];
                        pos[i] = to;
                        let j = order[to];
                        pos[j] = from;
                        order.swap(from, to);
                        if from < k && to >= k {
                            sum_k -= dice[i].0;
                            sum_b -= dice[i].1;
                            sum_k += dice[j].0;
                            sum_b += dice[j].1;
                        }
                        from += 1;
                        to -= 1;
                    }
                };
                reverse(from, to);
                from = None;
                to = None;
                last = i;
                for &x in &cur {
                    done.insert(x);
                }
                cur.clear();
                cur.push(i);
            }
            if done.contains(&i) {
                continue;
            }
            if from.is_none() {
                for &x in &same[i] {
                    from.minim(pos[x]);
                    to.maxim(pos[x]);
                    cur.push(x);
                }
            }
            cur.push(j);
            from.minim(pos[i]);
            to.maxim(pos[i]);
            from.minim(pos[j]);
            to.maxim(pos[j]);
        }
        let mut reverse = |from: Option<usize>, to: Option<usize>| {
            if !from.is_some() {
                return;
            }
            let mut from = from.unwrap();
            let mut to = to.unwrap();
            while from < to {
                let i = order[from];
                pos[i] = to;
                let j = order[to];
                pos[j] = from;
                order.swap(from, to);
                if from < k && to >= k {
                    sum_k -= dice[i].0;
                    sum_b -= dice[i].1;
                    sum_k += dice[j].0;
                    sum_b += dice[j].1;
                }
                from += 1;
                to -= 1;
            }
        };
        reverse(from, to);
        ans.maxim(sum_k * sum_k + sum_b);
    }
    type Mod = ModIntF;
    let ans = Mod::new_from_wide(ans) / Mod::new(36);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
