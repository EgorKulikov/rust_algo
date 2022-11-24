//{"name":"A. Клетчатая доска","group":"Yandex - Yandex Cup 2022: Алгоритм, спринт (квалификация)","url":"https://contest.yandex.ru/yacup/contest/42199/problems/?lang=ru&nc=HmLkkuvM","interactive":false,"timeLimit":1000,"tests":[{"input":"8 2\n","output":"Yes\n1 2 1 2 1 2 1 2\n2 1 2 1 2 1 2 1\n1 2 1 2 1 2 1 2\n2 1 2 1 2 1 2 1\n1 2 1 2 1 2 1 2\n2 1 2 1 2 1 2 1\n1 2 1 2 1 2 1 2\n2 1 2 1 2 1 2 1\n"},{"input":"2 1\n","output":"No\n"},{"input":"3 3\n","output":"Yes\n1 2 1\n2 3 2\n3 1 3\n"},{"input":"1 3\n","output":"No\n"},{"input":"5 3\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKletchatayaDoska"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    set_bool_output(BoolOutput::YesNo);
    if (n * n) % k != 0 {
        out_line!(false);
        return;
    }
    if n == 1 {
        out_line!(true);
        out_line!(1);
        return;
    }
    if k == 1 {
        out_line!(false);
        return;
    }
    out_line!(true);
    for i in 2.. {
        if k % i == 0 {
            let mut next = (1..=i).collect_vec();
            let ans = Arr2d::generate(n, n, |r, c| {
                let id = (r + c) % i;
                let res = next[id];
                next[id] += i;
                next[id] %= k;
                if next[id] == 0 {
                    next[id] = k;
                }
                res
            });
            out_line!(ans);
            return;
        }
    }
    unreachable!();
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
