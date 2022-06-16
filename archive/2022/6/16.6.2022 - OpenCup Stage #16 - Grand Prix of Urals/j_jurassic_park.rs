//{"name":"J. Jurassic Park","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/J/","interactive":false,"timeLimit":1000,"tests":[{"input":"12\ntyrannosaurus\nmicropachycephalosaurus\nvelociraptor\ntriceratops\ndiplodocus\ngraciliceratops\nstenopelix\npiatnitzkysaurus\niguanodon\nbrachiosaurus\ncaudipteryx\nharpymimus\n","output":"4 6 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JJurassicPark"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let names = input.read_vec::<String>(n);

    let mut ans = vec![0i32; 3];
    for name in names {
        if name.ends_with("saurus") {
            ans[0] += 1;
        } else if name.ends_with("raptor")
            || name.ends_with("ceratops")
            || name.ends_with("odon")
            || name.ends_with("pteryx")
            || name.ends_with("mimus")
        {
            ans[1] += 1;
        } else {
            ans[2] += 1;
        }
    }
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
