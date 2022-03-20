//{"name":"E. Арифметические операции","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"https://codeforces.com/contest/1654/problem/E","interactive":false,"timeLimit":5000,"tests":[{"input":"9\n3 2 7 8 6 9 5 4 1\n","output":"6\n"},{"input":"14\n19 2 15 8 9 14 17 13 4 14 4 11 15 7\n","output":"10\n"},{"input":"10\n100000 1 60000 2 20000 4 8 16 32 64\n","output":"7\n"},{"input":"4\n10000 20000 10000 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EArifmeticheskieOperatsii"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    const BUBEN: i32 = 300;

    let n = input.read_usize();
    let a = input.read_int_vec(n);

    let mut ans = 1;
    let mut map: DefaultMap<i32, usize> = DefaultMap::new();
    for i in -BUBEN..=BUBEN {
        map.clear();
        for (j, &v) in a.iter().enumerate() {
            map[v + i * j.into_i32()] += 1;
        }
        for &v in map.values() {
            ans.maxim(v);
        }
    }
    for (i, &v) in a.iter().enumerate() {
        map.clear();
        for (j, &w) in a.iter().enumerate().skip(i + 1) {
            let min_delta = (j - i).into_i32() * BUBEN;
            if v + min_delta > 100_000 && v <= min_delta {
                break;
            }
            let delta = w - v;
            let len_delta = (j - i).into_i32();
            if delta % len_delta == 0 {
                map[delta / len_delta] += 1;
            }
        }
        for &v in map.values() {
            ans.maxim(v + 1);
        }
    }
    out_line!(n - ans);
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
