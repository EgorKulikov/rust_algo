//{"name":"E. Extracting Weights","group":"Universal Cup - The 3rd Universal Cup. Stage 20: Kunming","url":"https://contest.ucup.ac/contest/1871/problem/9866","interactive":false,"timeLimit":1000,"tests":[{"input":"4 1\n1 2\n2 3\n2 4\n1 2 3\n","output":"\n"},{"input":"5 2\n1 2\n2 3\n3 4\n3 5\n4 5 3 2\n","output":"\n"},{"input":"6 2\n1 2\n2 3\n3 4\n4 5\n4 6\n5 4 3 2 1\n","output":"\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EExtractingWeights"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::slice_ext::splits::Split;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut matrix = Vec::new();
    let mut copy_matrix = Vec::new();
    let mut paths = Vec::new();
    let mut set = BitSet::new(n - 1);
    for i in 0..n {
        let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, len: usize| {
            if vert > 0 {
                set.set(vert - 1);
            }
            if len == k {
                if vert > i {
                    matrix.push(set.clone());
                    copy_matrix.push(set.clone());
                    paths.push((i + 1, vert + 1));
                }
                if vert > 0 {
                    set.unset(vert - 1);
                }
                return;
            }
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert, len + 1);
            }
            if vert > 0 {
                set.unset(vert - 1);
            }
        });
        dfs.call(i, n, 0);
    }

    if matrix.is_empty() {
        out.print_line(false);
        return;
    }

    fn gauss(
        matrix: &mut Vec<BitSet>,
        copy_maxtrix: &mut Vec<BitSet>,
        paths: &mut Vec<(usize, usize)>,
        v: &mut Vec<u32>,
    ) -> bool {
        let n = matrix[0].len();
        if matrix.len() < n {
            return false;
        }
        for i in 0..n {
            for j in i..matrix.len() {
                if matrix[j][i] {
                    matrix.swap(i, j);
                    copy_maxtrix.swap(i, j);
                    paths.swap(i, j);
                    break;
                }
            }
            if !matrix[i][i] {
                return false;
            }
            for j in 0..matrix.len() {
                if i == j {
                    continue;
                }
                if matrix[j][i] {
                    let (j_m, i_m) = matrix.two_mut(j, i);
                    *j_m ^= &i_m;
                    v[j] ^= v[i];
                }
            }
        }
        true
    }
    let mut fake_v = vec![0; matrix.len()];
    if !gauss(&mut matrix, &mut copy_matrix, &mut paths, &mut fake_v) {
        out.print_line(false);
        return;
    }
    out.print_line(true);
    out.print(('?', n - 1));
    for i in 0..n - 1 {
        out.print(format!(" {} {}", paths[i].0, paths[i].1));
    }
    out.print_line(());
    out.flush();
    let mut v = input.read_unsigned_vec(n - 1);
    copy_matrix.truncate(n - 1);
    // gauss(&mut copy_matrix, &mut matrix, &mut paths, &mut v);
    assert!(gauss(&mut copy_matrix, &mut matrix, &mut paths, &mut v));
    out.print_line(('!', v));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
