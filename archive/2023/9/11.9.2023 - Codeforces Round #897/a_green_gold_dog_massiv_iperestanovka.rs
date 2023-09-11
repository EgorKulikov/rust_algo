//{"name":"A. green_gold_dog, массив и перестановка","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n100000\n2\n1 1\n3\n10 3 3\n","output":"1\n2 1\n1 3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGreenGoldDogMassivIPerestanovka"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::permutation::Permutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut ans = (0..n).collect_vec();
    ans.sort_by_key(|&i| a[i]);
    let mut ans = Permutation::new(ans).inv().to_vec();
    for i in &mut ans {
        *i = n - *i;
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
