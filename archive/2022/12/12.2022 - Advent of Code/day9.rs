//{"name":"day9","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day9"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    // let mut head_x = 0;
    // let mut head_y = 0;
    // let mut tail_x = 0;
    // let mut tail_y = 0;
    let mut rope = vec![(0, 0); 10];
    let mut visited = HashSet::new();

    while !input.is_exhausted() {
        let dir = input.read_char();
        let len = input.read_size();
        input.skip_whitespace();

        for _ in 0..len {
            match dir {
                // 'R' => head_x += 1,
                // 'L' => head_x -= 1,
                // 'U' => head_y += 1,
                // 'D' => head_y -= 1,
                'R' => rope[0].0 += 1,
                'L' => rope[0].0 -= 1,
                'U' => rope[0].1 += 1,
                'D' => rope[0].1 -= 1,
                _ => unreachable!(),
            }
            for i in 1..rope.len() {
                let (head, tail) = rope.split_at_mut(i);
                let (head_x, head_y) = head[i - 1];
                let (tail_x, tail_y) = &mut tail[0];
                if head_x > *tail_x + 1 {
                    *tail_x = head_x - 1;
                    if head_y > *tail_y {
                        *tail_y += 1;
                    } else if head_y < *tail_y {
                        *tail_y -= 1;
                    }
                }
                if head_x < *tail_x - 1 {
                    *tail_x = head_x + 1;
                    if head_y > *tail_y {
                        *tail_y += 1;
                    } else if head_y < *tail_y {
                        *tail_y -= 1;
                    }
                }
                if head_y > *tail_y + 1 {
                    *tail_y = head_y - 1;
                    if head_x > *tail_x {
                        *tail_x += 1;
                    } else if head_x < *tail_x {
                        *tail_x -= 1;
                    }
                }
                if head_y < *tail_y - 1 {
                    *tail_y = head_y + 1;
                    if head_x > *tail_x {
                        *tail_x += 1;
                    } else if head_x < *tail_x {
                        *tail_x -= 1;
                    }
                }
            }
            /*if head_x > tail_x + 1 {
                tail_x = head_x - 1;
                tail_y = head_y;
            }
            if head_x < tail_x - 1 {
                tail_x = head_x + 1;
                tail_y = head_y;
            }
            if head_y > tail_y + 1 {
                tail_x = head_x;
                tail_y = head_y - 1;
            }
            if head_y < tail_y - 1 {
                tail_x = head_x;
                tail_y = head_y + 1;
            }
            visited.insert((tail_x, tail_y));*/
            visited.insert(rope[9]);
        }
    }

    out_line!(visited.len());
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
}
//END MAIN
