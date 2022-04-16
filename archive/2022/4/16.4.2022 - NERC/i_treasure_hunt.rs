//{"name":"i_treasure_hunt","group":"Manual","url":"","interactive":true,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i_treasure_hunt"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, Writable};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.read_int();
    let m = input.read_int();

    enum QueryType {
        Scan,
        Dig,
    }

    impl Writable for QueryType {
        fn write(&self, output: &mut Output) {
            match self {
                QueryType::Scan => {
                    "SCAN".write(output);
                }
                QueryType::Dig => {
                    "DIG".write(output);
                }
            }
        }
    }

    let mut query = |query_type: QueryType, x: i32, y: i32| -> i32 {
        out_line!(query_type, x + 1, y + 1);
        input.read_int()
    };

    let from00 = query(QueryType::Scan, 0, 0);
    let from0m = query(QueryType::Scan, 0, m - 1);
    let sum_x = (from00 + from0m) / 2 - (m - 1);
    let sum_y = m - 1 + (from00 - from0m) / 2;
    let from_mid_x = query(QueryType::Scan, sum_x / 2, 0);
    let from_mid_y = query(QueryType::Scan, 0, sum_y / 2);
    let x1 = (sum_x + from_mid_x - sum_y) / 2;
    let x2 = sum_x - x1;
    let y1 = (sum_y + from_mid_y - sum_x) / 2;
    let y2 = sum_y - y1;

    if query(QueryType::Dig, x1, y1) == 1 {
        assert_eq!(query(QueryType::Dig, x2, y2), 1);
    } else {
        assert_eq!(query(QueryType::Dig, x2, y1), 1);
        assert_eq!(query(QueryType::Dig, x1, y2), 1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
