//{"name":"COCI '21 Contest 2 #1 Kaučuk","group":"DMOJ","url":"https://dmoj.ca/problem/coci21c2p1","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nsection zivotinje\nsection boje\nsection voce\n","output":"1 zivotinje\n2 boje\n3 voce\n"},{"input":"4\nsection zivotinje\nsubsection macke\nsubsection psi\nsubsubsection mops\n","output":"1 zivotinje\n1.1 macke\n1.2 psi\n1.2.1 mops\n"},{"input":"4\nsection zivotinje\nsubsection psi\nsection voce\nsubsection ananas\n","output":"1 zivotinje\n1.1 psi\n2 voce\n2.1 ananas\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COCI21Contest21Kauuk"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();

    let mut section = 0;
    let mut sub_section = 0;
    let mut sub_sub_section = 0;

    for _ in 0..n {
        let t: String = input.read();
        let name: String = input.read();
        match t.as_str() {
            "section" => {
                section += 1;
                sub_section = 0;
                sub_sub_section = 0;
                out_line!(section, name);
            }
            "subsection" => {
                sub_section += 1;
                sub_sub_section = 0;
                out_line!(format!("{}.{} {}", section, sub_section, name));
            }
            "subsubsection" => {
                sub_sub_section += 1;
                out_line!(format!(
                    "{}.{}.{} {}",
                    section, sub_section, sub_sub_section, name
                ));
            }
            _ => panic!("unreachable"),
        }
    }
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
