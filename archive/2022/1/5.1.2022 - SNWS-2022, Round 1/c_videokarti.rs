//{"name":"C. Видеокарты","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n18 73 98\n3 2 1\n","output":"190\n"},{"input":"4\n10 8 36 6\n2 4 3 1\n","output":"120\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVideokarti"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let p = input.read_vec::<u64>(n);
    let order = input.read_usize_vec(n).dec_by_one();

    fn convert(n: u64) -> u64 {
        let rem = n % 10;
        if rem > 0 {
            n + 10 - rem
        } else {
            n
        }
    }
    let s = p.as_slice().partial_sums();
    if convert(s[n]) == 0 {
        out_line!(0);
        return;
    }
    let mut sum = convert(s[n]);
    let mut ans = sum;
    let mut ends = BTreeSet::new();
    let mut qty = 1;
    ends.insert(0);
    ends.insert(n);
    for i in order {
        let to_left = *ends.range(..=i).last().unwrap();
        let to_right = *ends.range(i + 1..).next().unwrap();
        let was = convert(s[to_right] - s[to_left]);
        sum -= was;
        qty -= 1;
        if to_left != i {
            let left = convert(s[i] - s[to_left]);
            ends.insert(i);
            sum += left;
            qty += 1;
        }
        if i + 1 != to_right {
            let right = convert(s[to_right] - s[i + 1]);
            ends.insert(i + 1);
            sum += right;
            qty += 1;
        }
        ans.maxim(sum * qty);
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
