//{"name":"Целочисленное разнообразие","group":"Codeforces","url":"https://m1.codeforces.com/contest/1616/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 1 2 2\n3\n1 2 3\n2\n0 0\n","output":"4\n3\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TselochislennoeRaznoobrazie"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    let mut map: DefaultMap<_, i32> = DefaultMap::new();
    for i in a {
        map[i.abs()] += 1;
    }
    out_line!(map
        .into_iter()
        .map(|(k, v)| if k == 0 { v.min(1) } else { v.min(2) })
        .sum::<i32>());
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
