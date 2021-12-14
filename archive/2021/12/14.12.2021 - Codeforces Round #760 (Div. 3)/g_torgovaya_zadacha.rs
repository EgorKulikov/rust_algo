//{"name":"G. Торговая задача","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/G","interactive":false,"timeLimit":4500,"tests":[{"input":"3 4 5\n10 30 15\n12 31 14 18\n0 1 2 3 4\n","output":"55\n56\n60\n64\n64\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GTorgovayaZadacha"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let q = input.read();
    let a: Vec<u64> = input.read_vec(n);
    let b: Vec<u64> = input.read_vec(m);
    let k: Vec<u64> = input.read_vec(q);

    let mut cur_ans = a.iter().sum::<u64>();
    let mut things = a
        .into_iter()
        .map(|i| (i, true))
        .chain(b.into_iter().map(|i| (i, false)))
        .collect_vec();

    things.sort_unstable();
    let mut part_sums = vec![(0u64, 0usize)];
    for (i, our) in things.iter().cloned() {
        let (last_sum, last_q) = part_sums.last().unwrap().clone();
        part_sums.push((last_sum + i, last_q + if our { 1 } else { 0 }));
    }
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    enum EventInner {
        Join(usize),
        Query(usize),
    }
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    struct Event {
        at: u64,
        t: EventInner,
    }
    let mut events = Vec::new();
    for i in 1..(n + m) {
        events.push(Event {
            at: things[i].0 - things[i - 1].0,
            t: EventInner::Join(i),
        });
    }
    for (i, j) in k.into_iter().enumerate() {
        events.push(Event {
            at: j,
            t: EventInner::Query(i),
        })
    }
    events.sort_unstable();
    let mut ans = vec![0u64; q];
    let mut dsu = DSU::new(n + m);
    for Event { at: _, t } in events {
        match t {
            EventInner::Join(i) => {
                let left = (i - dsu.size(i - 1), i);
                let right = (i, i + dsu.size(i));
                for (from, to) in [left, right] {
                    let our = part_sums[to].1 - part_sums[from].1;
                    cur_ans -= part_sums[to].0 - part_sums[to - our].0;
                }
                let our = part_sums[right.1].1 - part_sums[left.0].1;
                cur_ans += part_sums[right.1].0 - part_sums[right.1 - our].0;
                dsu.join(i - 1, i);
            }
            EventInner::Query(i) => {
                ans[i] = cur_ans;
            }
        }
    }
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
