//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    struct Tree {
        len: usize,
        children: Vec<Tree>,
    }

    impl Readable for Tree {
        fn read(input: &mut Input) -> Self {
            let len = input.read_size();
            let q = input.read_size();
            let mut children = Vec::with_capacity(q);
            for _ in 0..q {
                children.push(input.read());
            }
            Tree { len, children }
        }
    }

    let w = input.read_size();
    let tree: Tree = input.read();

    let mut ans = 1;
    let mut rec = RecursiveFunction::new(|f, tree: &Tree| {
        let mut ends = Vec::with_capacity(tree.children.len());
        for child in &tree.children {
            ends.push(f.call(child));
        }
        ends.sort();
        let mut res = 0;
        for i in ends {
            if res + i <= w {
                res += i;
            } else {
                ans += 1;
            }
        }
        if res + tree.len <= w {
            res += tree.len;
        } else {
            ans += 1;
            let rem = res + tree.len - w;
            let add = (rem - 1) / w;
            ans += add;
            res = rem - add * w;
        }
        res
    });
    rec.call(&tree);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
