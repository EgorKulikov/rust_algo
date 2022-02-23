//{"name":"D. Destructive Game","group":"Yandex - Stage 10: Grand Prix of Kyoto","url":"https://official.contest.yandex.ru/opencupXXII/contest/35263/problems/D/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n10 3\n7 4\n","output":"Bob\n"},{"input":"16\n903 5\n246 38\n884 12\n752 10\n200 17\n483 6\n828 27\n473 21\n983 35\n953 36\n363 35\n101 3\n34 23\n199 8\n134 2\n932 28\n","output":"Alice\n"},{"input":"16\n35 37\n852 17\n789 37\n848 40\n351 27\n59 32\n271 11\n395 20\n610 3\n631 33\n543 14\n256 28\n48 8\n277 24\n748 38\n109 40\n","output":"Bob\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDestructiveGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    let mut ans = 0;
    for _ in 0..n {
        let mut a = input.read_int();
        let b = input.read_int();
        if b % 2 == 1 {
            ans ^= a % 2;
        } else {
            a %= b + 1;
            if a == b {
                ans ^= 2;
            } else {
                ans ^= a % 2;
            }
        }
    }
    if ans == 0 {
        out_line!("Bob");
    } else {
        out_line!("Alice");
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
