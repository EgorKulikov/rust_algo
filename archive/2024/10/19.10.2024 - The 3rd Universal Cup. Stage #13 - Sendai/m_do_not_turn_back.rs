//{"name":"M. Do Not Turn Back","group":"Universal Cup - The 3rd Universal Cup. Stage 13: Sendai","url":"https://contest.ucup.ac/contest/1812/problem/9488","interactive":false,"timeLimit":1000,"tests":[{"input":"6 8 5\n1 2\n1 3\n2 3\n2 4\n3 5\n4 5\n4 6\n5 6\n","output":"2\n"},{"input":"11 11 2023\n1 2\n2 3\n3 4\n4 5\n5 6\n6 7\n7 8\n8 9\n9 10\n10 11\n1 11\n","output":"1\n"},{"input":"7 21 1000000000\n1 2\n1 3\n1 4\n1 5\n1 6\n1 7\n2 3\n2 4\n2 5\n2 6\n2 7\n3 4\n3 5\n3 6\n3 7\n4 5\n4 6\n4 7\n5 6\n5 7\n6 7\n","output":"405422475\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MDoNotTurnBack"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    type Mod = ModIntF;
    let graph = Graph::from_biedges(n, &edges);
    let mat1 = Matrix::zero(2 * n, 2 * n).do_with(|mat| {
        for i in 0..n {
            mat[(i + n, i)] = Mod::one();
            mat[(i, i + n)] = -Mod::from_index(graph[i].len()) + Mod::one();
            for e in &graph[i] {
                mat[(i, e.to())] = Mod::one();
            }
        }
    });
    let mat2 = Matrix::zero(2 * n, 2 * n).do_with(|mat| {
        for i in 0..n {
            mat[(i + n, i)] = Mod::one();
            mat[(i, i + n)] = -Mod::from_index(graph[i].len());
            for e in &graph[i] {
                mat[(i, e.to())] = Mod::one();
            }
        }
    });
    let pw = Matrix::power(&mat1, k - 1);
    let res = Matrix::mult(&mat2, &pw);
    out.print_line(res[(0, n - 1)]);
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
