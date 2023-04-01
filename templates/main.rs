//$JSON

use algo_lib::io::input::Input;
use algo_lib::io::output::output;

$SOLVE

//START MAIN
#[test]
fn stress_test() {
    stress_test::stress_test(run);
}

mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
