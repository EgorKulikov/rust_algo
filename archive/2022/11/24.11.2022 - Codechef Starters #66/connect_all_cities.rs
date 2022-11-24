//{"name":"Connect All Cities","group":"CodeChef - START66A","url":"https://www.codechef.com/START66A/problems-old/CNCTCT","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n4 4 4 4\n3\n2 4 6\n","output":"12\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ConnectAllCities"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_unsigned_vec(n);

    let mut ans = 0;
    let mut dsu = DSU::new(n);
    for i in 0..30 {
        let mut leader = None;
        for j in 0..n {
            if a[j].is_set(i) {
                if let Some(leader) = leader {
                    if dsu.join(leader, j) {
                        ans += 1u64 << i;
                    }
                } else {
                    leader = Some(j);
                }
            }
        }
    }
    if dsu.count() > 1 {
        out_line!(-1);
    } else {
        out_line!(ans);
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
