//{"name":"day17","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day17"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let mut patern = vec![0u16; 7];
    let mut height = 0;
    let figures = [
        vec![15u16],
        vec![2, 7, 2],
        vec![7, 4, 4],
        vec![1, 1, 1, 1],
        vec![3, 3],
    ];
    let mut at = 0;

    // const END: usize = 2022;
    const END: usize = 1000000000000;
    let mut delta = vec![Vec::new(); s.len()];
    let mut i = 0;
    let mut add_ans = 0;
    while i < END {
        if i % 5 == 0 {
            if at < 10 {
                delta[at].push((i, height + add_ans));
                if delta[at].len() == 2 {
                    let di = delta[at][1].0 - delta[at][0].0;
                    let d_ans = delta[at][1].1 - delta[at][0].1;
                    let times = (END - 1 - i) / di;
                    add_ans += times * d_ans;
                    i += times * di;
                }
            }
        }
        let mut shift = 2;
        let id = i % figures.len();
        let mut cur_height = height + 3;

        let check = |cur_height, shift: usize| -> bool {
            assert!(cur_height + figures[id].len() <= patern.len());
            for (&f, &p) in figures[id].iter().zip(patern.iter().skip(cur_height)) {
                if (f << shift) >= 128 {
                    return false;
                }
                if (f << shift) & p != 0 {
                    return false;
                }
            }
            true
        };

        loop {
            match s[at] {
                b'<' => {
                    if shift > 0 && check(cur_height, shift - 1) {
                        shift -= 1;
                    }
                }
                b'>' => {
                    if check(cur_height, shift + 1) {
                        shift += 1;
                    }
                }
                _ => unreachable!(),
            }
            at += 1;
            if at == s.len() {
                at = 0;
            }
            if cur_height > 0 && check(cur_height - 1, shift) {
                cur_height -= 1;
            } else {
                let mut add = 0;
                for ((j, &f), p) in figures[id]
                    .iter()
                    .enumerate()
                    .zip(patern.iter_mut().skip(cur_height))
                {
                    if cur_height + j == height {
                        height += 1;
                        add += 1;
                    }
                    assert_eq!(*p & (f << shift), 0);
                    *p |= f << shift;
                }
                if add > 0 {
                    patern.resize(patern.len() + add, 0);
                }
                break;
            }
        }

        i += 1;
    }

    out_line!(height + add_ans);
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
