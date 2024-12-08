//{"name":"day_8","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_8"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(input.read_line());
    }

    let mut antennas = DefaultHashMap::<_, Vec<_>>::new();
    for i in data.indices() {
        for j in data[i].indices() {
            if data[i][j] != b'.' {
                antennas[data[i][j]].push((i as isize, j as isize));
            }
        }
    }

    //part 1
    {
        let n = data.len() as isize;
        let m = data[0].len() as isize;
        let mut ans = FxHashSet::default();
        for v in antennas.values() {
            for i in v.indices() {
                for j in v.indices() {
                    if i == j {
                        continue;
                    }
                    let (a, b) = v[i];
                    let (c, d) = v[j];
                    let x = 2 * a - c;
                    let y = 2 * b - d;
                    if x >= 0 && x < n && y >= 0 && y < m {
                        ans.insert((x, y));
                    }
                }
            }
        }
        out.print_line(ans.len());
    }

    //part 2
    {
        let n = data.len() as isize;
        let m = data[0].len() as isize;
        let mut ans = FxHashSet::default();
        for v in antennas.values() {
            for i in v.indices() {
                for j in v.indices() {
                    if i == j {
                        continue;
                    }
                    let (a, b) = v[i];
                    let (c, d) = v[j];
                    for k in 0.. {
                        let x = (k + 1) * a - k * c;
                        let y = (k + 1) * b - k * d;
                        if x >= 0 && x < n && y >= 0 && y < m {
                            ans.insert((x, y));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        out.print_line(ans.len());
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
