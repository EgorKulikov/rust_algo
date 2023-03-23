//{"name":"Array Yaar","group":"CodeChef - CDVN2023","url":"https://www.codechef.com/CDVN2023/problems/ARRYRR?tab=statement","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n3\n","output":"11\n2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ArrayYaar"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();

    let a = (2..=n + 1).collect_vec();
    type Mod = ModInt7;
    let n = Mod::from_index(n);
    let ans = (n + Mod::one()) * (n + Mod::new(2)) / Mod::new(2) + Mod::one();
    out_line!(ans);
    out_line!(a);
}

#[test]
fn test() {
    for n in 3..=100 {
        let a = (2i128..=n + 1).collect_vec();
        let sum_cubes = a.iter().fold(0i128, |acc, &x| acc + x * x * x);
        assert!(sum_cubes % 2 == 1 || sum_cubes % 4 == 0);
        // println!("sum_cubes = {}", sum_cubes);
    }
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
