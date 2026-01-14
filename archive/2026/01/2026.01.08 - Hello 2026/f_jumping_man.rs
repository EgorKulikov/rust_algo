//{"name":"F. Jumping Man","group":"Codeforces - Hello 2026","url":"https://codeforces.com/contest/2183/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n3\nabb\n1 2\n1 3\n2\naa\n1 2\n4\nccbb\n1 2\n2 3\n2 4\n4\naaaa\n1 4\n4 2\n2 3\n10\ncacbcccbac\n1 2\n2 3\n3 4\n2 5\n1 6\n2 7\n3 8\n4 9\n8 10\n","output":"9 1 1\n5 1\n29 9 1 1\n69 5 1 19\n185 65 19 3 1 1 1 3 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let lca = graph.lca();
    let ch = Vec::with_gen(n, |v| {
        let mut cur = Vec::new();
        for e in &graph[v] {
            if Some(e.to()) != lca.parent(v) {
                cur.push(e.to());
            }
        }
        cur
    });
    type Mod = ModIntF;
    /*let mut mem = Memoization3d::new(n, n, 5, |mem, v1, v2, mode| -> Mod {
        match mode {
            0 => {
                let mut res = Mod::one();
                for to in ch[v1].copy_iter() {
                    res += mem.call(to, v2, 1);
                }
                res
            }
            1 => {
                let mut res = mem.call(v1, v2, 2);
                for to in ch[v1].copy_iter() {
                    res += mem.call(to, v2, 1);
                }
                res
            }
            2 => {
                let mut res = Mod::zero();
                for to in ch[v2].copy_iter() {
                    res += mem.call(v1, to, 3);
                }
                res
            }
            3 => {
                let mut res = Mod::zero();
                if s[v1] == s[v2] {
                    res += mem.call(v1, v2, 0);
                }
                for to in ch[v2].copy_iter() {
                    res += mem.call(v1, to, 3);
                }
                res
            }
            4 => {
                let mut res = mem.call(v1, v2, 3);
                for to in ch[v1].copy_iter() {
                    res += mem.call(to, v2, 4);
                }
                res
            }
            _ => unreachable!(),
        }
    });*/
    let mut mem = Arr3d::new(n, n, 5, Mod::zero());
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
        }
        let v1 = vert;
        let mut dfs2 = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert);
            }
            let v2 = vert;
            let mut res = Mod::one();
            for to in ch[v1].copy_iter() {
                res += mem[(to, v2, 1)];
            }
            mem[(v1, v2, 0)] = res;
            let mut res = Mod::zero();
            for to in ch[v2].copy_iter() {
                res += mem[(v1, to, 3)];
            }
            mem[(v1, v2, 2)] = res;
            let mut res = mem[(v1, v2, 2)];
            for to in ch[v1].copy_iter() {
                res += mem[(to, v2, 1)];
            }
            mem[(v1, v2, 1)] = res;
            let mut res = Mod::zero();
            if s[v1] == s[v2] {
                res += mem[(v1, v2, 0)];
            }
            for to in ch[v2].copy_iter() {
                res += mem[(v1, to, 3)];
            }
            mem[(v1, v2, 3)] = res;
            let mut res = mem[(v1, v2, 3)];
            for to in ch[v1].copy_iter() {
                res += mem[(to, v2, 4)];
            }
            mem[(v1, v2, 4)] = res;
        });
        dfs2.call(0, n);
    });
    dfs.call(0, n);
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        ans.push(mem[(i, i, 4)]);
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
