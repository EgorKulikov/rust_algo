//{"name":"C. Слово на бумаге","group":"Codeforces - Codeforces Round 886 (Div. 4)","url":"https://codeforces.com/contest/1850/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n........\n........\n........\n........\n...i....\n........\n........\n........\n........\n.l......\n.o......\n.s......\n.t......\n........\n........\n........\n........\n........\n........\n........\n......t.\n......h.\n......e.\n........\n........\n........\n........\n........\n.......g\n.......a\n.......m\n.......e\na.......\na.......\na.......\na.......\na.......\na.......\na.......\na.......\n","output":"i\nlost\nthe\ngame\naaaaaaaa\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSlovoNaBumage"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let t = input.read_char_table(8, 8);

    out_line!(t.into_iter().filter(|&c| c != '.').collect::<String>());
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
