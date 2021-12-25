//{"name":"Ex - Manhattan Christmas Tree","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_h","interactive":false,"timeLimit":7000,"tests":[{"input":"4\n3 3\n4 6\n7 4\n2 5\n6\n3 5 1\n3 5 2\n3 5 3\n3 5 4\n100 200 3\n300 200 1\n","output":"1\n2\n2\n5\n293\n489\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ExManhattanChristmasTree"}}}

use algo_lib::collections::persistent_fenwick::PersistentFenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read();
    let mut trees: Vec<(i32, i32)> = input.read_vec(n);

    trees.iter_mut().for_each(|(x, y)| {
        let nx = *x + *y + 1;
        let ny = *x - *y;
        *x = nx;
        *y = ny + 100000;
    });
    trees.sort();
    let mut ft = PersistentFenwickTree::new(200001, 0);
    for (x, y) in trees {
        ft.add(y as usize, 1, x);
    }

    let q = input.read();
    for _ in 0..q {
        let a: i32 = input.read();
        let b: i32 = input.read();
        let k: i32 = input.read();

        let x = a + b + 1;
        let y = a - b + 100000;

        let mut left = 0;
        let mut right = 200000;
        while left < right {
            let mid = (left + right) / 2;

            let from_y = (y - mid).max(0) as usize;
            let to_y = (y + mid + 1) as usize;
            let from_x = (x - mid - 1).max(0);
            let to_x = x + mid;
            let val = ft.get(from_y, to_y, to_x) - ft.get(from_y, to_y, from_x);
            if val >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        out_line!(left);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
