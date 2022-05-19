//{"name":"F. BpbBppbpBB","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/F/","interactive":false,"timeLimit":1000,"tests":[{"input":"10 17\n#################\n#################\n#################\n####..#####..####\n###....###....###\n###....###....###\n####..#####..####\n#################\n#################\n#################\n","output":"1 0\n"},{"input":"14 11\n.##########\n.##########\n.##########\n.####..####\n.###....###\n.###....###\n.####..####\n.##########\n.##########\n.##########\n.###.......\n.###.......\n.###.......\n.###.......\n","output":"0 1\n"},{"input":"20 14\n.##########...\n.##########...\n.##########...\n.####..####...\n.###....###...\n.###....###...\n.####..####...\n.##########...\n.##########...\n.##########...\n.#############\n.#############\n.#############\n.#######..####\n....###....###\n....###....###\n....####..####\n##############\n##############\n##############\n","output":"0 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBpbBppbpBB"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let t = input.read_table::<char>(n, m);

    if n < 10 || m < 10 {
        out_line!(0, 0);
        return;
    }
    let mut pos = HashSet::new();
    for i in 0..n - 9 {
        for j in 0..m - 9 {
            let mut good = true;
            for k in 0..10 {
                for l in 0..10 {
                    let target = if k >= 3
                        && k < 7
                        && l >= 3
                        && l < 7
                        && (k != 3 && k != 6 || l != 3 && l != 6)
                    {
                        '.'
                    } else {
                        '#'
                    };
                    if t[(i + k, j + l)] != target {
                        good = false;
                        break;
                    }
                }
                if !good {
                    break;
                }
            }
            if good {
                pos.insert((i, j));
            }
        }
    }
    let mut c = 0;
    let mut s = 0;
    while let Some(&(i, j)) = pos.iter().next() {
        pos.remove(&(i, j));
        if i >= 7 && pos.contains(&(i - 7, j)) {
            pos.remove(&(i - 7, j));
            c += 1;
            continue;
        }
        if i + 7 < n && pos.contains(&(i + 7, j)) {
            pos.remove(&(i + 7, j));
            c += 1;
            continue;
        }
        if j >= 7 && pos.contains(&(i, j - 7)) {
            pos.remove(&(i, j - 7));
            c += 1;
            continue;
        }
        if j + 7 < m && pos.contains(&(i, j + 7)) {
            pos.remove(&(i, j + 7));
            c += 1;
            continue;
        }
        s += 1;
    }
    out_line!(c, s);
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
