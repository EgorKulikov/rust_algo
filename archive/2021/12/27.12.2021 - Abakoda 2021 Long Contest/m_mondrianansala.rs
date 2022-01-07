//{"name":"M. Mondrianansala","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/M","interactive":false,"timeLimit":2500,"tests":[{"input":"6\n","output":"6\nAAABBB\nAAABBB\nBBBAAA\nBBBAAA\nAAABBB\nAAABBB\n"},{"input":"24\n","output":"24\nAAAAAAAAAAAAAAABBBBBBBBB\nAAAAAAAAAAAAAAABBBBBBBBB\nAAAAAAAAAAAAAAABBBBBBBBB\nAAAAAAAAAAAAAAABBBBBBBBB\nAAAAAAAAAAAAAAABBBBBBBBB\nAAAAAAAAAAAAAAABBBBBBBBB\nAAAAAAAAAAAAAAACCCCCCDDD\nAAAAAAAAAAAAAAACCCCCCDDD\nAAAAAAAAAAAAAAACCCCCCBBB\nAAAAAAAAAAAAAAACCCCCCBBB\nDDDBBBBBBBBBBBBCCCCCCDDD\nDDDBBBBBBBBBBBBCCCCCCDDD\nCCCBBBBBBBBBBBBCCCCCCBBB\nCCCBBBBBBBBBBBBCCCCCCBBB\nDDDBBBBBBBBBBBBCCCCCCDDD\nDDDBBBBBBBBBBBBAADDAADDD\nCCCBBBBBBBBBBBBAADDAABBB\nCCCBBBBBBBBBBBBAADDAABBB\nAAAACCCDDDDDDDDDBBBBCCCC\nAAAACCCDDDDDDDDDBBBBCCCC\nAAAABBBDDDDDDDDDBBBBCCCC\nAAAABBBDDDDDDDDDBBBBCCCC\nAAAACCCDDDDDDDDDBBBBCCCC\nAAAACCCDDDDDDDDDBBBBCCCC\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MMondrianansala"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let m = input.read_usize();

    let mut side = 12;
    while side * 2 <= 2500 {
        side *= 2;
    }
    #[derive(Eq, PartialEq)]
    struct Rect {
        from_row: usize,
        to_row: usize,
        from_col: usize,
        to_col: usize,
    }

    impl Rect {
        fn area(&self) -> usize {
            (self.to_row - self.from_row) * (self.to_col - self.from_col)
        }

        fn dividable(&self) -> bool {
            (self.to_row - self.from_row) % 2 == 0 && (self.to_col - self.from_col) % 2 == 0
        }

        fn valid(&self) -> bool {
            (self.to_row - self.from_row) * 2 == (self.to_col - self.from_col) * 3
                || (self.to_row - self.from_row) * 3 == (self.to_col - self.from_col) * 2
        }
    }

    impl PartialOrd for Rect {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Rect {
        fn cmp(&self, other: &Self) -> Ordering {
            // let res = self.dividable().cmp(&other.dividable());
            // if res != Ordering::Equal {
            //     res
            // } else {
            self.area().cmp(&other.area())
            // }
        }
    }

    let mut heap = BinaryHeap::with_capacity(m);
    match m % 3 {
        0 => {
            heap.push(Rect {
                from_row: 0,
                to_row: side / 2,
                from_col: 0,
                to_col: side / 3,
            });
            heap.push(Rect {
                from_row: side / 2,
                to_row: side,
                from_col: 0,
                to_col: side / 3,
            });
            heap.push(Rect {
                from_row: 0,
                to_row: side / 2,
                from_col: side / 3,
                to_col: 2 * side / 3,
            });
            heap.push(Rect {
                from_row: side / 2,
                to_row: side,
                from_col: side / 3,
                to_col: 2 * side / 3,
            });
            heap.push(Rect {
                from_row: 0,
                to_row: side / 2,
                from_col: 2 * side / 3,
                to_col: side,
            });
            heap.push(Rect {
                from_row: side / 2,
                to_row: side,
                from_col: 2 * side / 3,
                to_col: side,
            });
        }
        1 => {
            heap.push(Rect {
                from_row: 0,
                to_row: side / 2,
                from_col: 0,
                to_col: side / 3,
            });
            heap.push(Rect {
                from_row: 0,
                to_row: side / 2,
                from_col: side / 3,
                to_col: 2 * side / 3,
            });
            heap.push(Rect {
                from_row: 0,
                to_row: side / 2,
                from_col: 2 * side / 3,
                to_col: side,
            });
            heap.push(Rect {
                from_row: side / 2,
                to_row: side,
                from_col: 0,
                to_col: 3 * side / 4,
            });
            heap.push(Rect {
                from_row: side / 2,
                to_row: side * 4 / 6,
                from_col: 3 * side / 4,
                to_col: side,
            });
            heap.push(Rect {
                from_row: side * 4 / 6,
                to_row: side * 5 / 6,
                from_col: 3 * side / 4,
                to_col: side,
            });
            heap.push(Rect {
                from_row: side * 5 / 6,
                to_row: side,
                from_col: 3 * side / 4,
                to_col: side,
            });
        }
        2 => {
            heap.push(Rect {
                from_row: 0,
                to_row: side / 2,
                from_col: 0,
                to_col: 3 * side / 4,
            });
            heap.push(Rect {
                from_row: side / 2,
                to_row: side,
                from_col: 0,
                to_col: 3 * side / 4,
            });
            heap.push(Rect {
                from_row: 0,
                to_row: side / 6,
                from_col: 3 * side / 4,
                to_col: side,
            });
            heap.push(Rect {
                from_row: side / 6,
                to_row: 2 * side / 6,
                from_col: 3 * side / 4,
                to_col: side,
            });
            heap.push(Rect {
                from_row: 2 * side / 6,
                to_row: 3 * side / 6,
                from_col: 3 * side / 4,
                to_col: side,
            });
            heap.push(Rect {
                from_row: 3 * side / 6,
                to_row: 4 * side / 6,
                from_col: 3 * side / 4,
                to_col: side,
            });
            heap.push(Rect {
                from_row: 4 * side / 6,
                to_row: 5 * side / 6,
                from_col: 3 * side / 4,
                to_col: side,
            });
            heap.push(Rect {
                from_row: 5 * side / 6,
                to_row: 6 * side / 6,
                from_col: 3 * side / 4,
                to_col: side,
            });
        }
        _ => unreachable!(),
    }

    while heap.len() < m {
        if !heap.peek().unwrap().dividable() {
            if heap.len() + 24 > m {
                break;
            }
            let mut tops = Vec::new();
            for _ in 0..3 {
                tops.push(heap.pop().unwrap());
            }
            let mut bad = false;
            for r in tops.iter() {
                if r.dividable() {
                    bad = true;
                    break;
                }
            }
            if bad {
                for r in tops {
                    heap.push(r);
                }
                break;
            }
            for r in tops {
                for i in 0..3 {
                    for j in 0..3 {
                        heap.push(Rect {
                            from_row: (r.from_row * (3 - i) + r.to_row * i) / 3,
                            to_row: (r.from_row * (2 - i) + r.to_row * (i + 1)) / 3,
                            from_col: (r.from_col * (3 - j) + r.to_col * j) / 3,
                            to_col: (r.from_col * (2 - j) + r.to_col * (j + 1)) / 3,
                        })
                    }
                }
            }
            continue;
        }

        let top = heap.pop().unwrap();

        assert!(top.valid());
        // if (top.to_row - top.from_row) % 2 != 0 {
        //     loop {}
        // }
        // if (top.to_col - top.from_col) % 2 != 0 {
        //     loop {}
        // }
        // assert!(top.dividable());
        heap.push(Rect {
            from_row: top.from_row,
            to_row: (top.from_row + top.to_row) / 2,
            from_col: top.from_col,
            to_col: (top.from_col + top.to_col) / 2,
        });
        heap.push(Rect {
            from_row: top.from_row,
            to_row: (top.from_row + top.to_row) / 2,
            from_col: (top.from_col + top.to_col) / 2,
            to_col: top.to_col,
        });
        heap.push(Rect {
            from_row: (top.from_row + top.to_row) / 2,
            to_row: top.to_row,
            from_col: top.from_col,
            to_col: (top.from_col + top.to_col) / 2,
        });
        heap.push(Rect {
            from_row: (top.from_row + top.to_row) / 2,
            to_row: top.to_row,
            from_col: (top.from_col + top.to_col) / 2,
            to_col: top.to_col,
        });
    }

    let mut delta = m - heap.len();
    assert_eq!(delta % 3, 0);

    let mut ans = Arr2d::new(side, side, '.');
    while let Some(top) = heap.pop() {
        assert!(top.valid());
        if delta > 0 && top.dividable() {
            heap.push(Rect {
                from_row: top.from_row,
                to_row: (top.from_row + top.to_row) / 2,
                from_col: top.from_col,
                to_col: (top.from_col + top.to_col) / 2,
            });
            heap.push(Rect {
                from_row: top.from_row,
                to_row: (top.from_row + top.to_row) / 2,
                from_col: (top.from_col + top.to_col) / 2,
                to_col: top.to_col,
            });
            heap.push(Rect {
                from_row: (top.from_row + top.to_row) / 2,
                to_row: top.to_row,
                from_col: top.from_col,
                to_col: (top.from_col + top.to_col) / 2,
            });
            heap.push(Rect {
                from_row: (top.from_row + top.to_row) / 2,
                to_row: top.to_row,
                from_col: (top.from_col + top.to_col) / 2,
                to_col: top.to_col,
            });
            delta -= 3;
            continue;
        }
        let mut used: i32 = 0;
        if top.from_row != 0 {
            for i in top.from_col..top.to_col {
                let c = ans[(top.from_row - 1, i)];
                if c != '.' {
                    let id = ((c as u8) - b'A').into_usize();
                    used.set_bit(id);
                }
            }
        }
        if top.to_row != side {
            for i in top.from_col..top.to_col {
                let c = ans[(top.to_row, i)];
                if c != '.' {
                    let id = ((c as u8) - b'A').into_usize();
                    used.set_bit(id);
                }
            }
        }
        if top.from_col != 0 {
            for i in top.from_row..top.to_row {
                let c = ans[(i, top.from_col - 1)];
                if c != '.' {
                    let id = ((c as u8) - b'A').into_usize();
                    used.set_bit(id);
                }
            }
        }
        if top.to_col != side {
            for i in top.from_row..top.to_row {
                let c = ans[(i, top.to_col)];
                if c != '.' {
                    let id = ((c as u8) - b'A').into_usize();
                    used.set_bit(id);
                }
            }
        }
        assert_ne!(used.count_ones(), 26);
        let c = {
            let mut res = '.';
            for i in 0..26 {
                if !used.is_set(i) {
                    res = (b'A' + i.into_u8()) as char;
                    break;
                }
            }
            assert_ne!(res, '.');
            res
        };
        for i in top.from_row..top.to_row {
            for j in top.from_col..top.to_col {
                ans[(i, j)] = c;
            }
        }
    }
    assert_eq!(delta, 0);
    out_line!(side);
    output().print_table(&ans);
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
