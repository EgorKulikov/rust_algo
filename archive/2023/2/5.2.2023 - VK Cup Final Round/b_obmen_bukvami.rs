//{"name":"B. Обмен буквами","group":"Codeforces - VK Cup 2022 - Финальный раунд (Engine)","url":"https://codeforces.com/contest/1784/problem/B","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n2\nnwi\ninw\n3\ninn\nnww\nwii\n4\nwin\nwww\niii\nnnn\n","output":"0\n2\n2 w 3 i\n3 w 1 n\n3\n2 w 3 i\n2 w 4 n\n3 i 4 n\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BObmenBukvami"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut qty = Arr2d::new(n, 3, 0);
    for i in 0..n {
        let s: Str = input.read();
        for c in s {
            match c {
                b'w' => qty[i][0] += 1,
                b'i' => qty[i][1] += 1,
                b'n' => qty[i][2] += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut need = Arr2d::new(3, 3, Vec::new());
    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                if qty[(i, j)] > 1 && qty[(i, k)] < 1 {
                    qty[(i, j)] -= 1;
                    qty[(i, k)] += 1;
                    need[(j, k)].push(i);
                }
            }
        }
    }
    let mut ans = Vec::new();
    let letters = ['w', 'i', 'n'];
    for i in 0..3 {
        for j in 0..i {
            while !need[(i, j)].is_empty() && !need[(j, i)].is_empty() {
                ans.push((
                    need[(i, j)].pop().unwrap() + 1,
                    letters[i],
                    need[(j, i)].pop().unwrap() + 1,
                    letters[j],
                ));
            }
        }
    }
    while !need[(0, 1)].is_empty() {
        let i = need[(0, 1)].pop().unwrap();
        let j = need[(1, 2)].pop().unwrap();
        let k = need[(2, 0)].pop().unwrap();
        ans.push((i + 1, 'w', j + 1, 'i'));
        ans.push((j + 1, 'w', k + 1, 'n'));
    }
    while !need[(0, 2)].is_empty() {
        let i = need[(0, 2)].pop().unwrap();
        let j = need[(2, 1)].pop().unwrap();
        let k = need[(1, 0)].pop().unwrap();
        ans.push((i + 1, 'w', j + 1, 'n'));
        ans.push((j + 1, 'w', k + 1, 'i'));
    }
    out_line!(ans.len());
    output().print_per_line(&ans);
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
