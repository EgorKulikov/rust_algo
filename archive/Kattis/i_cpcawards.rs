//{"name":"ICPC Awards","group":"Kattis","url":"https://open.kattis.com/problems/icpcawards","interactive":false,"timeLimit":1000,"tests":[{"input":"30\nSeoul ACGTeam\nVNU LINUX\nSJTU Mjolnir\nVNU WINDOWS\nNTU PECaveros\nHUST BKJuniors\nHCMUS HCMUSSerendipity\nVNU UBUNTU\nSJTU Metis\nHUST BKDeepMind\nHUST BKTornado\nHCMUS HCMUSLattis\nNUS Tourism\nVNU DOS\nHCMUS HCMUSTheCows\nVNU ANDROID\nHCMUS HCMUSPacman\nHCMUS HCMUSGeomecry\nUIndonesia DioramaBintang\nVNU SOLARIS\nUIndonesia UIChan\nFPT ACceptable\nHUST BKIT\nPTIT Miners\nPSA PSA\nDaNangUT BDTTNeverGiveUp\nVNU UNIXBSD\nCanTho CTUA2LTT\nSoongsil Team10deung\nSoongsil BezzerBeater\n","output":"Seoul ACGTeam\nVNU LINUX\nSJTU Mjolnir\nNTU PECaveros\nHUST BKJuniors\nHCMUS HCMUSSerendipity\nNUS Tourism\nUIndonesia DioramaBintang\nFPT ACceptable\nPTIT Miners\nPSA PSA\nDaNangUT BDTTNeverGiveUp\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ICPCAwards"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut set = FxHashSet::default();
    out.print_per_line_iter(
        input
            .iter::<(Str, Str)>()
            .take(n)
            .filter(|(a, _)| set.insert(a.clone()))
            .take(12),
    );
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}

#[cfg(test)]
mod tester;
//END MAIN
