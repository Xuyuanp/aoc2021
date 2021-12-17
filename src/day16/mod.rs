use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: usize,
        val: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn parse(bits: &mut dyn Iterator<Item = Bit>, n: usize) -> Option<usize> {
        let mut res = 0;

        for _ in 0..n {
            res = res << 1 | (bits.next()? as usize);
        }

        Some(res)
    }

    fn decode_multi(bits: &mut dyn Iterator<Item = Bit>, nums: usize) -> Option<Vec<Self>> {
        (0..nums).map(|_| Self::decode_one(bits)).collect()
    }

    fn decode_all(bits: &mut dyn Iterator<Item = Bit>) -> Vec<Self> {
        let mut packets = Vec::new();

        while let Some(packet) = Self::decode_one(bits) {
            packets.push(packet);
        }

        packets
    }

    fn decode_one(bits: &mut dyn Iterator<Item = Bit>) -> Option<Self> {
        let version = Self::parse(bits, 3)?;
        Some(match Self::parse(bits, 3)? {
            4 => {
                let mut val = 0;
                loop {
                    let b1 = bits.next()?;
                    val = val << 4 | Self::parse(bits, 4)?;
                    if b1 == 0 {
                        break;
                    }
                }
                Self::Literal { version, val }
            }
            type_id => {
                let sub_packets = match bits.next()? {
                    0 => {
                        let total_length = Self::parse(bits, 15)?;
                        Self::decode_all(&mut bits.take(total_length))
                    }
                    1 => {
                        let nums = Self::parse(bits, 11)?;
                        Self::decode_multi(bits, nums)?
                    }
                    length_type_id => unreachable!("unknown length_type_id: {}", length_type_id),
                };
                Self::Operator {
                    version,
                    type_id,
                    sub_packets,
                }
            }
        })
    }

    fn get_version(&self) -> usize {
        match self {
            Self::Literal { version, .. } => *version,
            Self::Operator {
                version,
                sub_packets,
                ..
            } => version + sub_packets.iter().map(Self::get_version).sum::<usize>(),
        }
    }

    fn eval(&self) -> usize {
        match self {
            Self::Literal { val, .. } => *val,
            Self::Operator {
                type_id,
                sub_packets,
                ..
            } => match type_id {
                0 => sub_packets.iter().map(Self::eval).sum(),
                1 => sub_packets.iter().map(Self::eval).product(),
                2 => sub_packets.iter().map(Self::eval).min().unwrap(),
                3 => sub_packets.iter().map(Self::eval).max().unwrap(),
                5 => (sub_packets[0].eval() > sub_packets[1].eval()) as usize,
                6 => (sub_packets[0].eval() < sub_packets[1].eval()) as usize,
                7 => (sub_packets[0].eval() == sub_packets[1].eval()) as usize,
                _ => unreachable!("unknown type_id: {}", type_id),
            },
        }
    }
}

type Bit = u8;

// FIXME: does boolean type make it better?
fn get_bitsmap() -> HashMap<char, [Bit; 4]> {
    HashMap::from([
        ('0', [0, 0, 0, 0]),
        ('1', [0, 0, 0, 1]),
        ('2', [0, 0, 1, 0]),
        ('3', [0, 0, 1, 1]),
        ('4', [0, 1, 0, 0]),
        ('5', [0, 1, 0, 1]),
        ('6', [0, 1, 1, 0]),
        ('7', [0, 1, 1, 1]),
        ('8', [1, 0, 0, 0]),
        ('9', [1, 0, 0, 1]),
        ('A', [1, 0, 1, 0]),
        ('B', [1, 0, 1, 1]),
        ('C', [1, 1, 0, 0]),
        ('D', [1, 1, 0, 1]),
        ('E', [1, 1, 1, 0]),
        ('F', [1, 1, 1, 1]),
    ])
}

fn parse_input(input: &Vec<String>) -> Vec<Packet> {
    let bitsmap = get_bitsmap();
    let mut bits = input[0]
        .chars()
        .map(|c| bitsmap.get(&c).unwrap())
        .flatten()
        .map(|b| *b);

    Packet::decode_all(&mut bits)
}

pub fn part1(input: &Vec<String>) -> bool {
    let packets = parse_input(input);

    let res: usize = packets.iter().map(Packet::get_version).sum();

    res == 871
}

pub fn part2(input: &Vec<String>) -> bool {
    let packets = parse_input(input);
    assert_eq!(packets.len(), 1, "only one root packet expected");
    let res = packets[0].eval();

    res == 68703010504
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal() {
        let packet = &parse_input(&vec!["D2FE28".to_string()])[0];
        assert_eq!(
            *packet,
            Packet::Literal {
                version: 6,
                val: 2021
            },
            ""
        );
    }

    #[test]
    fn parse_operator_0() {
        let packet = &parse_input(&vec!["38006F45291200".to_string()])[0];
        assert_eq!(
            *packet,
            Packet::Operator {
                version: 1,
                type_id: 6,
                sub_packets: vec![
                    Packet::Literal {
                        version: 6,
                        val: 10
                    },
                    Packet::Literal {
                        version: 2,
                        val: 20,
                    }
                ]
            },
            ""
        );
    }

    #[test]
    fn parse_operator_1() {
        let packet = &parse_input(&vec!["EE00D40C823060".to_string()])[0];
        assert_eq!(
            *packet,
            Packet::Operator {
                version: 7,
                type_id: 3,
                sub_packets: vec![
                    Packet::Literal { version: 2, val: 1 },
                    Packet::Literal { version: 4, val: 2 },
                    Packet::Literal { version: 1, val: 3 }
                ]
            },
            ""
        );
    }

    #[test]
    fn more_example_0() {
        let packet = &parse_input(&vec!["8A004A801A8002F478".to_string()])[0];
        assert_eq!(
            *packet,
            Packet::Operator {
                version: 4,
                type_id: 2,
                sub_packets: vec![Packet::Operator {
                    version: 1,
                    type_id: 2,
                    sub_packets: vec![Packet::Operator {
                        version: 5,
                        type_id: 2,
                        sub_packets: vec![Packet::Literal {
                            version: 6,
                            val: 15,
                        }]
                    }]
                }]
            },
            ""
        );
        assert_eq!(packet.get_version(), 16, "");
    }

    #[test]
    fn more_example_1() {
        let packet = &parse_input(&vec!["620080001611562C8802118E34".to_string()])[0];
        assert_eq!(packet.get_version(), 12, "");
    }

    #[test]
    fn more_example_2() {
        let packet = &parse_input(&vec!["C0015000016115A2E0802F182340".to_string()])[0];
        assert_eq!(packet.get_version(), 23, "");
    }

    #[test]
    fn more_example_3() {
        let packet = &parse_input(&vec!["A0016C880162017C3686B18A3D4780".to_string()])[0];
        assert_eq!(packet.get_version(), 31, "");
    }
}
