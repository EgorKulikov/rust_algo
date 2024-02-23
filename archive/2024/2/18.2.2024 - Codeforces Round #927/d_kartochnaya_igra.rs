//{"name":"D. Карточная игра","group":"Codeforces - Codeforces Round 927 (Div. 3)","url":"https://codeforces.com/contest/1932/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3\nS\n3C 9S 4C 6D 3S 7S\n2\nC\n3S 5D 9S 6H\n1\nH\n6C 5D\n1\nS\n7S 3S\n1\nH\n9S 9H\n1\nS\n9S 9H\n1\nC\n9D 8H\n2\nC\n9C 9S 6H 8C\n","output":"3C 4C\n6D 9S\n3S 7S\nIMPOSSIBLE\nIMPOSSIBLE\n3S 7S\n9S 9H\n9H 9S\nIMPOSSIBLE\n6H 9C\n9S 8C\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DKartochnayaIgra"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let trump = input.read_char() as u8;
    let c = input.read_str_vec(2 * n);

    let mut by_suit = DefaultHashMap::<_, Vec<_>>::new();
    for c in c {
        by_suit[c[1]].push(c);
    }
    let mut ans = Vec::with_capacity(n);
    let mut leftover = Vec::new();
    for (&k, v) in by_suit.iter_mut() {
        v.sort();
        if k != trump {
            if v.len() % 2 == 1 {
                leftover.push(v[0].clone());
            }
            for i in v.indices().skip(v.len() % 2).step_by(2) {
                ans.push((v[i].clone(), v[i + 1].clone()));
            }
        }
    }
    if leftover.len() > by_suit[trump].len() {
        out.print_line("IMPOSSIBLE");
        return;
    }
    for i in by_suit[trump].indices().skip(leftover.len()).step_by(2) {
        ans.push((by_suit[trump][i].clone(), by_suit[trump][i + 1].clone()));
    }
    for (s1, s2) in leftover.into_iter().zip(by_suit[trump].iter().cloned()) {
        ans.push((s1, s2));
    }
    out.print_per_line(&ans);
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
    //    tester::stress_test();
}
//END MAIN
