//{"name":"B. Boarding","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/B/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n... ...\n... ...\nX.. .X.\n4\n6 5 2 1\n","output":"YES\naba aba\nacb acb\nXdb .X.\n"},{"input":"3\n... ...\n.XX ...\nX.. ..X\n2\n9 2\n","output":"YES\naba aba\naXX a.a\nX.a a.X\n"},{"input":"3\n... ...\n.XX ...\nX.. ..X\n2\n2 10\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBoarding"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut seats = input.read_vec::<Str>(2 * n);
    let k = input.read_usize();
    let groups = input.read_usize_vec(k);

    let mut groups = groups
        .into_iter()
        .enumerate()
        .map(|(i, g)| (g, b'a' + i.into_u8()))
        .collect_vec();
    groups.sort_unstable();
    groups.reverse();
    let mut singles = VecDeque::new();
    let mut pairs = VecDeque::new();
    let mut triples = VecDeque::new();
    for (j, s) in seats.iter().enumerate() {
        if s[1] == b'X' {
            for i in (0..3).step_by(2) {
                if s[i] == b'.' {
                    singles.push_back((j, i));
                }
            }
        } else {
            if s[0] == b'.' {
                if s[2] == b'.' {
                    triples.push_back(j);
                } else {
                    pairs.push_back((j, 0));
                }
            } else {
                if s[2] == b'.' {
                    pairs.push_back((j, 2));
                } else {
                    singles.push_back((j, 1));
                }
            }
        }
    }
    for (mut sz, marker) in groups {
        if sz > singles.len() + pairs.len() + triples.len() * 2 {
            out_line!("NO");
            return;
        }
        let mut add_singles = Vec::new();
        while let Some(i) = triples.pop_back() {
            if sz > 0 && sz <= 1 + triples.len() + pairs.len() {
                sz -= 1;
                seats[i][1] = marker;
                add_singles.push((i, 0));
                add_singles.push((i, 2));
            } else if sz >= 2 {
                sz -= 2;
                seats[i][0] = marker;
                seats[i][2] = marker;
                add_singles.push((i, 1));
            } else {
                triples.push_back(i);
                break;
            }
        }
        while let Some((i, j)) = pairs.pop_back() {
            if sz > 0 {
                sz -= 1;
                seats[i][j] = marker;
                add_singles.push((i, 1));
            } else {
                pairs.push_back((i, j));
                break;
            }
        }
        while let Some((i, j)) = singles.pop_back() {
            if sz > 0 {
                sz -= 1;
                seats[i][j] = marker;
            } else {
                singles.push_back((i, j));
                break;
            }
        }
        for (i, j) in add_singles {
            singles.push_back((i, j));
        }
    }
    out_line!("YES");
    for (s1, s2) in seats.consecutive_iter().step_by(2) {
        out_line!(s1, s2);
    }
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
