//{"name":"T539826 202411G 三角含数","group":"Luogu","url":"https://www.luogu.com.cn/problem/T539826?contestId=213071","interactive":false,"timeLimit":1000,"tests":[{"input":"223456 223456\n","output":"1\n"},{"input":"123456 123456\n","output":"0\n"},{"input":"123456 223456\n","output":"23029\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T539826202411G"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_size();
    let r = input.read_size();

    let mut ans = 0;
    let mut d = [0; 6];
    let mut d1 = [0; 3];
    let mut d2 = [0; 3];
    for mut i in l..=r {
        for j in 0..6 {
            d[j] = i % 10;
            i /= 10;
        }
        for j in 0i16..64 {
            if j.count_ones() == 3 {
                let mut a = 0;
                let mut b = 0;
                for k in 0..6 {
                    if j.is_set(k) {
                        d1[a] = d[k];
                        a += 1;
                    } else {
                        d2[b] = d[k];
                        b += 1;
                    }
                }
                let sd1 = d1.iter().sum::<usize>();
                let sd2 = d2.iter().sum::<usize>();
                if *d1.iter().max().unwrap() * 2 < sd1 && *d2.iter().max().unwrap() * 2 < sd2 {
                    ans += 1;
                    break;
                }
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
