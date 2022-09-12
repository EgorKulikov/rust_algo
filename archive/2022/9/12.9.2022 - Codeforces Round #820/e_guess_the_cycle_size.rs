//{"name":"E. Guess the Cycle Size","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/E","interactive":true,"timeLimit":1000,"tests":[{"input":"1\n\n2\n\n-1\n","output":"? 1 2\n\n? 1 3\n\n? 1 4\n\n! 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EGuessTheCycleSize"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    for i in 2..=26 {
        out_line!("?", 1, i);
        let a = input.read_long();
        if a == -1 {
            out_line!("!", i - 1);
            return;
        }
        out_line!("?", i, 1);
        let b = input.read_long();
        if a != b {
            out_line!("!", a + b);
            return;
        }
    }
    // let mut map: DefaultMap<_, usize> = DefaultMap::new();
    // for i in (2..33).step_by(2) {
    //     out_line!("?", 1, i);
    //     out_line!("?", 1, i + 1);
    //     out_line!("?", i, i + 1);
    //
    //     let a = input.read_long();
    //     if a == -1 {
    //         out_line!("!", i - 1);
    //         return;
    //     }
    //     let b = input.read_long();
    //     if b == -1 {
    //         out_line!("!", i);
    //         return;
    //     }
    //     let c = input.read_long();
    //
    //     map[a + b + c] += 1;
    // }
    //
    // let mut best = None;
    // let mut ans = 0;
    // for (&k, &v) in map.iter() {
    //     if best.maxim(v + map[2 * k]) {
    //         ans = k;
    //     }
    // }
    // out_line!("!", ans);
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
