//{"name":"B1. Копирование файлов I","group":"Codeforces - VK Cup 2022 - Квалификация (Engine)","url":"https://codeforces.com/contest/1769/problem/B1","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n6\n","output":"0\n16\n33\n50\n66\n83\n100\n"},{"input":"2\n100 500\n","output":"0\n95\n96\n97\n98\n99\n100\n"},{"input":"4\n1000 2 2 998\n","output":"0\n50\n99\n100\n"},{"input":"6\n170 130 400 256 30 100\n","output":"0\n17\n43\n44\n84\n90\n99\n100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B1KopirovanieFailovI"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let sum: i64 = a.iter().sum();
    for i in 0..=100 {
        let from = (sum * i + 99) / 100;
        let to = (sum * (i + 1) + 99) / 100;

        let mut so_far = 0;
        for &j in &a {
            let cur_from = (j * i + 99) / 100 + so_far;
            let cur_to = (j * (i + 1) + 99) / 100 + so_far;
            if !(cur_from >= to || cur_to <= from || cur_from == cur_to) {
                out_line!(i);
                break;
            }
            so_far += j;
        }
    }
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
