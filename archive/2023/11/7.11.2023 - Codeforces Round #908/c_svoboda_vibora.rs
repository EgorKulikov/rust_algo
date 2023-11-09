//{"name":"C. Свобода выбора","group":"Codeforces - Codeforces Round 908 (Div. 1)","url":"https://codeforces.com/contest/1893/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n3\n3 5 6\n10 11 12\n3 3 1\n1 1 3\n12\n4\n2 4 4\n12 13\n1 5\n1\n7 1000 1006\n1000 1001 1002 1003 1004 1005 1006\n147 145 143 143 143 143 142\n1\n2 48 50\n48 50\n25 25\n2\n1 1 1\n1\n1\n1 1 1\n2\n1\n1\n1 1 1\n1\n2\n2\n1 1 1\n1\n1\n2 1 1\n1 2\n1 1\n2\n4 8 10\n11 12 13 14\n3 3 3 3\n2 3 4\n11 12\n2 2\n","output":"1\n139\n0\n1\n1\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSvobodaVibora"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut l = Vec::with_capacity(n);
    let mut r = Vec::with_capacity(n);
    let mut sets = Vec::with_capacity(n);
    let mut sum_k = 0;
    let mut sum_l = 0;
    let mut sum_r = 0;
    for _ in 0..n {
        let k = input.read_size();
        l.push(input.read_size());
        r.push(input.read_size());
        let a = input.read_size_vec(k);
        let c = input.read_size_vec(k);
        sets.push(a.into_iter().zip(c).collect_vec());
        sum_k += k;
        sum_l += l.last().unwrap();
        sum_r += r.last().unwrap();
    }

    if sum_r - sum_l + 1 > sum_k {
        out.print_line(0);
        return;
    }
    let mut pos = DefaultHashMap::<_, Vec<_>>::new();
    let mut size = Vec::with_capacity(n);
    for (i, set) in sets.iter().enumerate() {
        let mut cur_size = 0;
        for (a, b) in set {
            cur_size += b;
            pos[*a].push((i, *b));
        }
        size.push(cur_size);
    }
    let mut ans = None;
    for i in sum_l..=sum_r {
        let mut has = sum_r;
        let mut min_take = 0;
        for &(x, b) in &pos[i] {
            let not_i = size[x] - b;
            if l[x] > not_i {
                min_take += l[x] - not_i;
            }
            has -= r[x];
            has += r[x].min(not_i);
        }
        let cur = min_take.max(i.max(has) - has);
        ans.minim(cur);
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
