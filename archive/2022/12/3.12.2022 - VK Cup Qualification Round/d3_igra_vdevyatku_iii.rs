//{"name":"D3. Игра в Девятку III","group":"Codeforces - VK Cup 2022 - Квалификация (Engine)","url":"https://codeforces.com/contest/1769/problem/D3","interactive":false,"timeLimit":5000,"tests":[{"input":"2\n","output":"KS QD 8D QC 8S 8C JD 9H AC TH 9S 9D QH 7H 8H TS 7S 9C\n6D JS 7D KH QS TC AD AS KC 6C 7C TD AH KD 6S JC JH 6H\n\nJC JS 8S TD JD KH 7D 9C KC TH QD 8D 7H TC KD 9H 8C 6D\n7S AC QH AD 8H TS 6H JH 6C AH 7C 6S 9D QC AS QS KS 9S\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D3IgraVDevyatkuIII"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::cards::card_to_string;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

#[test]
fn test() {
    use algo_lib::collections::arr5d::Arr5d;
    use algo_lib::collections::min_max::MinimMaxim;
    use algo_lib::misc::random::random;
    use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
    use algo_lib::numbers::num_traits::primitive::Primitive;
    use std::io::{stdin, stdout, Write};

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

    fn total_score(hand: u64) -> i32 {
        let other = u64::all_bits(36) ^ hand;
        (score(hand, other) + score(other, hand)).abs()
    }

    let mut hand = 0;
    hand.set_bit(3);
    hand.set_bit(12);
    for i in 18..36 {
        if i != 21 && i != 30 {
            hand.set_bit(i);
        }
    }
    let mut hands = vec![hand];
    let random = random();
    let mut done = 1u64 << 30;
    while hands.len() < 27 {
        let index = random.next(hands.len().into_u64()).into_usize();
        let mut hand = hands[index];
        loop {
            let card = random.next(36).into_usize();
            if !hand.is_set(card) {
                hand.set_bit(card);
                break;
            }
        }
        loop {
            let card = random.next(36).into_usize();
            if hand.is_set(card) {
                hand.unset_bit(card);
                break;
            }
        }
        let score = total_score(hand);
        if !done.is_set(score.into_usize()) {
            done.set_bit(score.into_usize());
            hands.push(hand);
            println!("{} done, {} total", score, hands.len());
            println!("{:#?}", hands);
            stdout().flush().unwrap();
        } else {
            print!(".");
            stdout().flush().unwrap();
        }
    }
    println!("{:#?}", hands);
}

fn solve(input: &mut Input, _test_case: usize) {
    let ans = [
        67643379720,
        67106508936,
        59053445130,
        24693707274,
        67643379714,
        67642331176,
        58516574376,
        59045057034,
        67374944266,
        67373897768,
        33014159404,
        67642331148,
        67039401096,
        24676995594,
        59044008586,
        58986336282,
        66905216136,
        24675947146,
        59052527626,
        62811541656,
        24424225324,
        45631688856,
        67240759306,
        63348412456,
        24693183242,
        67641806892,
    ];
    let k = input.read_size();
    fn u64_to_hand(x: u64) -> Vec<String> {
        let mut res = Vec::with_capacity(18);
        for i in 0..36 {
            if x.is_set(i) {
                let rank = i % 9 + 4;
                let suit = i / 9;
                res.push(card_to_string(rank, suit));
            }
        }
        res
    }
    for i in ans.into_iter().take(k) {
        out_line!(u64_to_hand(i));
        out_line!(u64_to_hand(u64::all_bits(36) ^ i));
        out_line!();
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
