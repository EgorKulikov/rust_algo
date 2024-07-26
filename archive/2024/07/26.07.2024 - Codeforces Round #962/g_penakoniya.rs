//{"name":"G. Пенакония","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n8 3\n1 8\n2 7\n4 5\n13 4\n1 13\n2 12\n3 11\n4 10\n10 2\n2 3\n3 4\n10 4\n3 8\n5 10\n2 10\n4 10\n4 1\n1 3\n5 2\n3 5\n1 4\n5 2\n2 5\n1 3\n","output":"4\n7\n2\n7\n2\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPenakoniya"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::random;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let mut id = Vec::with_capacity(m);
    for _ in 0..m {
        id.push(random().gen());
    }
    let mut change = vec![0; n];
    for (i, (u, v)) in edges.into_iter().enumerate() {
        change[u] ^= id[i];
        change[v] ^= id[i];
    }
    let mut qty = DefaultHashMap::<_, usize>::new();
    let mut cur = 0;
    for i in change {
        cur ^= i;
        qty[cur] += 1;
    }
    out.print_line(n - qty.values().max().unwrap());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
