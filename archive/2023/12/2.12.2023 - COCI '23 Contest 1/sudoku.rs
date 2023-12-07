//{"name":"#1 - Sudoku","group":"DMOJ - COCI '23 Contest 1","url":"https://dmoj.ca/problem/coci23c1p1","interactive":false,"timeLimit":1000,"tests":[{"input":"+---+---+---+\n|52.|...|.81|\n|.39|58.|...|\n|.8.|.9.|...|\n+---+---+---+\n|24.|...|1.3|\n|1..|43.|86.|\n|.63|..7|.24|\n+---+---+---+\n|...|1.9|35.|\n|..8|.74|6..|\n|31.|86.|7.9|\n+---+---+---+\n","output":"OK\n"},{"input":"+---+---+---+\n|3..|6..|..4|\n|4.9|8.1|..7|\n|..7|.49|6..|\n+---+---+---+\n|946|157|8.2|\n|.2.|3..|745|\n|.7.|28.|...|\n+---+---+---+\n|...|4..|..5|\n|8.5|.6.|.2.|\n|734|..8|5..|\n+---+---+---+\n","output":"GRESKA\n"},{"input":"+---+---+---+\n|5..|98.|67.|\n|6..|...|.31|\n|.2.|613|.4.|\n+---+---+---+\n|.96|8.2|1.7|\n|.28|..5|.9.|\n|7.3|19.|6..|\n+---+---+---+\n|962|.7.|.1.|\n|1.5|...|76.|\n|.7.|5..|9..|\n+---+---+---+\n","output":"GRESKA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Sudoku"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let sudoku = input.read_char_table(13, 13);

    for i in 0..13 {
        let mut mask = 0;
        for &c in sudoku.row(i) {
            if c.is_ascii_digit() {
                let id = c as usize - '1' as usize;
                if mask.is_set(id) {
                    out.print_line("GRESKA");
                    return;
                }
                mask.set_bit(id);
            }
        }
    }
    for i in 0..13 {
        let mut mask = 0;
        for &c in sudoku.column(i) {
            if c.is_ascii_digit() {
                let id = c as usize - '1' as usize;
                if mask.is_set(id) {
                    out.print_line("GRESKA");
                    return;
                }
                mask.set_bit(id);
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            let mut mask = 0;
            for k in i * 4..(i + 1) * 4 {
                for l in j * 4..(j + 1) * 4 {
                    let c = sudoku[(k, l)];
                    if c.is_ascii_digit() {
                        let id = c as usize - '1' as usize;
                        if mask.is_set(id) {
                            out.print_line("GRESKA");
                            return;
                        }
                        mask.set_bit(id);
                    }
                }
            }
        }
    }
    out.print_line("OK");
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
