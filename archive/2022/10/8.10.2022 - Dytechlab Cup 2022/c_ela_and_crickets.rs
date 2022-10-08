//{"name":"C. Ela and Crickets","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n8\n7 2 8 2 7 1\n5 1\n8\n2 2 1 2 2 1\n5 5\n8\n2 2 1 2 2 1\n6 6\n8\n1 1 1 2 2 1\n5 5\n8\n2 2 1 2 2 1\n8 8\n8\n8 8 8 7 7 8\n4 8\n","output":"YES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CElaAndCrickets"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let pos = input.read_usize_pair_vec(3);
    let target = input.read::<(usize, usize)>();

    let main_row = {
        let mut v = pos.iter().map(|p| p.0).collect::<Vec<_>>();
        v.sort();
        v[1]
    };
    let main_col = {
        let mut v = pos.iter().map(|p| p.1).collect::<Vec<_>>();
        v.sort();
        v[1]
    };
    if target.0 % 2 != main_row % 2 && target.1 % 2 != main_col % 2 {
        out_line!(false);
        return;
    }
    if target.0 != main_row
        && target.1 != main_col
        && (main_row == 1 || main_row == n)
        && (main_col == 1 || main_col == n)
    {
        out_line!(false);
        return;
    }
    out_line!(true);
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
}
//END MAIN
