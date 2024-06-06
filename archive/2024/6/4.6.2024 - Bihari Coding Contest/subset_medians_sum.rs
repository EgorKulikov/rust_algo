//{"name":"Subset Medians Sum","group":"HackerRank - Bihari Coding Contest","url":"https://www.hackerrank.com/contests/bihari-coding-contest/challenges/subset-medians-sum","interactive":false,"timeLimit":4000,"tests":[{"input":"6\n3 5 7 2 4 6\n","output":"144\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SubsetMediansSum"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n).sorted();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n);
    let mut ans = Mod::zero();
    for i in 0..n {
        ans += Mod::new(a[i]) * c.c(n - 1, i);
    }
    out.print_line(ans);
    // let mut b = Vec::with_capacity((n + 1) / 2);
    // for i in 0..=(n - 1) / 2 {
    //     b.push(c.inv_fact(i).square());
    // }
    // let mut d = Vec::with_capacity((n + 1) / 2);
}

#[test]
fn test() {
    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(21);
    for n in 1..=20 {
        let mut cur = Vec::with_capacity(n);
        for j in 0..n {
            let max = j.min(n - 1 - j);
            let mut cc = Mod::zero();
            for i in 0..=max {
                cc += c.c(j, i) * c.c(n - 1 - j, i);
            }
            cur.push(cc);
        }
        println!("n = {n} {:?}", cur);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
