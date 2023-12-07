//{"name":"day3","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day3"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D8;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut table = Vec::new();
    while !input.is_exhausted() {
        table.push(input.read_str());
    }

    {
        // part 1
        let mut ans = 0;
        let mut map = DefaultHashMap::<_, Vec<_>>::new();
        for i in table.indices() {
            let mut j = 0;
            while j < table[i].len() {
                if table[i][j].is_ascii_digit() {
                    let mut k = j;
                    let mut cur = 0i64;
                    let mut account = false;
                    while k < table[i].len() && table[i][k].is_ascii_digit() {
                        cur = cur * 10 + (table[i][k] as i64 - '0' as i64);
                        for (r, c) in D8::iter(i, k, table.len(), table[i].len()) {
                            if table[r][c] != b'.' && !table[r][c].is_ascii_digit() {
                                account = true;
                            }
                        }
                        k += 1;
                    }
                    if account {
                        ans += cur;
                    }
                    j = k;
                }
                j += 1;
            }
        }
        out.print_line(ans);
    }

    {
        // part 2
        let mut ans = 0;
        let mut map = DefaultHashMap::<_, Vec<_>>::new();
        for i in table.indices() {
            let mut j = 0;
            while j < table[i].len() {
                if table[i][j].is_ascii_digit() {
                    let mut k = j;
                    let mut cur = 0i64;
                    let mut gears = Vec::new();
                    while k < table[i].len() && table[i][k].is_ascii_digit() {
                        cur = cur * 10 + (table[i][k] as i64 - '0' as i64);
                        for (r, c) in D8::iter(i, k, table.len(), table[i].len()) {
                            if table[r][c] == b'*' {
                                gears.push((r, c));
                            }
                        }
                        k += 1;
                    }
                    gears.sort();
                    gears.dedup();
                    for (r, c) in gears {
                        map[(r, c)].push(cur);
                    }
                    j = k;
                }
                j += 1;
            }
        }
        for (_, v) in map {
            if v.len() == 2 {
                ans += v[0] * v[1];
            }
        }
        out.print_line(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
