//{"name":"D. Divide And Show","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/D/","interactive":false,"timeLimit":2000,"tests":[{"input":"7 4\n4\n0 2 5 4\n0 2 7 2\n0 1 5 4\n2 0 7 1\n","output":"YES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDivideAndShow"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let k = input.read_usize();

    let mut map: DefaultMap<usize, Vec<usize>> = DefaultMap::new();
    let val = |x, y| {
        if y == 0 {
            x
        } else if x == n {
            n + y
        } else if y == m {
            2 * n + m - x
        } else {
            assert_eq!(x, 0);
            2 * n + 2 * m - y
        }
    };
    for i in 0..k {
        let x1 = input.read_usize();
        let y1 = input.read_usize();
        let x2 = input.read_usize();
        let y2 = input.read_usize();
        let v1 = val(x1, y1);
        map[v1].push(i);
        let v2 = val(x2, y2);
        map[v2].push(i);
    }
    let mut list = map.into_iter().collect_vec();
    list.sort();
    let list = list.into_iter().map(|(_, v)| v).collect_vec();
    for (v, u) in list
        .iter()
        .zip(list.iter().skip(1).chain(list.iter().take(1)))
    {
        let mut i = 0;
        let mut j = 0;
        while i < v.len() && j < u.len() {
            if v[i] == u[j] {
                out_line!("YES");
                return;
            }
            if v[i] < u[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
    }
    out_line!("NO");
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
