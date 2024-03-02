//{"name":"Carrot Click Game","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/contest/problem/1238/3","interactive":false,"timeLimit":2000,"tests":[{"input":"2 4\n1 2\n2 5\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CarrotClickGame"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let boosts = input.read_size_pair_vec(n);

    let mut cur = DefaultHashMap::<usize, Option<usize>>::new();
    cur[1] = Some(0);
    let mut ways = Vec::new();
    for _ in 0..k {
        ways.clear();
        for (&k, &v) in cur.iter() {
            ways.push((k, v.unwrap()));
        }
        ways.sort();
        ways.reverse();
        let mut best = None;
        cur.clear();
        for &(k, v) in &ways {
            if !best.maxim(v) {
                continue;
            }
            cur[k].maxim(k + v);
            for &(a, b) in &boosts {
                if v >= a {
                    cur[k + b].maxim(v - a);
                }
            }
        }
    }
    let ans = cur.into_values().max().unwrap();
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
