//{"name":"E - Takahashi's Anguish","group":"AtCoder - Tokio Marine & Nichido Fire Insurance Programming Contest 2022（AtCoder Beginner Contest 256)","url":"https://atcoder.jp/contests/abc256/tasks/abc256_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 3 2\n1 10 100\n","output":"10\n"},{"input":"8\n7 3 5 5 8 4 1 2\n36 49 73 38 30 85 27 45\n","output":"57\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETakahashisAnguish"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let x = input.read_usize_vec(n).dec_by_one();
    let c = input.read_long_vec(n);

    let mut ans = 0;
    let mut mark = vec![0; n];
    for i in 0..n {
        if mark[i] != 0 {
            continue;
        }
        let mut cur = Vec::new();
        let mut j = i;
        while mark[j] == 0 {
            cur.push(j);
            mark[j] = 1;
            j = x[j];
        }
        if mark[j] != 2 {
            let mut to_add = None;
            while mark[j] == 1 {
                mark[j] = 2;
                to_add.minim(c[j]);
                j = x[j];
            }
            ans += to_add.unwrap();
        }
        for j in cur {
            mark[j] = 2;
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
