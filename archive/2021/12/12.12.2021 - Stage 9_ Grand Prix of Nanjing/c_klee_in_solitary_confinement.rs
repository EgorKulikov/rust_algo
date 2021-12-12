//{"name":"C. Klee in Solitary Confinement","group":"Yandex - Stage 9: Grand Prix of Nanjing","url":"https://official.contest.yandex.ru/opencupXXII/contest/33444/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n2 2 4 4 4\n","output":"5\n"},{"input":"7 1\n3 2 3 2 2 2 3\n","output":"6\n"},{"input":"7 1\n2 3 2 3 2 3 3\n","output":"5\n"},{"input":"9 -100\n-1 -2 1 2 -1 -2 1 -2 1\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKleeInSolitaryConfinement"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::{HashMap, VecDeque};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k: i32 = input.read();
    let a: Vec<i32> = input.read_vec(n);

    let mut map: HashMap<i32, Vec<bool>> = HashMap::new();
    let mut add = |num, val| {
        if !map.contains_key(&num) {
            map.insert(num, Vec::new());
        }
        map.get_mut(&num).unwrap().push(val);
    };
    for i in a {
        add(i, false);
        if k != 0 {
            add(i + k, true);
        }
    }
    let mut ans = 0;
    for seq in map.into_values() {
        let mut qty = vec![0];
        for v in seq.iter() {
            qty.push(*qty.last().unwrap());
            if *v {
                *qty.last_mut().unwrap() += 1;
            }
        }
        let mut tail = VecDeque::new();
        let mut balance = 0;
        tail.push_back((seq.len(), 0));
        for (i, val) in seq.iter().enumerate().rev() {
            if *val {
                balance -= 1;
            } else {
                balance += 1;
            }
            if balance > tail.back().unwrap().1 {
                tail.push_back((i, balance));
            }
        }
        for i in 0..seq.len() {
            let mid = tail.back().unwrap().0;
            ans.maxim(
                i - (qty[i] - qty[0]) + qty[mid] - qty[i] + (seq.len() - mid)
                    - (qty[seq.len()] - qty[mid]),
            );
            if mid == i {
                tail.pop_back();
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
