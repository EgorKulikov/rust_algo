//{"name":"task_b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"task_b"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_vec::<(usize, usize)>(m).dec_by_one();

    let mut deg = vec![0; n];
    let mut dsu = DSU::new(n);
    for (u, v) in edges {
        deg[u] += 1;
        deg[v] += 1;
        dsu.join(u, v);
    }
    if dsu.count() == 1 && deg.into_iter().filter(|c| *c == 2).count() == n - 2 {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
