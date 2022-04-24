//{"name":"Hamiltonian Tour","group":"Google Coding Competitions - Round B 2022 - Kick Start 2022","url":"https://codingcompetitions.withgoogle.com/kickstart/round/00000000008caa74/0000000000acf318","interactive":false,"timeLimit":25000,"tests":[{"input":"3\n1 1\n*\n2 2\n**\n*#\n3 4\n****\n*#*#\n****\n","output":"Case #1: SENW\nCase #2: SSSENNEENWWW\nCase #3: ESSSSEEENNNWWNEEEEESWWSSSEESWWWWWWWNNNNN\n"},{"input":"3\n3 1\n*\n*\n#\n1 3\n*#*\n3 4\n**#*\n**#*\n****\n","output":"Case #1: SSSENNNW\nCase #2: IMPOSSIBLE\nCase #3: ESSSSENNNNESSSSEEENNNNESSSSSWWWWWWWNNNNN\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HamiltonianTour"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use std::collections::VecDeque;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let r = input.read_usize();
    let c = input.read_usize();
    let mut b = input.read_table::<char>(r, c);

    let mut next = Arr2d::new(2 * r, 2 * c, None);
    next[(0, 0)] = Some((1, 0));
    next[(1, 0)] = Some((1, 1));
    next[(1, 1)] = Some((0, 1));
    next[(0, 1)] = Some((0, 0));
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    b[(0, 0)] = '#';
    while let Some((cr, cc)) = queue.pop_front() {
        for (nr, nc) in D4::iter(cr, cc, r, c) {
            if b[(nr, nc)] == '*' {
                next[(2 * nr, 2 * nc)] = Some((2 * nr + 1, 2 * nc));
                next[(2 * nr + 1, 2 * nc)] = Some((2 * nr + 1, 2 * nc + 1));
                next[(2 * nr + 1, 2 * nc + 1)] = Some((2 * nr, 2 * nc + 1));
                next[(2 * nr, 2 * nc + 1)] = Some((2 * nr, 2 * nc));
                if nr == cr + 1 {
                    next[(2 * nr - 1, 2 * nc)] = Some((2 * nr, 2 * nc));
                    next[(2 * nr, 2 * nc + 1)] = Some((2 * nr - 1, 2 * nc + 1));
                } else if nr + 1 == cr {
                    next[(2 * nr + 2, 2 * nc + 1)] = Some((2 * nr + 1, 2 * nc + 1));
                    next[(2 * nr + 1, 2 * nc)] = Some((2 * nr + 2, 2 * nc));
                } else if nc == cc + 1 {
                    next[(2 * nr + 1, 2 * nc - 1)] = Some((2 * nr + 1, 2 * nc));
                    next[(2 * nr, 2 * nc)] = Some((2 * nr, 2 * nc - 1));
                } else {
                    next[(2 * nr, 2 * nc + 2)] = Some((2 * nr, 2 * nc + 1));
                    next[(2 * nr + 1, 2 * nc + 1)] = Some((2 * nr + 1, 2 * nc + 2));
                }
                b[(nr, nc)] = '#';
                queue.push_back((nr, nc));
            }
        }
    }
    let mut cr = 0;
    let mut cc = 0;
    let mut ans = "".to_string();
    loop {
        let (nr, nc) = next[(cr, cc)].unwrap();
        if nc == cc + 1 {
            ans.push('E');
        } else if nc + 1 == cc {
            ans.push('W');
        } else if nr == cr + 1 {
            ans.push('S');
        } else {
            ans.push('N');
        }
        cr = nr;
        cc = nc;
        if cr == 0 && cc == 0 {
            break;
        }
    }
    if b.into_iter().find('*').is_some() {
        ans = "IMPOSSIBLE".to_string();
    }
    out_line!(format!("Case #{}:", test_case), ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
