//{"name":"Mode of an array","group":"HackerEarth - January Easy '22","url":"https://www.hackerearth.com/challenges/competitive/january-easy-22/algorithm/making-more-e942ff93/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 1\n1 1 3\n4 1\n1 1 3 3\n","output":"0\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ModeOfAnArray"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_unsigned();
    let a = input.read_unsigned_vec(n);

    let mut map = DefaultMap::new();
    for i in a {
        map[i] += 1;
    }
    let mut have = map.remove(&k).unwrap_or(0);
    let mut heap = BinaryHeap::new();
    for &v in map.values() {
        heap.push(v);
    }
    let mut ans = 0;
    while let Some(m) = heap.pop() {
        if m < have {
            break;
        }
        have += 1;
        ans += 1;
        heap.push(m - 1);
    }
    out_line!(ans);
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
