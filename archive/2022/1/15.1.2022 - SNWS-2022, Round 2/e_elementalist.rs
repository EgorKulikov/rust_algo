//{"name":"E. Elementalist","group":"Yandex - SNWS-2022, Round 2","url":"https://contest.yandex.ru/snws2022/contest/23958/problems/E/","interactive":false,"timeLimit":3000,"tests":[{"input":"24\n0 0\n0 1\n0 2\n0 3\n0 4\n0 5\n1 0\n1 1\n1 2\n1 3\n1 4\n1 5\n2 0\n2 1\n2 2\n2 3\n2 4\n2 5\n3 0\n3 1\n3 2\n3 3\n3 4\n3 5\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EElementalist"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D8Dirs;
use algo_lib::misc::value_ref::ConstValueRef;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let pts: Vec<(isize, isize)> = input.read_vec(n);

    let mut p: Vec<DefaultMap<isize, Vec<usize>>> = vec![DefaultMap::new(); 8];
    for (&(dx, dy), p) in D8Dirs::val().iter().zip(p.iter_mut()) {
        for (j, &(x, y)) in pts.iter().enumerate() {
            let val = dy * x - dx * y;
            p[val].push(j);
        }
        for v in p.values_mut() {
            v.sort_by_key(|&i| pts[i].0 * dx + pts[i].1 * dy);
        }
    }
    let mut ans = 0;
    let mut cur = vec![0u64; n];
    let mut next = vec![0u64; n];
    for i in 0..n {
        cur.fill(0);
        cur[i] = 1;
        for p in &p {
            next.fill(0);
            for v in p.values() {
                let mut on_line = 0;
                for &j in v {
                    next[j] = on_line;
                    on_line += cur[j];
                }
            }
            swap(&mut cur, &mut next);
        }
        ans += cur[i];
    }
    out_line!(ans);
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
