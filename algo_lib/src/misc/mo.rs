pub trait MoWorker<T, R> {
    fn empty() -> Self;
    fn add_left(&mut self, val: &T);
    fn add_right(&mut self, val: &T);
    fn remove_left(&mut self, val: &T);
    fn result(&self) -> R;
}

// Queries are 0-based (l, r), both inclusive. Sorted by (block of l, r), so r
// only ever grows within a block and the worker is rebuilt from empty on each
// block change - no remove_right needed. O((n + q) * n / block) = O(n sqrt(q)).
pub fn mo<T, R, W: MoWorker<T, R>>(arr: &[T], queries: &[(usize, usize)]) -> Vec<R> {
    if queries.is_empty() {
        return Vec::new();
    }
    let n = arr.len();
    let block = (((n as f64) / (queries.len() as f64).sqrt()).ceil() as usize).max(1);
    let mut order: Vec<usize> = (0..queries.len()).collect();
    order.sort_by_key(|&i| (queries[i].0 / block, queries[i].1));

    let mut res: Vec<Option<R>> = (0..queries.len()).map(|_| None).collect();
    let mut worker = W::empty();
    let mut cur_block = usize::MAX;
    // Current range is [cur_left, cur_right), half-open.
    let mut cur_left = 0;
    let mut cur_right = 0;
    for i in order {
        let (from, to) = queries[i];
        assert!(from <= to && to < n);
        if from / block != cur_block {
            cur_block = from / block;
            worker = W::empty();
            cur_left = from;
            cur_right = from;
        }
        while cur_right <= to {
            worker.add_right(&arr[cur_right]);
            cur_right += 1;
        }
        while cur_left > from {
            cur_left -= 1;
            worker.add_left(&arr[cur_left]);
        }
        while cur_left < from {
            worker.remove_left(&arr[cur_left]);
            cur_left += 1;
        }
        res[i] = Some(worker.result());
    }
    res.into_iter().map(Option::unwrap).collect()
}

#[cfg(test)]
mod test {
    use super::{mo, MoWorker};
    use crate::misc::random::{Random, RandomTrait};

    struct Distinct {
        count: [u32; 16],
        distinct: u32,
        sum: u64,
    }

    impl Distinct {
        fn add(&mut self, val: &u32) {
            self.count[*val as usize] += 1;
            if self.count[*val as usize] == 1 {
                self.distinct += 1;
            }
            self.sum += *val as u64;
        }
    }

    impl MoWorker<u32, (u32, u64)> for Distinct {
        fn empty() -> Self {
            Self {
                count: [0; 16],
                distinct: 0,
                sum: 0,
            }
        }

        fn add_left(&mut self, val: &u32) {
            self.add(val);
        }

        fn add_right(&mut self, val: &u32) {
            self.add(val);
        }

        fn remove_left(&mut self, val: &u32) {
            self.count[*val as usize] -= 1;
            if self.count[*val as usize] == 0 {
                self.distinct -= 1;
            }
            self.sum -= *val as u64;
        }

        fn result(&self) -> (u32, u64) {
            (self.distinct, self.sum)
        }
    }

    #[test]
    fn matches_brute_force() {
        let mut rng = Random::new_with_seed(42);
        for _ in 0..100 {
            let n = 1 + rng.gen_bound(50usize);
            let arr: Vec<u32> = (0..n).map(|_| rng.gen_bound(16u32)).collect();
            let q = 1 + rng.gen_bound(50usize);
            let queries: Vec<(usize, usize)> = (0..q)
                .map(|_| {
                    let from = rng.gen_bound(n);
                    let to = from + rng.gen_bound(n - from);
                    (from, to)
                })
                .collect();
            let answers = mo::<_, _, Distinct>(&arr, &queries);
            for (&(from, to), answer) in queries.iter().zip(answers.iter()) {
                let mut expected = Distinct::empty();
                for val in &arr[from..=to] {
                    expected.add(val);
                }
                assert_eq!(*answer, expected.result(), "range [{from}, {to}]");
            }
        }
    }

    #[test]
    fn no_queries() {
        let answers: Vec<(u32, u64)> = mo::<_, _, Distinct>(&[1u32, 2, 3], &[]);
        assert!(answers.is_empty());
    }

    #[test]
    fn fixed_answers() {
        let arr = [1u32, 2, 1, 3];
        let queries = [(0, 3), (1, 2), (2, 2), (0, 0), (1, 3), (0, 3)];
        let answers = mo::<_, _, Distinct>(&arr, &queries);
        assert_eq!(
            answers,
            vec![(3, 7), (2, 3), (1, 1), (1, 1), (3, 6), (3, 7)]
        );
    }

    // The window itself as the result: verifies that add_left/add_right feed
    // elements from the correct sides in the correct index order.
    struct Window {
        window: std::collections::VecDeque<u32>,
    }

    impl MoWorker<u32, Vec<u32>> for Window {
        fn empty() -> Self {
            Self {
                window: std::collections::VecDeque::new(),
            }
        }

        fn add_left(&mut self, val: &u32) {
            self.window.push_front(*val);
        }

        fn add_right(&mut self, val: &u32) {
            self.window.push_back(*val);
        }

        fn remove_left(&mut self, val: &u32) {
            assert_eq!(self.window.pop_front(), Some(*val));
        }

        fn result(&self) -> Vec<u32> {
            self.window.iter().copied().collect()
        }
    }

    #[test]
    fn window_contents_all_ranges() {
        let n = 12;
        let arr: Vec<u32> = (0..n as u32).collect();
        let mut queries = Vec::new();
        for from in 0..n {
            for to in from..n {
                queries.push((from, to));
            }
        }
        let answers = mo::<_, _, Window>(&arr, &queries);
        for (&(from, to), answer) in queries.iter().zip(answers.iter()) {
            assert_eq!(answer, &arr[from..=to], "range [{from}, {to}]");
        }
    }

    #[test]
    fn single_element() {
        let answers = mo::<_, _, Window>(&[7u32], &[(0, 0), (0, 0)]);
        assert_eq!(answers, vec![vec![7], vec![7]]);
    }
}
