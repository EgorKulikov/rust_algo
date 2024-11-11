//{"name":"P11228 [CSP-J 2024] 地图探险（民间数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11228?contestId=209924","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1 5 4\n1 1 2\n....x\n5 5 20\n1 1 0\n.....\n.xxx.\n.x.x.\n..xx.\nx....\n","output":"3\n13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11228CSPJ2024"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let mut x = input.read_size() - 1;
    let mut y = input.read_size() - 1;
    let mut d = input.read_size();
    let map = input.read_char_table(n, m);

    let mut ans = FxHashSet::default();
    let mut visited = FxHashSet::default();
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    for _ in 0..k {
        if visited.contains(&(x, y, d)) {
            break;
        }
        visited.insert((x, y, d));
        ans.insert((x, y));
        let nx = x as isize + dx[d];
        let ny = y as isize + dy[d];
        if nx >= 0
            && nx < n as isize
            && ny >= 0
            && ny < m as isize
            && map[(nx as usize, ny as usize)] == b'.'
        {
            x = nx as usize;
            y = ny as usize;
        } else {
            d = (d + 1) % 4;
        }
    }
    ans.insert((x, y));
    out.print_line(ans.len());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
