//{"name":"D1. Игра в Девятку I","group":"Codeforces - VK Cup 2022 - Квалификация (Engine)","url":"https://codeforces.com/contest/1769/problem/D1","interactive":false,"timeLimit":5000,"tests":[{"input":"JD 7S 9S JS 8S 9D 6D 8C 8D TH KS QD QH TD 6C AD KD AC\nKH QC 9H 6H KC 9C JC TS 6S QS TC JH 7D 7H AS AH 7C 8H\n","output":"Alice\n"},{"input":"7S KD 8C AH QS AC KS JC 6C 7D 9H TS 7C 6D JH JD 6S KC\n8D QD AS TD AD TH KH 9S JS 9C QC 8S 8H 7H TC QH 9D 6H\n","output":"Bob\n"},{"input":"6C 7S 6H KS 9C 6S QS 7C TS JD 8H KC 9D 8C 7H KD JC QC\n6D TH TD AD JS 9H TC QD 8D AC JH AH KH AS 7D 8S 9S QH\n","output":"Alice\n"},{"input":"JS KS JD TH KH JC KC QD AS JH 6H 9H 7H 6C 9D AC 6D 9S\n8D 8H 7C 7S KD 7D 6S QH 8C TS AD TD TC 9C QC 8S QS AH\n","output":"Bob\n"},{"input":"6S KC TD 8S AC 9S KD TS TH 7D 7C KH TC QC JH QD JC JD\nQH AS 9H 6C 8C 9C 6D AH AD KS JS 7H 6H 8H 9D QS 7S 8D\n","output":"Bob\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1IgraVDevyatkuI"}}}

use algo_lib::collections::arr5d::Arr5d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::cards::string_to_card;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn num_cards(hand: u64, state: &[u8; 4]) -> i32 {
    let mut res = 0;
    for (j, &i) in state.into_iter().enumerate() {
        if i == 0 {
            res += (hand >> (j * 9) & 511).count_ones() as i32;
        } else {
            let down = (i - 1) / 6;
            let up = (i - 1) % 6;
            res += (hand >> (j * 9) & u64::all_bits((3 - down) as usize)).count_ones() as i32;
            res += (hand >> (j * 9 + 4 + up.into_usize()) & u64::all_bits(5 - up as usize))
                .count_ones() as i32;
        }
    }
    res
}

fn score(alice: u64, bob: u64) -> i32 {
    let mut ans = Arr5d::new(2, 25, 25, 25, 25, None);
    let mut rec = RecursiveFunction2::new(|f, player: usize, state: [u8; 4]| -> i32 {
        match ans[(
            player,
            state[0].into_usize(),
            state[1].into_usize(),
            state[2].into_usize(),
            state[3].into_usize(),
        )] {
            Some(x) => x,
            None => {
                let we = if player == 0 { alice } else { bob };
                let res = if num_cards(if player == 0 { bob } else { alice }, &state) == 0 {
                    -num_cards(we, &state)
                } else {
                    let mut ans = i32::MIN;
                    let mut has_move = false;
                    for i in 0..4 {
                        let to_check = if state[i] == 0 {
                            vec![(3, 1)]
                        } else {
                            let mut res = Vec::new();
                            let down = (state[i] - 1) / 6;
                            let up = (state[i] - 1) % 6;
                            if down < 3 {
                                res.push((2 - down, 6));
                            }
                            if up < 5 {
                                res.push((4 + up, 1));
                            }
                            res
                        };
                        for (card, shift) in to_check {
                            if we.is_set(9 * i + card.into_usize()) {
                                has_move = true;
                                let mut new_state = state;
                                new_state[i] = new_state[i] as u8 + shift;
                                ans.maxim(-f.call(1 - player, new_state));
                            }
                        }
                    }
                    if !has_move {
                        ans = -f.call(1 - player, state);
                    }
                    ans
                };
                ans[(
                    player,
                    state[0].into_usize(),
                    state[1].into_usize(),
                    state[2].into_usize(),
                    state[3].into_usize(),
                )] = Some(res);
                res
            }
        }
    });
    rec.call(0, [0; 4])
}

fn solve(input: &mut Input, _test_case: usize) {
    let mut read_hand = || {
        let mut hand = 0u64;
        for _ in 0..18 {
            let (rank, suit) = string_to_card(input.read_string().as_str());
            hand.set_bit(suit * 9 + rank - 4);
        }
        hand
    };

    let alice = read_hand();
    let bob = read_hand();

    out_line!(if score(alice, bob) > 0 {
        "Alice"
    } else {
        "Bob"
    });
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
