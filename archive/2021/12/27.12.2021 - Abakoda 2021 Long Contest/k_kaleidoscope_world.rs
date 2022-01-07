//{"name":"K. Kaleidoscope World","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/K","interactive":false,"timeLimit":1000,"tests":[{"input":"3\njus102res\n2\nU 2\nLU 1\ndreeei\n1\nRU 4\nACSF313\n3\nLD 2\nRD 2\nU 1\n3\nACSF313 dreeei\njus102res ACSF313\njus102res dreeei\n","output":"5\n4\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KKaleidoscopeWorld"}}}

use algo_lib::collections::id::Id;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut id = Id::new();
    let mut pos = Vec::with_capacity(n);

    for _ in 0..n {
        id.get(input.read::<String>());
        let mut x = 0;
        let mut y = 0;
        let m = input.read_usize();
        for (dir, dist) in input.read_vec::<(String, i64)>(m) {
            match dir.as_str() {
                "U" => y += 2 * dist,
                "D" => y -= 2 * dist,
                "RU" => {
                    x += dist;
                    y += dist;
                }
                "RD" => {
                    x += dist;
                    y -= dist;
                }
                "LU" => {
                    x -= dist;
                    y += dist;
                }
                "LD" => {
                    x -= dist;
                    y -= dist;
                }
                _ => unreachable!(),
            }
        }
        pos.push((x, y));
    }
    let q = input.read_usize();
    for _ in 0..q {
        let from = pos[id.get(input.read_string())];
        let to = pos[id.get(input.read_string())];

        let dx = (from.0 - to.0).abs();
        let dy = (from.1 - to.1).abs();
        out_line!(dx.max((dx + dy) / 2));
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
