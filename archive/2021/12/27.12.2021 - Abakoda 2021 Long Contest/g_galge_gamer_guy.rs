//{"name":"G. Galge Gamer Guy","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n.A..\n","output":"1 0 1 2\n-1 -1 -1 -1\n"},{"input":"5\nC..CA\n","output":"-1 -1 -1 -1 0\n0 1 1 0 -1\n"},{"input":"6\nCA...C\n","output":"-1 0 1 2 3 -1\n0 -1 3 2 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GGalgeGamerGuy"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let map: Str = input.read();

    enum Girl {
        None,
        Alexa,
        Cecille,
    }
    impl From<u8> for Girl {
        fn from(c: u8) -> Self {
            match c {
                b'.' => Girl::None,
                b'A' => Girl::Alexa,
                b'C' => Girl::Cecille,
                _ => unreachable!(),
            }
        }
    }
    let mut alexa = vec![None; n];
    let mut cecille = vec![None; n];
    let mut left_girl = Girl::None;
    let mut left_dist = 0;
    let (mut right_girl, mut right_dist) = {
        let mut girl = Girl::None;
        let mut dist = n;
        for (i, c) in map.iter().enumerate() {
            if c != b'.' {
                girl = c.into();
                dist = i;
                break;
            }
        }
        (girl, dist)
    };
    for (i, c) in map.iter().enumerate() {
        if c != b'.' {
            left_girl = c.into();
            left_dist = 0;
        }
        match left_girl {
            Girl::None => {}
            Girl::Alexa => {
                alexa[i].minim(left_dist);
            }
            Girl::Cecille => {
                cecille[i].minim(left_dist);
            }
        }
        match right_girl {
            Girl::None => {}
            Girl::Alexa => {
                alexa[i].minim(right_dist);
            }
            Girl::Cecille => {
                cecille[i].minim(right_dist);
            }
        }
        if c != b'.' {
            right_girl = Girl::None;
            right_dist = n - i;
            for (j, c) in map[i..].iter().cloned().enumerate().skip(1) {
                if c != b'.' {
                    right_girl = c.into();
                    right_dist = j;
                    break;
                }
            }
        }
        right_dist -= 1;
        left_dist += 1;
    }
    out_line!(alexa);
    out_line!(cecille);
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
