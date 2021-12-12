//{"name":"D. Paimon Sorting","group":"Yandex - Stage 9: Grand Prix of Nanjing","url":"https://official.contest.yandex.ru/opencupXXII/contest/33444/problems/D/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n2 3 2 1 5\n3\n1 2 3\n1\n1\n","output":"0 2 3 5 7\n0 2 4\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPaimonSorting"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut a: Vec<usize> = input.read_vec(n);
    a = a.dec_by_one();

    let ans = do_solve(n, a);
    out_line!(ans);
}

fn do_solve(n: usize, a: Vec<usize>) -> Vec<u64> {
    let mut ft = FenwickTree::new(n);
    let mut ans = Vec::new();
    // let mut qty = vec![0usize; n];
    let mut c_max = a[0];
    let mut second_index = 0;
    for (i, a) in a.into_iter().enumerate() {
        if i == 0 {
            ans.push(0u64);
        } else if a > c_max {
            if second_index == 0 {
                ans.push(ans.last().unwrap() + 2);
            } else {
                ans.push(ans.last().unwrap() + 2 + ((i - second_index) as u64));
            }
            // ans.push(ans.last().unwrap() + 2);
        } else {
            ans.push(ans.last().unwrap() + (ft.get(a + 1, n) as u64));
        }
        if ft.get(a, a + 1) == 0 {
            ft.add(a, 1usize);
        }
        if a == c_max {
            if second_index == 0 {
                second_index = i;
            }
        } else if a > c_max {
            c_max = a;
            second_index = 0;
        }
        // qty[a] += 1;
    }
    ans
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
