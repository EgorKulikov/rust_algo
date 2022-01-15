//{"name":"F. Еда по воздуху","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5 1\n0..2.\n.XXX.\n.....\n.....\n.X.X1\n","output":"9\n"},{"input":"2 2 2\n03\n12\n","output":"4\n"},{"input":"1 5 1\n.0X21\n","output":"8\n"},{"input":"3 4 1\nX0XX\nX.X2\n1XXX\n","output":"-1\n"},{"input":"3 9 5\n...1...2.\n..3..4..5\nXX06XXXXX\n","output":"21\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FYedaPoVozdukhu"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::arr5d::Arr5d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let r = input.read_usize();
    let c = input.read_usize();
    let k = input.read_usize();
    let grid = input.read_table::<char>(r, c);

    let mut ans: Arr5d<i16> = Arr5d::new(k + 1, r, c, 2 * r + 1, 2 * c + 1, -1);
    let mut start = None;
    let mut finish = None;
    for i in 0..r {
        for j in 0..c {
            if grid[(i, j)] == '0' {
                start = Some((i, j));
            }
            if grid[(i, j)] == (b'0'.into_usize() + k + 1).into_u8() as char {
                finish = Some((i, j));
            }
        }
    }
    let start = (0, start.unwrap().0, start.unwrap().1, r, c);
    let finish = (k, finish.unwrap().0, finish.unwrap().1, r, c);
    ans[start] = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(next) = q.pop_front() {
        if next == finish {
            out_line!(ans[next]);
            return;
        }
        assert_ne!(ans[next], -1);
        let res = ans[next];
        let (step, cr, cc, dr, dc) = next;
        for i in -1..=1 {
            for j in -1..=1 {
                let delta_r = dr.into_isize() - r.into_isize() + i;
                let delta_c = dc.into_isize() - c.into_isize() + j;
                if delta_r < 0 && cr.into_isize() < -delta_r {
                    continue;
                }
                if delta_c < 0 && cc.into_isize() < -delta_c {
                    continue;
                }
                if delta_r > 0 && cr + delta_r.into_usize() >= r {
                    continue;
                }
                if delta_c > 0 && cc + delta_c.into_usize() >= c {
                    continue;
                }
                let nr = (cr.into_isize() + delta_r).into_usize();
                let nc = (cc.into_isize() + delta_c).into_usize();
                if grid[(nr, nc)] == 'X' {
                    continue;
                }
                let ndr = (delta_r + r.into_isize()).into_usize();
                let ndc = (delta_c + c.into_isize()).into_usize();
                let nstep = if step < k
                    && grid[(nr, nc)] == (b'0'.into_usize() + step + 1).into_u8() as char
                {
                    step + 1
                } else {
                    step
                };
                let key = (nstep, nr, nc, ndr, ndc);
                if ans[key] == -1 {
                    ans[key] = res + 1;
                    q.push_back(key);
                }
            }
        }
    }
    out_line!(-1);
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
