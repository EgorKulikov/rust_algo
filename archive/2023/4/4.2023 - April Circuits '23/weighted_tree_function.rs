//{"name":"Weighted Tree Function","group":"HackerEarth - April Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/april-circuits-23/algorithm/weighted-tree-function-2-0961d46e/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2 10\n1 3 2\n","output":"86\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WeightedTreeFunction"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut edges = input.read_vec::<(usize, usize, i32)>(n - 1);

    edges.sort_by_key(|&(_, _, w)| w);
    let mut dsu = DSU::new(n);
    type Mod = ModInt7;
    let mut weight = Vec::with_capacity(n);
    for i in 0..n {
        weight.push(Mod::from_index(i + 1));
    }
    let mut ans = Mod::zero();
    for (u, v, w) in edges {
        let u = dsu.get(u - 1);
        let v = dsu.get(v - 1);
        ans += weight[u] * weight[v] * Mod::from(w);
        let wv = weight[v];
        weight[u] += wv;
        weight[v] = weight[u];
        dsu.join(u, v);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
