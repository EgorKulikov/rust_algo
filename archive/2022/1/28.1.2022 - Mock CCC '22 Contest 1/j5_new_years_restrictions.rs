//{"name":"J5 - New Year's Restrictions","group":"DMOJ - Mock CCC '22 Contest 1","url":"https://dmoj.ca/problem/mccc3j5","interactive":false,"timeLimit":600,"tests":[{"input":"4 2\n1 2\n3 2\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"J5NewYearsRestrictions"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_usize_pair_vec(m).dec_by_one();

    let mut graph = vec![0usize; n];
    for (u, v) in edges {
        graph[u].set_bit(v);
        graph[v].set_bit(u);
    }
    let mut ans = 0;
    for i in 0..(1 << n) {
        let mut ok = true;
        for j in 0..n {
            if i.is_set(j) && (i & graph[j]) != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            ans.maxim(i.count_ones());
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
