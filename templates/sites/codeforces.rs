type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    $CARET
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]type PreCalc = ();

    fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
        $CARET
    }

    pub static TEST_TYPE: TestType = TestType::MultiNumber;
    pub static TASK_TYPE: TaskType = TaskType::$INTERACTIVE;

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

    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
