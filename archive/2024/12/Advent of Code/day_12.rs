//{"name":"day_12","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_12"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
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
        let n = data.len();
        let m = data[0].len();
        let mut ans = 0;
        let mut done = Arr2d::new(n, m, false);

        for i in 0..n {
            for j in 0..m {
                if done[(i, j)] {
                    continue;
                }
                let mut area = 0;
                let mut internal = 0;
                let mut queue = vec![(i, j)];
                done[(i, j)] = true;
                while let Some((r, c)) = queue.pop() {
                    area += 1;
                    for (nr, nc) in D4::iter(r, c, n, m) {
                        if data[nr][nc] == data[r][c] {
                            internal += 1;
                            if !done[(nr, nc)] {
                                done[(nr, nc)] = true;
                                queue.push((nr, nc));
                            }
                        }
                    }
                }
                let perimeter = area * 4 - internal;
                ans += area * perimeter;
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let n = data.len();
        let m = data[0].len();
        let mut ans = 0;
        let mut done = Arr2d::new(n, m, false);

        for i in 0..n {
            for j in 0..m {
                if done[(i, j)] {
                    continue;
                }
                let mut area = 0;
                let mut queue = vec![(i, j)];
                done[(i, j)] = true;
                let mut fence = DefaultHashMap::<_, Vec<_>>::new();
                while let Some((r, c)) = queue.pop() {
                    area += 1;
                    for (nr, nc) in D4::iter(r, c, n, m) {
                        if data[nr][nc] == data[r][c] {
                            if !done[(nr, nc)] {
                                done[(nr, nc)] = true;
                                queue.push((nr, nc));
                            }
                        } else {
                            if r > nr {
                                fence[(r, 0)].push(c);
                            } else if r < nr {
                                fence[(nr, 1)].push(nc);
                            } else if c > nc {
                                fence[(c, 2)].push(r);
                            } else {
                                fence[(nc, 3)].push(r);
                            }
                        }
                    }
                    if r == 0 {
                        fence[(0, 0)].push(c);
                    }
                    if r == n - 1 {
                        fence[(n, 1)].push(c);
                    }
                    if c == 0 {
                        fence[(0, 2)].push(r);
                    }
                    if c == m - 1 {
                        fence[(m, 3)].push(r);
                    }
                }
                let mut perimeter = 0;
                for mut v in fence.into_values() {
                    v.sort();
                    v.dedup();
                    perimeter += 1;
                    for (a, b) in v.consecutive_iter() {
                        if b - a > 1 {
                            perimeter += 1;
                        }
                    }
                }
                ans += area * perimeter;
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
