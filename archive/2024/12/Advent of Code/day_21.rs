//{"name":"day_21","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_21"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::HashMap;

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::string::str::StrReader;
use algo_lib::value_ref;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(input.read_line());
    }

    let numpad = [b"789", b"456", b"123", b" 0A"];
    // let keypad = [b" ^A", b"<v>"];

    // part 1
    {
        value_ref!(Cache CACHE: HashMap<((usize, usize), (usize, usize), usize), usize> = HashMap::new());
        fn do_move(from: (usize, usize), to: (usize, usize), depth: usize) -> usize {
            if let Some(&v) = Cache::val_mut().get(&(from, to, depth)) {
                return v;
            }
            let (x1, y1) = from;
            let (x2, y2) = to;

            if depth == 0 {
                return x1.abs_diff(x2) + y1.abs_diff(y2) + 1;
            }
            let mut ans = usize::MAX;
            if y1 != 0 || (depth == 2 && x2 != 3) || (depth != 2 && x2 != 0) {
                let mut cur = 0;
                let mut pos = (0, 2);
                for _ in x1..x2 {
                    cur += do_move(pos, (1, 1), depth - 1);
                    pos = (1, 1);
                }
                for _ in x2..x1 {
                    cur += do_move(pos, (0, 1), depth - 1);
                    pos = (0, 1);
                }
                for _ in y1..y2 {
                    cur += do_move(pos, (1, 2), depth - 1);
                    pos = (1, 2);
                }
                for _ in y2..y1 {
                    cur += do_move(pos, (1, 0), depth - 1);
                    pos = (1, 0);
                }
                ans = cur + do_move(pos, (0, 2), depth - 1);
            }
            if y2 != 0 || (depth == 2 && x1 != 3) || (depth != 2 && x1 != 0) {
                let mut cur = 0;
                let mut pos = (0, 2);
                for _ in y1..y2 {
                    cur += do_move(pos, (1, 2), depth - 1);
                    pos = (1, 2);
                }
                for _ in y2..y1 {
                    cur += do_move(pos, (1, 0), depth - 1);
                    pos = (1, 0);
                }
                for _ in x1..x2 {
                    cur += do_move(pos, (1, 1), depth - 1);
                    pos = (1, 1);
                }
                for _ in x2..x1 {
                    cur += do_move(pos, (0, 1), depth - 1);
                    pos = (0, 1);
                }
                ans.minim(cur + do_move(pos, (0, 2), depth - 1));
            }
            Cache::val_mut().insert((from, to, depth), ans);
            ans
        }
        let mut ans = 0;
        for line in &data {
            value_ref!(Cache CACHE: HashMap<((usize, usize), (usize, usize), usize), usize> = HashMap::new());
            fn do_move(from: (usize, usize), to: (usize, usize), depth: usize) -> usize {
                if let Some(&v) = Cache::val_mut().get(&(from, to, depth)) {
                    return v;
                }
                let (x1, y1) = from;
                let (x2, y2) = to;

                if depth == 0 {
                    return x1.abs_diff(x2) + y1.abs_diff(y2) + 1;
                }
                let mut ans = usize::MAX;
                if y1 != 0 || (depth == 25 && x2 != 3) || (depth != 25 && x2 != 0) {
                    let mut cur = 0;
                    let mut pos = (0, 2);
                    for _ in x1..x2 {
                        cur += do_move(pos, (1, 1), depth - 1);
                        pos = (1, 1);
                    }
                    for _ in x2..x1 {
                        cur += do_move(pos, (0, 1), depth - 1);
                        pos = (0, 1);
                    }
                    for _ in y1..y2 {
                        cur += do_move(pos, (1, 2), depth - 1);
                        pos = (1, 2);
                    }
                    for _ in y2..y1 {
                        cur += do_move(pos, (1, 0), depth - 1);
                        pos = (1, 0);
                    }
                    ans = cur + do_move(pos, (0, 2), depth - 1);
                }
                if y2 != 0 || (depth == 25 && x1 != 3) || (depth != 25 && x1 != 0) {
                    let mut cur = 0;
                    let mut pos = (0, 2);
                    for _ in y1..y2 {
                        cur += do_move(pos, (1, 2), depth - 1);
                        pos = (1, 2);
                    }
                    for _ in y2..y1 {
                        cur += do_move(pos, (1, 0), depth - 1);
                        pos = (1, 0);
                    }
                    for _ in x1..x2 {
                        cur += do_move(pos, (1, 1), depth - 1);
                        pos = (1, 1);
                    }
                    for _ in x2..x1 {
                        cur += do_move(pos, (0, 1), depth - 1);
                        pos = (0, 1);
                    }
                    ans.minim(cur + do_move(pos, (0, 2), depth - 1));
                }
                Cache::val_mut().insert((from, to, depth), ans);
                ans
            }
            let mut cur = 0;
            let mut pos = (3, 2);
            let mut num = 0;
            for c in line.iter() {
                'outer: for i in 0..4 {
                    for j in 0..3 {
                        if numpad[i][j] == c {
                            cur += do_move(pos, (i, j), 2);
                            pos = (i, j);
                            break 'outer;
                        }
                    }
                }
                if c.is_ascii_digit() {
                    num *= 10;
                    num += (c - b'0') as usize;
                }
            }
            eprintln!("{} {}", cur, num);
            ans += cur * num;
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut ans = 0;
        for line in &data {
            let mut cur = 0;
            let mut pos = (3, 2);
            let mut num = 0;
            for c in line.iter() {
                'outer: for i in 0..4 {
                    for j in 0..3 {
                        if numpad[i][j] == c {
                            cur += do_move(pos, (i, j), 25);
                            pos = (i, j);
                            break 'outer;
                        }
                    }
                }
                if c.is_ascii_digit() {
                    num *= 10;
                    num += (c - b'0') as usize;
                }
            }
            eprintln!("{} {}", cur, num);
            ans += cur * num;
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
