//{"name":"A. Astronomy and Bugs","group":"Yandex - SNWS-2022, Round 3","url":"https://contest.yandex.ru/snws2022/contest/23959/problems/A/","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1\n2 2\n3 3\n1 2\n","output":"8\n"},{"input":"10\n19 11\n8 22\n9 7\n8 17\n20 24\n11 11\n16 5\n22 9\n19 22\n20 1\n","output":"53\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAstronomyAndBugs"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let pts = input.read_vec::<(u32, u32)>(n);

    let mut xs: DefaultMap<_, usize> = DefaultMap::new();
    let mut ys: DefaultMap<_, usize> = DefaultMap::new();
    for &(x, y) in &pts {
        xs[x] += 1;
        ys[y] += 1;
    }
    let mut ans = xs.len() * ys.len();
    for (x, y) in pts {
        if xs[x] == 1 && ys[y] == 1 {
            ans -= 1;
        }
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
