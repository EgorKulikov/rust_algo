//{"name":"E1. Цифровая деревня (простая версия)","group":"Codeforces - Codeforces Round 977 (Div. 2, на основе COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n9 8 5\n2 5 6 8 9\n1 2 1\n1 3 2\n3 4 10\n4 5 3\n4 6 5\n1 7 10\n7 8 4\n7 9 2\n3 3 2\n3 1\n1 2 1\n2 3 3\n1 3 2\n","output":"34 19 9 4 0 0 0 0 0\n2 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1TsifrovayaDerevnyaProstayaVersiya"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_size();
    let s = input.read_size_vec(p).dec();
    let mut edges = input.read_vec::<(usize, usize, usize)>(m).dec();

    let mut special = vec![0; n];
    for i in s {
        special[i] = 1;
    }
    let mut dsu = DSU::new(n);
    let mut ans = vec![0; n];
    let mut cur = p - 1;
    let mut score = vec![0; n];
    edges.sort_by_key(|x| x.2);
    // let mut total_score = 0;
    let mut deltas = Vec::new();
    for (mut u, mut v, x) in edges {
        u = dsu.get(u);
        v = dsu.get(v);
        if u == v {
            continue;
        }
        if special[u] != 0 && special[v] != 0 {
            let left_score = x * special[u] + score[v];
            let right_score = x * special[v] + score[u];
            let new_score = left_score.min(right_score);
            let delta = new_score - score[u] - score[v];
            // total_score += delta;
            score[u] = new_score;
            score[v] = new_score;
            // cur -= 1;
            // ans[cur] = total_score;
            deltas.push(delta);
        } else {
            let sc = score[u] + score[v];
            score[u] = sc;
            score[v] = sc;
        }
        let sp = special[u] + special[v];
        special[u] = sp;
        special[v] = sp;
        dsu.join(u, v);
    }
    deltas.sort();
    for d in deltas {
        ans[cur - 1] = ans[cur] + d;
        cur -= 1;
    }
    out.print_line(ans);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
