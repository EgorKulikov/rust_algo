use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::string::Str;
use std::vec::IntoIter;

pub trait CommaList {
    fn read_list<T: Readable>(&mut self) -> Vec<T>;
}

impl CommaList for Input<'_> {
    fn read_list<T: Readable>(&mut self) -> Vec<T> {
        let mut s: String = self.read();
        s = s.replace(",", " ");
        let mut b = s.as_bytes();
        let input = Input::new(&mut b);
        input.into_iter().collect_vec()
    }
}

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let s: Str = inp.read();
    let mut bits = Vec::new();
    for c in s {
        let val = if (c as char).is_numeric() {
            c - b'0'
        } else {
            c - b'A' + 10
        };
        for i in (0usize..4).rev() {
            bits.push(val.is_set(i));
        }
    }

    let mut iter = bits.into_iter();

    fn read_number(iter: &mut IntoIter<bool>, len: usize) -> u64 {
        let mut res = 0;
        for _ in 0..len {
            res *= 2;
            if iter.next().unwrap() {
                res += 1;
            }
        }
        res
    }

    struct Packet {
        version: u64,
        content: PacketContent,
    }

    enum PacketContent {
        Operation(Operation, Vec<Packet>),
        Value(u128),
    }

    enum Operation {
        Sum,
        Product,
        Minimum,
        Maximum,
        GreaterThen,
        LessThen,
        EqualTo,
    }

    enum PacketLength {
        NumPackets(u64),
        NumBits(u64),
    }

    impl Packet {
        fn parse(iter: &mut IntoIter<bool>) -> (Self, u64) {
            let version = read_number(iter, 3);
            let (content, length) = PacketContent::parse(iter);
            (Self { version, content }, length + 3)
        }

        fn sum_ver(&self) -> u64 {
            self.version + self.content.sum_ver()
        }

        fn value(&self) -> u128 {
            self.content.value()
        }
    }

    impl PacketContent {
        fn parse(iter: &mut IntoIter<bool>) -> (Self, u64) {
            let type_id = read_number(iter, 3);
            if type_id == 4 {
                let mut content = 0;
                let mut len = 3;
                loop {
                    len += 5;
                    let continue_bit = iter.next().unwrap();
                    content *= 16;
                    content += read_number(iter, 4) as u128;
                    if !continue_bit {
                        break;
                    }
                }
                (PacketContent::Value(content), len)
            } else {
                let (mut packet_length, mut len) = PacketLength::parse(iter);
                len += 3;
                let mut content = Vec::new();
                while packet_length.remaining() != 0 {
                    let (packet, length) = Packet::parse(iter);
                    content.push(packet);
                    len += length;
                    packet_length.decrease(length);
                }
                (Self::Operation(type_id.into(), content), len)
            }
        }

        fn sum_ver(&self) -> u64 {
            match self {
                PacketContent::Operation(_, val) => val.iter().map(|p| p.sum_ver()).sum::<u64>(),
                PacketContent::Value(_) => 0,
            }
        }

        fn value(&self) -> u128 {
            match self {
                PacketContent::Operation(op, packets) => match op {
                    Operation::Sum => packets.iter().map(|p| p.value()).sum::<u128>(),
                    Operation::Product => packets.iter().map(|p| p.value()).product::<u128>(),
                    Operation::Minimum => packets.iter().map(|p| p.value()).min().unwrap(),
                    Operation::Maximum => packets.iter().map(|p| p.value()).max().unwrap(),
                    Operation::GreaterThen => {
                        assert_eq!(packets.len(), 2);
                        if packets[0].value() > packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::LessThen => {
                        assert_eq!(packets.len(), 2);
                        if packets[0].value() < packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::EqualTo => {
                        assert_eq!(packets.len(), 2);
                        if packets[0].value() == packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                },
                PacketContent::Value(val) => *val,
            }
        }
    }

    impl From<u64> for Operation {
        fn from(idx: u64) -> Self {
            match idx {
                0 => Self::Sum,
                1 => Self::Product,
                2 => Self::Minimum,
                3 => Self::Maximum,
                5 => Self::GreaterThen,
                6 => Self::LessThen,
                7 => Self::EqualTo,
                _ => panic!("unreachable"),
            }
        }
    }

    impl PacketLength {
        fn parse(iter: &mut IntoIter<bool>) -> (Self, u64) {
            if iter.next().unwrap() {
                (Self::NumPackets(read_number(iter, 11)), 12)
            } else {
                (Self::NumBits(read_number(iter, 15)), 16)
            }
        }

        fn remaining(&self) -> u64 {
            match self {
                PacketLength::NumPackets(len) => *len,
                PacketLength::NumBits(len) => *len,
            }
        }

        fn decrease(&mut self, length: u64) {
            match self {
                PacketLength::NumPackets(len) => *len -= 1,
                PacketLength::NumBits(len) => *len -= length,
            }
        }
    }

    let (outer, _) = Packet::parse(&mut iter);

    println!("{}", outer.sum_ver());
    println!("{}", outer.value());
}
