//{"name":"Connecting Wells","group":"CodeChef - START164A","url":"https://www.codechef.com/START164A/problems/SPC2025Q6","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1 1\n1 2\n2\n1 1\n2 2\n3\n1 5\n4 3\n2 4\n1\n1 1\n","output":"1\n1\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ConnectingWells"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let wells = input.read_size_pair_vec(n);

    if n == 1 {
        out.print_line(0);
        return;
    }
    let mut edges = Vec::new();
    for i in 0..n {
        let (x0, y0) = wells[i];
        for j in 0..i {
            let (x1, y1) = wells[j];
            let dx = x0.abs_diff(x1);
            let dy = y0.abs_diff(y1);
            if dx == 0 {
                edges.push(((dy + 1) / 2, i, j));
            } else if dy == 0 {
                edges.push(((dx + 1) / 2, i, j));
            } else {
                edges.push((dx.max(dy), i, j));
            }
        }
    }
    edges.sort();
    let mut dsu = DSU::new(n);
    for (d, i, j) in edges {
        dsu.join(i, j);
        if dsu.set_count() == 1 {
            out.print_line(d);
            return;
        }
    }
    unreachable!();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
