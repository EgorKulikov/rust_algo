//{"name":"C. Правильная скобочная последовательность Jatayu","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n()\n3\n()(())\n3\n((()))\n4\n(())(())\n","output":"1\n2\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPravilnayaSkobochnayaPosledovatelnostJatayu"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s = input.read_vec::<char>(2 * n);

    let mut dsu = DSU::new(2 * n);
    let mut state = Vec::new();
    state.push((0, 0));
    for i in 0..2 * n {
        if s[i] == '(' {
            state.last_mut().unwrap().1 = i;
            state.push((i + 1, i + 1));
        } else {
            state.pop();
            let &(l, r) = state.last().unwrap();
            dsu.join(l, i);
            dsu.join(r, i);
        }
    }
    out_line!(dsu.count());
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
