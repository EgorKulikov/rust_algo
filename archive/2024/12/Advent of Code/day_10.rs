//{"name":"day_10","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_10"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(input.read_line());
    }

    // part 1
    {
        let mut score = Arr2d::new(data.len(), data[0].len(), FxHashSet::default());
        for i in score.rows() {
            for j in score.cols() {
                if data[i][j] == b'9' {
                    score[(i, j)].insert((i, j));
                }
            }
        }
        for pos in (b'0'..=b'8').rev() {
            for i in score.rows() {
                for j in score.cols() {
                    if data[i][j] == pos {
                        for (di, dj) in D4::iter(i, j, score.d1(), score.d2()) {
                            if data[di][dj] == pos + 1 {
                                let from = score[(di, dj)].clone();
                                score[(i, j)].extend(&from);
                            }
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for i in score.rows() {
            for j in score.cols() {
                if data[i][j] == b'0' {
                    ans += score[(i, j)].len();
                }
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut score = Arr2d::new(data.len(), data[0].len(), 0);
        for i in score.rows() {
            for j in score.cols() {
                if data[i][j] == b'9' {
                    score[(i, j)] = 1;
                }
            }
        }
        for pos in (b'0'..=b'8').rev() {
            for i in score.rows() {
                for j in score.cols() {
                    if data[i][j] == pos {
                        for (di, dj) in D4::iter(i, j, score.d1(), score.d2()) {
                            if data[di][dj] == pos + 1 {
                                let from = score[(di, dj)];
                                score[(i, j)] += from;
                            }
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for i in score.rows() {
            for j in score.cols() {
                if data[i][j] == b'0' {
                    ans += score[(i, j)];
                }
            }
        }
        out.print_line(ans);
    }
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
