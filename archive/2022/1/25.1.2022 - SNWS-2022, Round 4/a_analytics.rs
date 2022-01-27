//{"name":"A. Analytics","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/A/","interactive":false,"timeLimit":2000,"tests":[{"input":"6 7 8\n1 2\n3 5\n2 4\n2 4\n2 6\n1 5\n2 1\n1 1 7 1\n1 1 7 2\n1 1 7 3\n1 1 7 4\n1 1 7 5\n1 1 7 6\n2 4 5 2\n1 1 1 1\n","output":"6\n3\n5\n4\n2\n1\n4\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAnalytics"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let q = input.read_usize();
    let changes = input.read_usize_pair_vec(m).dec_by_one();

    const BUBEN: usize = 350;
    struct Part<'s> {
        from: usize,
        to: usize,
        changes: &'s [(usize, usize)],
        p: Permutation,
        q: Permutation,
    }

    impl<'s> Part<'s> {
        fn new(from: usize, to: usize, n: usize, changes: &'s [(usize, usize)]) -> Self {
            let changes = &changes[from..to];
            let mut p = Permutation::new_ident(n);
            for &(a, b) in changes {
                p.swap(a, b);
            }
            let q = p.inv();
            Self {
                from,
                to,
                changes,
                p,
                q,
            }
        }

        fn direct(&self, start: usize, end: usize, mut cur: usize) -> usize {
            if end <= self.from || start >= self.to {
                cur
            } else if start <= self.from && self.to <= end {
                self.q[cur]
            } else {
                for i in start.max(self.from)..end.min(self.to) {
                    let (a, b) = self.changes[i - self.from];
                    if a == cur {
                        cur = b;
                    } else if b == cur {
                        cur = a;
                    }
                }
                cur
            }
        }

        fn reverse(&self, start: usize, end: usize, mut cur: usize) -> usize {
            if end <= self.from || start >= self.to {
                cur
            } else if start <= self.from && self.to <= end {
                self.p[cur]
            } else {
                for i in (start.max(self.from)..end.min(self.to)).rev() {
                    let (a, b) = self.changes[i - self.from];
                    if a == cur {
                        cur = b;
                    } else if b == cur {
                        cur = a;
                    }
                }
                cur
            }
        }
    }

    let mut parts = Vec::new();
    for i in (0..m).step_by(BUBEN) {
        parts.push(Part::new(i, (i + BUBEN).min(m), n, &changes));
    }

    for _ in 0..q {
        let tp = input.read_usize();
        let f = input.read_usize() - 1;
        let t = input.read_usize();
        let mut x = input.read_usize() - 1;

        if tp == 2 {
            for i in (f / BUBEN)..=((t - 1) / BUBEN) {
                x = parts[i].direct(f, t, x);
            }
        } else {
            for i in ((f / BUBEN)..=((t - 1) / BUBEN)).rev() {
                x = parts[i].reverse(f, t, x);
            }
        }
        out_line!(x + 1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
