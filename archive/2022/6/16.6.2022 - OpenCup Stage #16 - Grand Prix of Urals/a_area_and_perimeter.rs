//{"name":"A. Area and Perimeter","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/A/","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1/4\n3/8\n2/7\n1/2\n3/2\n","output":"YES\n1 1\n#\nYES\n3 4\n....\n.##.\n.#..\nNO\nYES\n3 3\n###\n#.#\n###\nYES\n8 8\n...####.\n.######.\n#######.\n#######.\n########\n.#######\n...#####\n....####\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAreaAndPerimeter"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read_string();

    let tokens = s.split("/").collect_vec();
    let p = tokens[0].parse::<usize>().unwrap();
    let q = tokens[1].parse::<usize>().unwrap();
    for r in 1..=100 {
        for c in 1..=r {
            let perimeter = (r + c) * 2;
            let mut area = perimeter * p;
            if area % q != 0 {
                continue;
            }
            area /= q;
            if area >= r + c - 1 && area <= r * c {
                out_line!("YES");
                out_line!(r, c);
                let mut ans = Arr2d::new(r, c, '.');
                for i in 0..r {
                    ans[(i, 0)] = '#';
                }
                for i in 0..c {
                    ans[(0, i)] = '#';
                }
                let mut rem = area - (r + c - 1);
                for i in 1..r {
                    for j in 1..c {
                        if rem > 0 {
                            rem -= 1;
                            ans[(i, j)] = '#';
                        }
                    }
                }
                output().print_table(&ans);
                return;
            }
        }
    }
    out_line!("NO");
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
