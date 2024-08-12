//{"name":"Binary Strings Yet Again","group":"CodeChef - START146A","url":"https://www.codechef.com/START146A/problems/BINSTRAGAIN","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 1\n13\n4 1\n3\n54 2\n12344321 123321\n","output":"2\n4\n629\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BinaryStringsYetAgain"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = Vec<usize>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, mem: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_unsigned_vec(m);

    let mut aa = Vec::with_capacity(a.len() * 2);
    for x in a {
        aa.push(x & u32::all_bits(15));
        aa.push(x >> 15);
    }
    let a = aa;

    if n == 1 {
        out.print_line(0);
        return;
    }

    let mut ans = 0;

    struct Iter<'a> {
        a: &'a [u32],
        n: usize,
        step: usize,
        at: usize,
        bit: usize,
        target: usize,
        mem: &'a [usize],
        ones: usize,
        zeroes: usize,
        add: usize,
    }

    impl Iter<'_> {
        fn single(&mut self, a: u32, len: usize) {
            let cur_ones = a.count_ones() as usize;
            let cur_zeroes = len - cur_ones;
            self.add += cur_ones * self.zeroes + self.mem[a as usize];
            self.ones += cur_ones;
            self.zeroes += cur_zeroes;
        }
    }

    impl Iterator for Iter<'_> {
        type Item = (usize, usize);

        fn next(&mut self) -> Option<Self::Item> {
            if self.target == self.n {
                return None;
            }
            self.target += self.step;
            self.ones = 0;
            self.zeroes = 0;
            self.add = 0;
            let mut to = (self.at + 1) * 15;
            while self.target >= to {
                self.single(self.a[self.at] >> self.bit, 15 - self.bit);
                self.bit = 0;
                self.at += 1;
                to += 15;
            }
            to -= 15;
            let last_len = self.target - to - self.bit;
            if last_len != 0 {
                self.single(
                    (self.a[self.at] >> self.bit) & u32::all_bits(last_len),
                    last_len,
                );
            }
            self.bit = self.target - to;
            Some((self.ones, self.add))
        }
    }

    let check = |len: usize| -> usize {
        let it = Iter {
            a: &a,
            n,
            step: len,
            at: 0,
            bit: 0,
            target: 0,
            mem,
            ones: 0,
            zeroes: 0,
            add: 0,
        };
        let mut res = 0;
        if len * len <= n {
            let mut qty = vec![0; len + 1];
            for (i, add) in it {
                qty[i] += 1;
                res += add;
            }
            let mut zeroes = 0;
            for i in 0..=len {
                for _ in 0..qty[i] {
                    res += zeroes * i;
                    zeroes += len - i;
                }
            }
        } else {
            let qty = it.collect_vec().sorted();
            let mut zeroes = 0;
            for (ones, add) in qty {
                res += zeroes * ones + add;
                zeroes += len - ones;
            }
        }
        res
    };
    let mut nn = n;
    for i in 2.. {
        if i * i > nn {
            break;
        }
        if nn % i == 0 {
            ans.maxim(check(i));
            while nn % i == 0 {
                nn /= i;
            }
        }
    }
    if nn > 1 {
        ans.maxim(check(nn));
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Vec::with_capacity(1 << 15);
    for i in 0..(1 << 15) {
        let mut zeroes = 0;
        let mut res = 0;
        for j in 0..15 {
            if i.is_set(j) {
                res += zeroes;
            } else {
                zeroes += 1;
            }
        }
        pre_calc.push(res);
    }

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
